import type { OverrideType, Role, TodayStatus, WorkPolicyMode } from '../types/api.ts'

export const ROLE_LABELS: Record<Role, string> = {
  employee: 'Employé',
  manager: 'Manager',
}

export const TODAY_STATUS_LABELS: Record<TodayStatus, string> = {
  present: 'Présent',
  late: 'En retard',
  not_arrived: 'Pas encore arrivé',
  absent: 'Absent',
}

export const WORK_POLICY_MODE_LABELS: Record<WorkPolicyMode, string> = {
  flexible: 'Flexible',
  fixed: 'Horaires fixes',
}

export const OVERRIDE_LABELS: Record<OverrideType, string> = {
  paid_leave: 'Congé payé',
  rtt: 'RTT',
  day_off: 'Repos',
  custom_hours: 'Horaires aménagés',
}
