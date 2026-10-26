import { ME, USERS, TEAMS, MY_WORK_SESSIONS, MY_SCHEDULE, MY_WORK_POLICY } from '../../web/src/lib/fakeData.ts'

const quote = (value: string | null) => value === null ? 'NULL' : `'${value.replaceAll("'", "''")}'`
const id = (value: string) => `md5(${quote(`mybrigitte-dev:${value}`)})::uuid`
const insert = (table: string, columns: string[], rows: string[][]) =>
  `INSERT INTO ${table} (${columns.join(', ')}) VALUES\n${rows.map(row => `(${row.join(', ')})`).join(',\n')}\nON CONFLICT DO NOTHING;`
const users = [ME, ...USERS.items]
const manager = USERS.items.find(user => user.role === 'manager')!
const dates = MY_SCHEDULE.map(day => day.date).sort()
const end = new Date(`${dates.at(-1)}T12:00:00Z`)
end.setUTCDate(end.getUTCDate() + 1)
const policy = id('policy-lea')

console.log(`-- Generated from web/src/lib/fakeData.ts at seed execution, TZ Europe/Paris.
BEGIN;
DO $$ BEGIN
  IF right(current_database(), 4) <> '_dev' THEN
    RAISE EXCEPTION 'Development seed requires a database name ending in _dev';
  END IF;
END $$;
SELECT pg_advisory_xact_lock(78432701);
`)
console.log(insert('teams', ['id', 'name'], TEAMS.items.map(team => [id(team.id), quote(team.name)])))
console.log(insert('users', ['id', 'first_name', 'last_name', 'email', 'phone_number', 'role'], users.map(user => [id(user.id), quote(user.first_name), quote(user.last_name), quote(user.email), quote(user.phone_number), quote(user.role)])))
console.log(insert('team_members', ['user_id', 'team_id'], users.filter(user => user.team).map(user => [id(user.id), id(user.team!.id)])))
console.log(insert('user_work_policies', ['id', 'user_id', 'mode', 'default_break_minutes', 'late_tolerance_minutes', 'effective_from', 'effective_until', 'created_by'], [[policy, id(ME.id), quote('fixed'), '60', String(MY_WORK_POLICY.late_tolerance_minutes), quote(dates[0]), quote(end.toISOString().slice(0, 10)), id(manager.id)]]))
console.log(insert('work_schedules', ['id', 'policy_id', 'day_of_week', 'valid_from', 'valid_until', 'start_time', 'end_time', 'expected_break_minutes'], MY_SCHEDULE.map((day, index) => {
  const date = new Date(`${day.date}T12:00:00Z`)
  const weekday = date.getUTCDay() || 7
  date.setUTCDate(date.getUTCDate() + 1)
  return [id(`schedule-${index + 1}`), policy, String(weekday), quote(day.date), quote(date.toISOString().slice(0, 10)), quote(day.start_time), quote(day.end_time), String(day.break_minutes ?? 0)]
})))
console.log(insert('work_sessions', ['id', 'user_id', 'start_time', 'end_time'], MY_WORK_SESSIONS.items.map(session => [id(session.id), id(ME.id), quote(session.start_time), quote(session.end_time)])))
console.log('COMMIT;')
