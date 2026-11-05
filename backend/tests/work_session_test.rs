mod common;

use backend::entity::{user, work_session};
use chrono::{SubsecRound, Utc};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};

#[tokio::test]
async fn work_session_is_read_back_with_its_user() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let employee = common::insert_user(&db, user::Role::Employee, None, "Ada", "Lovelace").await;
    // Postgres keeps microseconds, chrono has nanoseconds: truncate so both sides compare equal.
    let start = Utc::now().trunc_subsecs(6).fixed_offset();

    let inserted = work_session::ActiveModel {
        user_id: Set(employee.id),
        start_time: Set(start),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let found = work_session::Entity::find_by_id(inserted.id)
        .one(&db)
        .await
        .unwrap()
        .expect("the inserted session should exist");
    assert_eq!(found.user_id, employee.id);
    assert_eq!(found.start_time, start);
    assert_eq!(found.end_time, None);

    let owner = found
        .find_related(user::Entity)
        .one(&db)
        .await
        .unwrap()
        .expect("the session should have a user");
    assert_eq!(owner.id, employee.id);
}
