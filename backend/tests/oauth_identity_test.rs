mod common;

use backend::entity::{oauth_identity, user};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};

#[tokio::test]
async fn oauth_identity_is_read_back_with_its_user() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let employee = common::insert_user(&db, user::Role::Employee, None, "Ada", "Lovelace").await;
    let subject = common::unique("subject");

    let inserted = oauth_identity::ActiveModel {
        user_id: Set(employee.id),
        provider: Set("microsoft".to_string()),
        issuer: Set("https://login.microsoftonline.com/tenant/v2.0".to_string()),
        subject: Set(subject.clone()),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let found = oauth_identity::Entity::find_by_id(inserted.id)
        .one(&db)
        .await
        .unwrap()
        .expect("the inserted identity should exist");
    assert_eq!(found.user_id, employee.id);
    assert_eq!(found.provider, "microsoft");
    assert_eq!(found.subject, subject);

    let owner = found
        .find_related(user::Entity)
        .one(&db)
        .await
        .unwrap()
        .expect("the identity should have a user");
    assert_eq!(owner.id, employee.id);

    let identities = employee
        .find_related(oauth_identity::Entity)
        .all(&db)
        .await
        .unwrap();
    assert_eq!(identities.len(), 1);
}
