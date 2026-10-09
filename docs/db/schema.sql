-- Time Manager / Trinity — PostgreSQL schema proposal
-- Version: 2026-10-09
-- Convention: validity periods [valid_from, valid_until) (upper bound exclusive).
-- Important: this is a schema proposal, to be tested as a SeaORM migration.
-- The app owns authorization (actor/team scope), account lifecycle and audit writes.

BEGIN;
CREATE EXTENSION IF NOT EXISTS btree_gist;

-- 1. USERS — internal identity. Manager accounts are created by managers,
-- not through public self-registration. password_hash is NULL before activation
-- or for an account using Microsoft sign-in only.
CREATE TABLE users (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    first_name text NOT NULL,
    last_name text NOT NULL,
    email text NOT NULL,
    phone_number text,
    password_hash text,
    role text NOT NULL CHECK (role IN ('employee', 'manager')),
    is_active boolean NOT NULL DEFAULT true,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT users_email_not_blank CHECK (btrim(email) <> '')
);
CREATE UNIQUE INDEX users_email_lower_unique ON users (lower(email));

-- 2. OAUTH_IDENTITIES — links an Entra/OIDC identity to one internal user.
-- Do not identify or link users merely by matching email addresses.
CREATE TABLE oauth_identities (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    provider text NOT NULL CHECK (provider = 'microsoft'),
    issuer text NOT NULL,
    subject text NOT NULL,
    created_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT oauth_identity_unique UNIQUE (provider, issuer, subject),
    CONSTRAINT oauth_issuer_not_blank CHECK (btrim(issuer) <> ''),
    CONSTRAINT oauth_subject_not_blank CHECK (btrim(subject) <> '')
);
CREATE INDEX oauth_identities_user_id_idx ON oauth_identities(user_id);

-- 3. AUTH_SESSIONS — revocable refresh sessions, stored by hash (not plaintext).
CREATE TABLE auth_sessions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    refresh_token_hash text NOT NULL UNIQUE,
    created_at timestamptz NOT NULL DEFAULT now(),
    expires_at timestamptz NOT NULL,
    revoked_at timestamptz,
    CONSTRAINT auth_sessions_expiration CHECK (expires_at > created_at),
    CONSTRAINT auth_sessions_revocation CHECK (revoked_at IS NULL OR revoked_at >= created_at)
);
CREATE INDEX auth_sessions_user_id_idx ON auth_sessions(user_id);

-- 4. TEAMS — adjacency list; parent nullable for organizational roots.
-- Important: FK + CHECK only prevent direct self-parenting, not longer cycles.
CREATE TABLE teams (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    name text NOT NULL,
    description text,
    parent_team_id uuid REFERENCES teams(id) ON DELETE RESTRICT,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT teams_no_self_parent CHECK (parent_team_id IS NULL OR parent_team_id <> id),
    CONSTRAINT teams_name_not_blank CHECK (btrim(name) <> '')
);
CREATE INDEX teams_parent_team_idx ON teams(parent_team_id);

-- 5. TEAM_MEMBERS — PK(user_id) enforces 0 or 1 team per user.
-- Managers are inferred from users.role + membership; no manager_id on teams.
CREATE TABLE team_members (
    user_id uuid PRIMARY KEY REFERENCES users(id) ON DELETE RESTRICT,
    team_id uuid NOT NULL REFERENCES teams(id) ON DELETE RESTRICT,
    joined_at timestamptz NOT NULL DEFAULT now()
);
CREATE INDEX team_members_team_id_idx ON team_members(team_id);

-- 6. USER_WORK_POLICIES — versioned calculation rules.
-- Fixed: weekly schedules may define working hours; flexible: weekly target.
-- No two policies may apply to the same user on the same date.
CREATE TABLE user_work_policies (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    mode text NOT NULL CHECK (mode IN ('fixed', 'flexible')),
    expected_minutes_per_week integer CHECK (expected_minutes_per_week IS NULL OR expected_minutes_per_week >= 0),
    default_break_minutes smallint NOT NULL DEFAULT 0 CHECK (default_break_minutes >= 0),
    late_tolerance_minutes smallint NOT NULL DEFAULT 5 CHECK (late_tolerance_minutes >= 0),
    effective_from date NOT NULL,
    effective_until date,
    created_by uuid NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    created_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT policies_positive_period CHECK (effective_until IS NULL OR effective_until > effective_from),
    CONSTRAINT policies_no_overlap EXCLUDE USING gist (
        user_id WITH =,
        daterange(effective_from, effective_until, '[)') WITH &&
    )
);
CREATE INDEX user_work_policies_user_idx ON user_work_policies(user_id);

-- 7. WORK_SCHEDULES — ONE RULE = ONE EXPECTED DAY (not a session/slot).
-- An applicable row is selected for (policy, date) by most recently created row.
-- day_of_week follows ISO numbering (Monday=1 ... Sunday=7).
-- [valid_from, valid_until) limits applicability, not the creation timestamp.
-- Both times NULL explicitly mean "no work expected"; break=0 then.
-- Work session spans are separate from the single scheduled daily envelope.
CREATE TABLE work_schedules (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    policy_id uuid NOT NULL REFERENCES user_work_policies(id) ON DELETE RESTRICT,
    day_of_week smallint NOT NULL CHECK (day_of_week BETWEEN 1 AND 7),
    valid_from date NOT NULL,
    valid_until date,
    start_time time,
    end_time time,
    expected_break_minutes smallint NOT NULL DEFAULT 0 CHECK (expected_break_minutes >= 0),
    created_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT schedules_positive_period CHECK (valid_until IS NULL OR valid_until > valid_from),
    CONSTRAINT schedules_times_both_or_neither CHECK (
        (start_time IS NULL AND end_time IS NULL AND expected_break_minutes = 0)
        OR (start_time IS NOT NULL AND end_time IS NOT NULL AND start_time <> end_time)
    ),
    -- Day envelopes are <24h: an end earlier than a start means next calendar day.
    CONSTRAINT schedules_break_shorter_than_envelope CHECK (
        start_time IS NULL OR expected_break_minutes <
        (CASE WHEN end_time > start_time
              THEN EXTRACT(EPOCH FROM (end_time - start_time)) / 60
              ELSE (86400 + EXTRACT(EPOCH FROM (end_time - start_time))) / 60
         END)
    )
);
CREATE INDEX work_schedules_lookup_idx
    ON work_schedules(policy_id, day_of_week, valid_from, created_at DESC);

-- 8. WORK_SESSIONS — actual continuous work periods, zero/one/many per day.
-- Closed session must end after start. Exclusion prevents overlapping sessions
-- for one employee, including overlap against an open-ended session.
CREATE TABLE work_sessions (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id uuid NOT NULL REFERENCES users(id) ON DELETE RESTRICT,
    start_time timestamptz NOT NULL,
    end_time timestamptz,
    created_at timestamptz NOT NULL DEFAULT now(),
    updated_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT work_sessions_valid_interval CHECK (end_time IS NULL OR end_time > start_time),
    CONSTRAINT work_sessions_no_overlap EXCLUDE USING gist (
        user_id WITH =,
        tstzrange(start_time, end_time, '[)') WITH &&
    )
);
CREATE UNIQUE INDEX work_sessions_one_open_per_user
    ON work_sessions(user_id) WHERE end_time IS NULL;
CREATE INDEX work_sessions_user_start_idx ON work_sessions(user_id, start_time DESC);

-- 9. AUDIT_LOG — generic append-only history (including manager corrections).
-- Enforce reason for explicit correction events. Log old/new work session values
-- in the same transaction as the update. DB permissions should prohibit UPDATE
-- or DELETE of audit rows by the runtime role.
CREATE TABLE audit_log (
    id uuid PRIMARY KEY DEFAULT gen_random_uuid(),
    table_name text NOT NULL,
    record_id uuid NOT NULL,
    action text NOT NULL CHECK (action IN ('CREATE', 'UPDATE', 'DELETE')),
    event_type text,
    old_data jsonb,
    new_data jsonb,
    performed_by uuid REFERENCES users(id) ON DELETE RESTRICT,
    reason text,
    created_at timestamptz NOT NULL DEFAULT now(),
    CONSTRAINT audit_log_correction_reason CHECK (
        event_type IS DISTINCT FROM 'WORK_SESSION_CORRECTED'
        OR (reason IS NOT NULL AND btrim(reason) <> '')
    )
);
CREATE INDEX audit_log_record_idx ON audit_log(table_name, record_id, created_at DESC);
CREATE INDEX audit_log_actor_idx ON audit_log(performed_by, created_at DESC);

COMMIT;

-- NOT EXPRESSIBLE BY THE SIMPLE CONSTRAINTS ABOVE (app transaction or DB triggers):
--  * Prevent indirect cycles in teams, including concurrent reparent operations.
--  * Consistency of schedules.valid_* within their linked policy effective_*.
--  * Decide & enforce stable ordering when schedules tie on created_at:
--    a UUID tie-breaker is deterministic, NOT a true creation sequence.
--  * Preserve already-accounted historical expected hours: most-recent-wins
--    backdated schedules can reclassify earlier dates unless forbidden.
--  * Disallow incompatible schedules if your business rules require it.
--  * Validate actor has correction privilege and atomically write audit entries.
--  * Set updated_at on writes (app or trigger); DEFAULT is not an auto-update.
--  * Use company timezone when resolving day_of_week and daily aggregates.
--  * Policy for account deletion/anonymization and historical retention.
