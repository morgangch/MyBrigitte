// TODO(clock-api) : fausses réponses de l'API, une constante par route, à remplacer par les vrais appels.

function _dayAt(daysAgo, hours, minutes) {
  const date = new Date()
  date.setDate(date.getDate() - daysAgo)
  date.setHours(hours, minutes, 0, 0)
  return date.toISOString()
}

function _dateOf(daysAgo) {
  return _dayAt(daysAgo, 12, 0).slice(0, 10)
}

// GET /me
export const ME = {
  id: 'user-1',
  first_name: 'Léa',
  last_name: 'Martin',
  email: 'lea.martin@mybrigitte.fr',
  phone_number: null,
  role: 'employee',
  team: { id: 'team-1', name: 'Service client' },
}

// GET /me/work-sessions?from=…&to=…
export const MY_WORK_SESSIONS = [
  { id: 'session-1', start_time: _dayAt(3, 9, 14), end_time: _dayAt(3, 12, 30) },
  { id: 'session-2', start_time: _dayAt(3, 13, 30), end_time: _dayAt(3, 17, 25) },
  { id: 'session-3', start_time: _dayAt(2, 8, 56), end_time: _dayAt(2, 12, 15) },
  { id: 'session-4', start_time: _dayAt(2, 13, 10), end_time: _dayAt(2, 17, 5) },
  { id: 'session-5', start_time: _dayAt(1, 8, 58), end_time: _dayAt(1, 12, 20) },
  { id: 'session-6', start_time: _dayAt(1, 13, 15), end_time: _dayAt(1, 17, 10) },
  { id: 'session-7', start_time: _dayAt(0, 8, 52), end_time: null },
]

// GET /me/schedule?from=…&to=…
export const MY_SCHEDULE = [
  { date: _dateOf(3), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420, override_type: null },
  { date: _dateOf(2), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420, override_type: null },
  { date: _dateOf(1), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420, override_type: null },
  { date: _dateOf(0), start_time: '09:00:00', end_time: '17:00:00', break_minutes: 60, expected_minutes: 420, override_type: null },
  { date: _dateOf(-1), start_time: null, end_time: null, break_minutes: null, expected_minutes: 0, override_type: 'rtt' },
]

// GET /me/summary?from=…&to=…
export const MY_SUMMARY = {
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
export const MY_DAILY_SUMMARY = [
  { date: _dateOf(3), expected_minutes: 420, worked_minutes: 431, overtime_minutes: 11, late_minutes: 14, override_type: null },
  { date: _dateOf(2), expected_minutes: 420, worked_minutes: 434, overtime_minutes: 14, late_minutes: 0, override_type: null },
  { date: _dateOf(1), expected_minutes: 420, worked_minutes: 437, overtime_minutes: 17, late_minutes: 0, override_type: null },
  { date: _dateOf(0), expected_minutes: 420, worked_minutes: 0, overtime_minutes: 0, late_minutes: 0, override_type: null },
  { date: _dateOf(-1), expected_minutes: 0, worked_minutes: 0, overtime_minutes: 0, late_minutes: 0, override_type: 'rtt' },
]
