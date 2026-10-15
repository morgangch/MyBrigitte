import type { PresenceStatus, Role, WorkPolicyMode } from '../types/api.ts'

export const ROLE_LABELS: Record<Role, string> = {
  employee: 'Employé',
  manager: 'Manager',
}

export const PRESENCE_STATUS_LABELS: Record<PresenceStatus, string> = {
  present: 'Présent',
  late: 'En retard',
  away: 'Sorti',
  not_arrived: 'Pas encore arrivé',
  absent: 'Absent',
  day_off: 'Repos',
}

export const WORK_POLICY_MODE_LABELS: Record<WorkPolicyMode, string> = {
  flexible: 'Flexible',
  fixed: 'Horaires fixes',
}
