use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Self (depth 0) and every descendant team. Loops forever if teams contain a cycle.
        db.execute_unprepared(
            "CREATE VIEW v_team_hierarchy AS
            WITH RECURSIVE hierarchy AS (
                SELECT id AS ancestor_team_id, id AS descendant_team_id, 0 AS depth
                FROM teams
                UNION ALL
                SELECT h.ancestor_team_id, child.id, h.depth + 1
                FROM hierarchy h
                JOIN teams child ON child.parent_team_id = h.descendant_team_id
            )
            SELECT ancestor_team_id, descendant_team_id, depth FROM hierarchy",
        )
        .await?;

        // Read scope only: includes self and manager peers, which are not writable.
        db.execute_unprepared(
            "CREATE VIEW v_manager_scope AS
            SELECT actor.id AS manager_id,
                   target.id AS target_user_id,
                   hierarchy.depth,
                   target.role AS target_role
            FROM users actor
            JOIN team_members actor_membership ON actor_membership.user_id = actor.id
            JOIN v_team_hierarchy hierarchy ON hierarchy.ancestor_team_id = actor_membership.team_id
            JOIN team_members target_membership ON target_membership.team_id = hierarchy.descendant_team_id
            JOIN users target ON target.id = target_membership.user_id
            WHERE actor.role = 'manager' AND actor.is_active AND target.is_active",
        )
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        db.execute_unprepared("DROP VIEW v_manager_scope").await?;
        db.execute_unprepared("DROP VIEW v_team_hierarchy").await?;

        Ok(())
    }
}
