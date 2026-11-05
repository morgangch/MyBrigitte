mod common;

use backend::entity::{audit_log, user};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};
use serde_json::json;

#[tokio::test]
async fn correction_is_read_back_with_its_author() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let manager = common::insert_user(&db, user::Role::Manager, None, "Mia", "Manager").await;
    let record_id = sea_orm::prelude::Uuid::new_v4();

    let inserted = audit_log::ActiveModel {
        table_name: Set("work_sessions".to_string()),
        record_id: Set(record_id),
        action: Set(audit_log::Action::Update),
        event_type: Set(Some("WORK_SESSION_CORRECTED".to_string())),
        old_data: Set(Some(json!({ "end_time": null }))),
        new_data: Set(Some(json!({ "end_time": "2026-01-05T18:00:00Z" }))),
        performed_by: Set(Some(manager.id)),
        reason: Set(Some("forgot to clock out".to_string())),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let found = audit_log::Entity::find_by_id(inserted.id)
        .one(&db)
        .await
        .unwrap()
        .expect("the inserted entry should exist");
    assert_eq!(found.table_name, "work_sessions");
    assert_eq!(found.record_id, record_id);
    assert_eq!(found.action, audit_log::Action::Update);
    assert_eq!(found.event_type.as_deref(), Some("WORK_SESSION_CORRECTED"));
    assert_eq!(found.old_data, Some(json!({ "end_time": null })));
    assert_eq!(
        found.new_data,
        Some(json!({ "end_time": "2026-01-05T18:00:00Z" }))
    );
    assert_eq!(found.reason.as_deref(), Some("forgot to clock out"));

    let author = found
        .find_related(user::Entity)
        .one(&db)
        .await
        .unwrap()
        .expect("the entry should have an author");
    assert_eq!(author.id, manager.id);

    let entries = manager
        .find_related(audit_log::Entity)
        .all(&db)
        .await
        .unwrap();
    assert_eq!(entries, vec![found]);
}

#[tokio::test]
async fn system_entry_has_no_author() {
    let Some(db) = common::test_db().await else {
        return;
    };

    let inserted = audit_log::ActiveModel {
        table_name: Set("users".to_string()),
        record_id: Set(sea_orm::prelude::Uuid::new_v4()),
        action: Set(audit_log::Action::Create),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    assert_eq!(inserted.performed_by, None);
    assert_eq!(inserted.old_data, None);
    let author = inserted.find_related(user::Entity).one(&db).await.unwrap();
    assert!(author.is_none());
}
