mod common;

use backend::entity::{user, user_work_policy};
use chrono::NaiveDate;
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};

#[tokio::test]
async fn work_policy_is_read_back_with_database_defaults() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let manager = common::insert_user(&db, user::Role::Manager, None, "Mia", "Manager").await;
    let employee = common::insert_user(&db, user::Role::Employee, None, "Ada", "Lovelace").await;
    let effective_from = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();

    let inserted = user_work_policy::ActiveModel {
        user_id: Set(employee.id),
        mode: Set(user_work_policy::Mode::Flexible),
        expected_minutes_per_week: Set(Some(2100)),
        effective_from: Set(effective_from),
        created_by: Set(manager.id),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let found = user_work_policy::Entity::find_by_id(inserted.id)
        .one(&db)
        .await
        .unwrap()
        .expect("the inserted policy should exist");
    assert_eq!(found.user_id, employee.id);
    assert_eq!(found.created_by, manager.id);
    assert_eq!(found.mode, user_work_policy::Mode::Flexible);
    assert_eq!(found.expected_minutes_per_week, Some(2100));
    assert_eq!(found.effective_from, effective_from);
    assert_eq!(found.effective_until, None);
    // Filled by the column defaults of the migration.
    assert_eq!(found.default_break_minutes, 0);
    assert_eq!(found.late_tolerance_minutes, 5);
}

#[tokio::test]
async fn work_policy_is_related_to_its_user_not_its_creator() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let manager = common::insert_user(&db, user::Role::Manager, None, "Mia", "Manager").await;
    let employee = common::insert_user(&db, user::Role::Employee, None, "Ada", "Lovelace").await;
    let policy = common::insert_work_policy(
        &db,
        employee.id,
        manager.id,
        NaiveDate::from_ymd_opt(2026, 1, 1).unwrap(),
    )
    .await;

    let owner = policy
        .find_related(user::Entity)
        .one(&db)
        .await
        .unwrap()
        .expect("the policy should have a user");
    assert_eq!(owner.id, employee.id);

    let employee_policies = employee
        .find_related(user_work_policy::Entity)
        .all(&db)
        .await
        .unwrap();
    assert_eq!(employee_policies.len(), 1);
    assert_eq!(employee_policies[0].id, policy.id);

    let manager_policies = manager
        .find_related(user_work_policy::Entity)
        .all(&db)
        .await
        .unwrap();
    assert!(manager_policies.is_empty());
}
