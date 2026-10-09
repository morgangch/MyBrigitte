-- Time Manager / Trinity — views for authorization scope and KPIs
-- These are ordinary views (live computation), NOT materialized views.

-- v_team_hierarchy: self (depth 0) and every descendant team.
-- WARNING: requires cycle prevention for teams.parent_team_id.
CREATE VIEW v_team_hierarchy AS
WITH RECURSIVE hierarchy AS (
    SELECT id AS ancestor_team_id, id AS descendant_team_id, 0 AS depth
    FROM teams
    UNION ALL
    SELECT h.ancestor_team_id, child.id, h.depth + 1
    FROM hierarchy h
    JOIN teams child ON child.parent_team_id = h.descendant_team_id
)
SELECT ancestor_team_id, descendant_team_id, depth FROM hierarchy;

-- v_manager_scope: team members within manager's subtree, including self
-- and manager peers. The VIEW grants NO write permissions by itself.
CREATE VIEW v_manager_scope AS
SELECT actor.id AS manager_id,
       target.id AS target_user_id,
       hierarchy.depth,
       target.role AS target_role
FROM users actor
JOIN team_members actor_membership ON actor_membership.user_id = actor.id
JOIN v_team_hierarchy hierarchy ON hierarchy.ancestor_team_id = actor_membership.team_id
JOIN team_members target_membership ON target_membership.team_id = hierarchy.descendant_team_id
JOIN users target ON target.id = target_membership.user_id
WHERE actor.role = 'manager' AND actor.is_active AND target.is_active;

-- For correction write access, Axum must enforce IN ADDITION:
--  * if actor==target: actor must belong to root team;
--  * if same team: target must be employee (peers are not writable);
--  * if descendant team depth>0: target is writable;
--  * role and team membership must be checked in the same transaction.
-- Do not cache v_manager_scope as a materialized view for authorization.

-- Suggested further analytics views/queries (NOT defined here):
-- v_work_session_daily — actual minutes per local working day; accurately
--   partition any session crossing local midnight before aggregation.
-- v_attendance_daily — expected vs. actual, lateness / no-show; generate
--   days from the requested date range rather than only observed sessions.
-- For a manager-selected date range, use parameterized SQL in Axum (or a
-- SQL function accepting from/to) to avoid an unbounded calendar view.
