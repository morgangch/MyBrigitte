// TODO(clock-api) : fausses réponses de l'API, une constante par route, à remplacer par les vrais appels.
import type { DailySummary, Me, ScheduleDay, Summary, Team, User, WorkSession } from '../types/api.ts'

function _dayAt(daysAgo: number, hours: number, minutes: number) {
  const date = new Date()
  date.setDate(date.getDate() - daysAgo)
  date.setHours(hours, minutes, 0, 0)
  return date.toISOString()
}

function _dateOf(daysAgo: number) {
  return _dayAt(daysAgo, 12, 0).slice(0, 10)
}

// GET /teams
export const TEAMS: Team[] = [
  { id: 'b0000000-0000-4000-8000-000000000001', name: 'Service client' },
  { id: 'b0000000-0000-4000-8000-000000000002', name: 'Opérations' },
  { id: 'b0000000-0000-4000-8000-000000000003', name: 'Finance' },
  { id: 'b0000000-0000-4000-8000-000000000004', name: 'IT' },
  { id: 'b0000000-0000-4000-8000-000000000005', name: 'RH' },
]

// GET /me
export const ME: Me = {
  id: 'a0000000-0000-4000-8000-000000000001',
  first_name: 'Léa',
  last_name: 'Martin',
  email: 'lea.martin@mybrigitte.fr',
  phone_number: null,
  microsoft_email: 'lea.martin@mybrigitte.fr',
  has_password: false,
  role: 'employee',
  team: TEAMS[0],
}

// GET /me/work-sessions?from=…&to=…
export const MY_WORK_SESSIONS: WorkSession[] = [
  { id: 'c0000000-0000-4000-8000-000000000001', start_time: _dayAt(3, 9, 14), end_time: _dayAt(3, 12, 30) },
  { id: 'c0000000-0000-4000-8000-000000000002', start_time: _dayAt(3, 13, 30), end_time: _dayAt(3, 17, 25) },
  { id: 'c0000000-0000-4000-8000-000000000003', start_time: _dayAt(2, 8, 56), end_time: _dayAt(2, 12, 15) },
  { id: 'c0000000-0000-4000-8000-000000000004', start_time: _dayAt(2, 13, 10), end_time: _dayAt(2, 17, 5) },
  { id: 'c0000000-0000-4000-8000-000000000005', start_time: _dayAt(1, 8, 58), end_time: _dayAt(1, 12, 20) },
  { id: 'c0000000-0000-4000-8000-000000000006', start_time: _dayAt(1, 13, 15), end_time: _dayAt(1, 17, 10) },
  { id: 'c0000000-0000-4000-8000-000000000007', start_time: _dayAt(0, 8, 52), end_time: null },
]

// GET /me/schedule?from=…&to=…
export const MY_SCHEDULE: ScheduleDay[] = [
  { date: _dateOf(3), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420, override_type: null },
  { date: _dateOf(2), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420, override_type: null },
  { date: _dateOf(1), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420, override_type: null },
  { date: _dateOf(0), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420, override_type: null },
  { date: _dateOf(-1), start_time: null, end_time: null, break_minutes: null, expected_minutes: 0, override_type: 'rtt' },
]

// GET /me/summary?from=…&to=…
export const MY_SUMMARY: Summary = {
  from: _dateOf(3),
  to: _dateOf(-1),
  worked_minutes: 1302,
  expected_minutes: 1680,
  overtime_minutes: 42,
  worked_days: 3,
  late_days: 1,
  late_tolerance_minutes: 5,
}

// GET /me/summary?from=…&to=…&group_by=day
export const MY_DAILY_SUMMARY: DailySummary[] = [
  { date: _dateOf(3), expected_minutes: 420, worked_minutes: 431, overtime_minutes: 11, late_minutes: 14, override_type: null },
  { date: _dateOf(2), expected_minutes: 420, worked_minutes: 434, overtime_minutes: 14, late_minutes: 0, override_type: null },
  { date: _dateOf(1), expected_minutes: 420, worked_minutes: 437, overtime_minutes: 17, late_minutes: 0, override_type: null },
  { date: _dateOf(0), expected_minutes: 420, worked_minutes: 0, overtime_minutes: 0, late_minutes: 0, override_type: null },
  { date: _dateOf(-1), expected_minutes: 0, worked_minutes: 0, overtime_minutes: 0, late_minutes: 0, override_type: 'rtt' },
]

// GET /users
export const USERS: User[] = [
  { id: ME.id, first_name: ME.first_name, last_name: ME.last_name, email: ME.email, phone_number: ME.phone_number, role: ME.role, team: ME.team, today_status: 'present' },
  { id: 'a0000000-0000-4000-8000-000000000002', first_name: 'Hugo', last_name: 'Bernard', email: 'hugo.bernard@mybrigitte.fr', phone_number: '06 12 34 56 78', role: 'manager', team: TEAMS[0], today_status: 'present' },
  { id: 'a0000000-0000-4000-8000-000000000003', first_name: 'Inès', last_name: 'Moreau', email: 'ines.moreau@mybrigitte.fr', phone_number: '06 98 76 54 32', role: 'employee', team: TEAMS[0], today_status: 'late' },
  { id: 'a0000000-0000-4000-8000-000000000004', first_name: 'Karim', last_name: 'Haddad', email: 'karim.haddad@mybrigitte.fr', phone_number: '07 45 12 89 63', role: 'manager', team: TEAMS[1], today_status: 'present' },
  { id: 'a0000000-0000-4000-8000-000000000005', first_name: 'Chloé', last_name: 'Lefèvre', email: 'chloe.lefevre@mybrigitte.fr', phone_number: '06 22 41 07 15', role: 'employee', team: TEAMS[2], today_status: 'not_arrived' },
  { id: 'a0000000-0000-4000-8000-000000000006', first_name: 'Mathis', last_name: 'Dubois', email: 'mathis.dubois@mybrigitte.fr', phone_number: '06 31 77 20 48', role: 'employee', team: TEAMS[3], today_status: 'present' },
  { id: 'a0000000-0000-4000-8000-000000000007', first_name: 'Aïcha', last_name: 'Diallo', email: 'aicha.diallo@mybrigitte.fr', phone_number: '07 63 18 92 05', role: 'manager', team: TEAMS[4], today_status: 'present' },
  { id: 'a0000000-0000-4000-8000-000000000008', first_name: 'Lucas', last_name: 'Petit', email: 'lucas.petit@mybrigitte.fr', phone_number: null, role: 'employee', team: TEAMS[3], today_status: 'absent' },
  { id: 'a0000000-0000-4000-8000-000000000009', first_name: 'Emma', last_name: 'Garnier', email: 'emma.garnier@mybrigitte.fr', phone_number: '06 57 29 40 93', role: 'employee', team: null, today_status: 'present' },
]
