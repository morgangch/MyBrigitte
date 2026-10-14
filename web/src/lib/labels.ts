import type { Role, TodayStatus } from '../types/api.ts'

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
