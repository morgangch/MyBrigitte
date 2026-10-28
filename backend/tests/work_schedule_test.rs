mod common;

use backend::entity::{user, user_work_policy, work_schedule};
use chrono::{NaiveDate, NaiveTime};
use sea_orm::{ActiveModelTrait, EntityTrait, ModelTrait, Set};

#[tokio::test]
async fn overnight_schedule_is_read_back_with_its_policy() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let employee = common::insert_user(&db, user::Role::Employee, None, "Ada", "Lovelace").await;
    let valid_from = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let policy = common::insert_work_policy(&db, employee.id, employee.id, valid_from).await;
    // Ends earlier than it starts: the shift finishes the next day.
    let start = NaiveTime::from_hms_opt(22, 0, 0).unwrap();
    let end = NaiveTime::from_hms_opt(6, 0, 0).unwrap();

    let inserted = work_schedule::ActiveModel {
        policy_id: Set(policy.id),
        day_of_week: Set(1),
        valid_from: Set(valid_from),
        start_time: Set(Some(start)),
        end_time: Set(Some(end)),
        expected_break_minutes: Set(30),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    let found = work_schedule::Entity::find_by_id(inserted.id)
        .one(&db)
        .await
        .unwrap()
        .expect("the inserted schedule should exist");
    assert_eq!(found.policy_id, policy.id);
    assert_eq!(found.day_of_week, 1);
    assert_eq!(found.valid_from, valid_from);
    assert_eq!(found.valid_until, None);
    assert_eq!(found.start_time, Some(start));
    assert_eq!(found.end_time, Some(end));
    assert_eq!(found.expected_break_minutes, 30);

    let owner = found
        .find_related(user_work_policy::Entity)
        .one(&db)
        .await
        .unwrap()
        .expect("the schedule should have a policy");
    assert_eq!(owner.id, policy.id);
}

#[tokio::test]
async fn day_off_is_read_back_without_times() {
    let Some(db) = common::test_db().await else {
        return;
    };
    let employee = common::insert_user(&db, user::Role::Employee, None, "Ada", "Lovelace").await;
    let valid_from = NaiveDate::from_ymd_opt(2026, 1, 1).unwrap();
    let policy = common::insert_work_policy(&db, employee.id, employee.id, valid_from).await;

    let inserted = work_schedule::ActiveModel {
        policy_id: Set(policy.id),
        day_of_week: Set(7),
        valid_from: Set(valid_from),
        ..Default::default()
    }
    .insert(&db)
    .await
    .unwrap();

    assert_eq!(inserted.start_time, None);
    assert_eq!(inserted.end_time, None);
    assert_eq!(inserted.expected_break_minutes, 0);

    let schedules = policy
        .find_related(work_schedule::Entity)
        .all(&db)
        .await
        .unwrap();
    assert_eq!(schedules, vec![inserted]);
}
