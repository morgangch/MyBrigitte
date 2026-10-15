
export type Role = 'employee' | 'manager'

export type OverrideType = 'paid_leave' | 'rtt' | 'day_off' | 'custom_hours'

// GET /me
export type Me = {
  id: string
  first_name: string
  last_name: string
  email: string
  phone_number: string | null
  microsoft_email: string | null
  has_password: boolean
  role: Role
  team: Team | null
}

// GET /me/work-sessions : end_time à null = session en cours
export type WorkSession = {
  id: string
  start_time: string
  end_time: string | null
}

// GET /me/schedule : ce qui est prévu un jour donné
export type ScheduleDay = {
  date: string
  start_time: string | null
  end_time: string | null
  break_minutes: number | null
  expected_minutes: number
  override_type: OverrideType | null
}

// GET /me/summary
export type Summary = {
  from: string
  to: string
  worked_minutes: number
  expected_minutes: number
  overtime_minutes: number
  worked_days: number
  late_days: number
  late_tolerance_minutes: number
}

// GET /me/summary?group_by=day
export type DailySummary = {
  date: string
  expected_minutes: number
  worked_minutes: number
  overtime_minutes: number
  late_minutes: number
  override_type: OverrideType | null
}

export type TodayStatus = 'present' | 'late' | 'not_arrived' | 'absent'

// GET /teams
export type Team = {
  id: string
  name: string
}

// GET /users
export type User = {
  id: string
  first_name: string
  last_name: string
  email: string
  phone_number: string | null
  role: Role
  team: Team | null
  today_status: TodayStatus
}

export type WorkPolicyMode = 'fixed' | 'flexible'

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
  team_id: string | null
  role: Role
  work_policy: NewWorkPolicy
}
