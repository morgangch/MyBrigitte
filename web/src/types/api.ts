export type Role = 'employee' | 'manager'

export type WorkPolicyMode = 'fixed' | 'flexible'

export type PresenceStatus = 'present' | 'late' | 'away' | 'not_arrived' | 'absent' | 'day_off'

export type AlertType = 'missing_clock_out' | 'late' | 'absent' | 'overtime_exceeded'

export type Page<T> = {
  items: T[]
  total: number
}

export type TeamRef = {
  id: string
  name: string
}

export type UserRef = {
  id: string
  first_name: string
  last_name: string
}

// GET /users, GET /users/{id}
export type User = {
  id: string
  first_name: string
  last_name: string
  email: string
  phone_number: string | null
  role: Role
  team: TeamRef | null
  is_active: boolean
  created_at: string
}

// GET /me
export type Me = User & {
  has_password: boolean
}

// GET /presence
export type Presence = {
  user: UserRef
  team: TeamRef | null
  status: PresenceStatus
  first_clock_in: string | null
}

// GET /me/work-sessions : end_time à null = session en cours
export type WorkSession = {
  id: string
  start_time: string
  end_time: string | null
}

// GET /me/schedule
export type ScheduleDay = {
  date: string
  start_time: string | null
  end_time: string | null
  break_minutes: number
  expected_minutes: number
}

// GET /me/work-policy
export type WorkPolicy = {
  id: string
  mode: WorkPolicyMode
  expected_minutes_per_week: number | null
  default_break_minutes: number
  late_tolerance_minutes: number
  effective_from: string
  effective_until: string | null
  weekly_schedule: {
    day_of_week: number
    start_time: string | null
    end_time: string | null
    expected_break_minutes: number
  }[]
}

// POST /users/{id}/work-policies (et dans POST /users)
export type NewWorkPolicy = {
  mode: WorkPolicyMode
  expected_minutes_per_week: number | null
  default_break_minutes: number
  late_tolerance_minutes: number
  effective_from: string
  weekday_start_time: string | null
  weekday_end_time: string | null
}

// POST /users
export type NewUser = {
  first_name: string
  last_name: string
  email: string
  phone_number: string | null
  team_id: string
  role: Role
  work_policy: NewWorkPolicy
}

// GET /…/stats
export type Stats = {
  worked_minutes: number
  expected_minutes: number
  overtime_minutes: number
  late_minutes: number
  expected_days: number
  worked_days: number
  late_days: number
  absent_days: number
  average_weekly_minutes: number | null
  punctuality_rate: number | null
  presence_rate: number | null
}

// GET /…/stats?group_by=day
export type DayStats = {
  date: string
  stats: Stats
}

// GET /…/stats?group_by=week : semaine du lundi au dimanche
export type WeekStats = {
  week_start: string
  stats: Stats
}

// GET /stats?group_by=team
export type TeamStats = {
  team: TeamRef
  stats: Stats
}

// GET /…/stats?compare=previous : période précédente de même durée
export type StatsComparison = {
  current: Stats
  previous: Stats
}

// GET /alerts
export type Alert = {
  type: AlertType
  user: UserRef
  team: TeamRef | null
  date: string
  message: string
}

// GET /teams, GET /teams/{id}
export type Team = {
  id: string
  name: string
  description: string | null
  parent: TeamRef | null
  managers: UserRef[]
  member_count: number
  member_preview: UserRef[]
}

// POST /teams
export type NewTeam = {
  name: string
  description: string | null
  parent_team_id: string
  member_ids: string[]
}
