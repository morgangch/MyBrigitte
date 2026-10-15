// TODO(clock-api) : fausses réponses de l'API, une constante par route, à remplacer par les vrais appels.
import type {
  DayStats,
  Me,
  Page,
  Presence,
  ScheduleDay,
  Stats,
  Team,
  TeamRef,
  TeamStats,
  User,
  UserRef,
  WorkPolicy,
  WorkSession,
} from '../types/api.ts'

function _dayAt(daysAgo: number, hours: number, minutes: number) {
  const date = new Date()
  date.setDate(date.getDate() - daysAgo)
  date.setHours(hours, minutes, 0, 0)
  return date.toISOString()
}

function _dateOf(daysAgo: number) {
  return _dayAt(daysAgo, 12, 0).slice(0, 10)
}

function _dayStats(expectedMinutes: number, workedMinutes: number, lateMinutes: number): Stats {
  const expectedDays = expectedMinutes > 0 ? 1 : 0
  const workedDays = workedMinutes > 0 ? 1 : 0
  const lateDays = lateMinutes > 0 ? 1 : 0
  return {
    worked_minutes: workedMinutes,
    expected_minutes: expectedMinutes,
    overtime_minutes: Math.max(0, workedMinutes - expectedMinutes),
    late_minutes: lateMinutes,
    expected_days: expectedDays,
    worked_days: workedDays,
    late_days: lateDays,
    absent_days: 0,
    average_weekly_minutes: null,
    punctuality_rate: workedDays > 0 ? ((workedDays - lateDays) / workedDays) * 100 : null,
    presence_rate: expectedDays > 0 ? (workedDays / expectedDays) * 100 : null,
  }
}

function _teamStats(memberCount: number, averageWeeklyMinutes: number, punctualityRate: number): Stats {
  const expectedDays = memberCount * 5
  const lateDays = Math.round(expectedDays * (1 - punctualityRate / 100))
  const expectedMinutes = memberCount * 2100
  const workedMinutes = memberCount * averageWeeklyMinutes
  return {
    worked_minutes: workedMinutes,
    expected_minutes: expectedMinutes,
    overtime_minutes: Math.max(0, workedMinutes - expectedMinutes),
    late_minutes: lateDays * 12,
    expected_days: expectedDays,
    worked_days: expectedDays,
    late_days: lateDays,
    absent_days: 0,
    average_weekly_minutes: averageWeeklyMinutes,
    punctuality_rate: punctualityRate,
    presence_rate: 100,
  }
}

const _TEAM_REFS = {
  direction: { id: 'b0000000-0000-4000-8000-000000000006', name: 'Direction' },
  serviceClient: { id: 'b0000000-0000-4000-8000-000000000001', name: 'Service client' },
  operations: { id: 'b0000000-0000-4000-8000-000000000002', name: 'Opérations' },
  finance: { id: 'b0000000-0000-4000-8000-000000000003', name: 'Finance' },
  it: { id: 'b0000000-0000-4000-8000-000000000004', name: 'IT' },
  rh: { id: 'b0000000-0000-4000-8000-000000000005', name: 'RH' },
} satisfies Record<string, TeamRef>

// GET /users
export const USERS: Page<User> = {
  total: 9,
  items: [
    { id: 'a0000000-0000-4000-8000-000000000001', first_name: 'Léa', last_name: 'Martin', email: 'lea.martin@mybrigitte.fr', phone_number: null, role: 'manager', team: _TEAM_REFS.direction, is_active: true, created_at: '2023-03-06T08:00:00.000Z' },
    { id: 'a0000000-0000-4000-8000-000000000002', first_name: 'Hugo', last_name: 'Bernard', email: 'hugo.bernard@mybrigitte.fr', phone_number: '06 12 34 56 78', role: 'manager', team: _TEAM_REFS.serviceClient, is_active: true, created_at: '2023-09-04T08:00:00.000Z' },
    { id: 'a0000000-0000-4000-8000-000000000003', first_name: 'Inès', last_name: 'Moreau', email: 'ines.moreau@mybrigitte.fr', phone_number: '06 98 76 54 32', role: 'employee', team: _TEAM_REFS.serviceClient, is_active: true, created_at: '2024-01-15T08:00:00.000Z' },
    { id: 'a0000000-0000-4000-8000-000000000004', first_name: 'Karim', last_name: 'Haddad', email: 'karim.haddad@mybrigitte.fr', phone_number: '07 45 12 89 63', role: 'manager', team: _TEAM_REFS.operations, is_active: true, created_at: '2023-05-02T08:00:00.000Z' },
    { id: 'a0000000-0000-4000-8000-000000000005', first_name: 'Chloé', last_name: 'Lefèvre', email: 'chloe.lefevre@mybrigitte.fr', phone_number: '06 22 41 07 15', role: 'employee', team: _TEAM_REFS.finance, is_active: true, created_at: '2024-06-03T08:00:00.000Z' },
    { id: 'a0000000-0000-4000-8000-000000000006', first_name: 'Mathis', last_name: 'Dubois', email: 'mathis.dubois@mybrigitte.fr', phone_number: '06 31 77 20 48', role: 'employee', team: _TEAM_REFS.it, is_active: true, created_at: '2025-02-10T08:00:00.000Z' },
    { id: 'a0000000-0000-4000-8000-000000000007', first_name: 'Aïcha', last_name: 'Diallo', email: 'aicha.diallo@mybrigitte.fr', phone_number: '07 63 18 92 05', role: 'manager', team: _TEAM_REFS.rh, is_active: true, created_at: '2023-03-20T08:00:00.000Z' },
    { id: 'a0000000-0000-4000-8000-000000000008', first_name: 'Lucas', last_name: 'Petit', email: 'lucas.petit@mybrigitte.fr', phone_number: null, role: 'employee', team: _TEAM_REFS.it, is_active: true, created_at: '2025-09-01T08:00:00.000Z' },
    { id: 'a0000000-0000-4000-8000-000000000009', first_name: 'Emma', last_name: 'Garnier', email: 'emma.garnier@mybrigitte.fr', phone_number: '06 57 29 40 93', role: 'employee', team: _TEAM_REFS.rh, is_active: true, created_at: '2026-01-05T08:00:00.000Z' },
  ],
}

function _userRef(index: number): UserRef {
  const user = USERS.items[index]
  return { id: user.id, first_name: user.first_name, last_name: user.last_name }
}

// GET /me
export const ME: Me = {
  ...USERS.items[0],
  has_password: false,
}

// GET /me/work-policy
export const MY_WORK_POLICY: WorkPolicy = {
  id: 'f0000000-0000-4000-8000-000000000001',
  mode: 'fixed',
  expected_minutes_per_week: null,
  default_break_minutes: 60,
  late_tolerance_minutes: 5,
  effective_from: '2023-03-06',
  effective_until: null,
  weekly_schedule: [1, 2, 3, 4, 5].map((dayOfWeek) => ({
    day_of_week: dayOfWeek,
    start_time: '09:00:00',
    end_time: '17:00:00',
    expected_break_minutes: 60,
  })),
}

// GET /me/work-sessions?from=…&to=…
export const MY_WORK_SESSIONS: Page<WorkSession> = {
  total: 7,
  items: [
    { id: 'c0000000-0000-4000-8000-000000000001', start_time: _dayAt(3, 9, 14), end_time: _dayAt(3, 12, 30) },
    { id: 'c0000000-0000-4000-8000-000000000002', start_time: _dayAt(3, 13, 30), end_time: _dayAt(3, 17, 25) },
    { id: 'c0000000-0000-4000-8000-000000000003', start_time: _dayAt(2, 8, 56), end_time: _dayAt(2, 12, 15) },
    { id: 'c0000000-0000-4000-8000-000000000004', start_time: _dayAt(2, 13, 10), end_time: _dayAt(2, 17, 5) },
    { id: 'c0000000-0000-4000-8000-000000000005', start_time: _dayAt(1, 8, 58), end_time: _dayAt(1, 12, 20) },
    { id: 'c0000000-0000-4000-8000-000000000006', start_time: _dayAt(1, 13, 15), end_time: _dayAt(1, 17, 10) },
    { id: 'c0000000-0000-4000-8000-000000000007', start_time: _dayAt(0, 8, 52), end_time: null },
  ],
}

// GET /me/schedule?from=…&to=…
export const MY_SCHEDULE: ScheduleDay[] = [
  { date: _dateOf(3), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420 },
  { date: _dateOf(2), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420 },
  { date: _dateOf(1), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420 },
  { date: _dateOf(0), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420 },
  { date: _dateOf(-1), start_time: null, end_time: null, break_minutes: 0, expected_minutes: 0 },
]

// GET /me/stats?from=…&to=…
export const MY_WEEK_STATS: Stats = {
  worked_minutes: 1302,
  expected_minutes: 1680,
  overtime_minutes: 42,
  late_minutes: 14,
  expected_days: 4,
  worked_days: 3,
  late_days: 1,
  absent_days: 0,
  average_weekly_minutes: 1302,
  punctuality_rate: 66.7,
  presence_rate: 75,
}

// GET /me/stats?from=…&to=…&group_by=day
export const MY_DAILY_STATS: DayStats[] = [
  { date: _dateOf(3), stats: _dayStats(420, 431, 14) },
  { date: _dateOf(2), stats: _dayStats(420, 434, 0) },
  { date: _dateOf(1), stats: _dayStats(420, 437, 0) },
  { date: _dateOf(0), stats: _dayStats(420, 0, 0) },
  { date: _dateOf(-1), stats: _dayStats(0, 0, 0) },
]

// GET /presence
export const PRESENCE: Presence[] = [
  { user: _userRef(0), team: _TEAM_REFS.direction, status: 'present', first_clock_in: _dayAt(0, 8, 52) },
  { user: _userRef(1), team: _TEAM_REFS.serviceClient, status: 'present', first_clock_in: _dayAt(0, 8, 47) },
  { user: _userRef(2), team: _TEAM_REFS.serviceClient, status: 'late', first_clock_in: _dayAt(0, 9, 21) },
  { user: _userRef(3), team: _TEAM_REFS.operations, status: 'present', first_clock_in: _dayAt(0, 7, 58) },
  { user: _userRef(4), team: _TEAM_REFS.finance, status: 'not_arrived', first_clock_in: null },
  { user: _userRef(5), team: _TEAM_REFS.it, status: 'present', first_clock_in: _dayAt(0, 9, 2) },
  { user: _userRef(6), team: _TEAM_REFS.rh, status: 'away', first_clock_in: _dayAt(0, 8, 30) },
  { user: _userRef(7), team: _TEAM_REFS.it, status: 'absent', first_clock_in: null },
  { user: _userRef(8), team: _TEAM_REFS.rh, status: 'day_off', first_clock_in: null },
]

// GET /teams
export const TEAMS: Page<Team> = {
  total: 6,
  items: [
    {
      ..._TEAM_REFS.direction,
      description: 'Pilotage de Trinity Market',
      parent: null,
      managers: [_userRef(0)],
      member_count: 1,
      member_preview: [_userRef(0)],
    },
    {
      ..._TEAM_REFS.serviceClient,
      description: 'Support et relation clients',
      parent: _TEAM_REFS.direction,
      managers: [_userRef(1)],
      member_count: 2,
      member_preview: [_userRef(1), _userRef(2)],
    },
    {
      ..._TEAM_REFS.operations,
      description: 'Logistique et entrepôts',
      parent: _TEAM_REFS.direction,
      managers: [_userRef(3)],
      member_count: 1,
      member_preview: [_userRef(3)],
    },
    {
      ..._TEAM_REFS.finance,
      description: 'Comptabilité et paie',
      parent: _TEAM_REFS.direction,
      managers: [],
      member_count: 1,
      member_preview: [_userRef(4)],
    },
    {
      ..._TEAM_REFS.it,
      description: 'Plateforme e-commerce et outils internes',
      parent: _TEAM_REFS.direction,
      managers: [],
      member_count: 2,
      member_preview: [_userRef(5), _userRef(7)],
    },
    {
      ..._TEAM_REFS.rh,
      description: null,
      parent: _TEAM_REFS.direction,
      managers: [_userRef(6)],
      member_count: 2,
      member_preview: [_userRef(6), _userRef(8)],
    },
  ],
}

// GET /stats?from=…&to=…&group_by=team
export const TEAM_STATS: TeamStats[] = [
  { team: _TEAM_REFS.direction, stats: _teamStats(1, 2130, 96) },
  { team: _TEAM_REFS.serviceClient, stats: _teamStats(2, 2118, 94) },
  { team: _TEAM_REFS.operations, stats: _teamStats(1, 2165, 91) },
  { team: _TEAM_REFS.finance, stats: _teamStats(1, 2080, 97) },
  { team: _TEAM_REFS.it, stats: _teamStats(2, 2232, 88) },
  { team: _TEAM_REFS.rh, stats: _teamStats(2, 2100, 96) },
]
