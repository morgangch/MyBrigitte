import { OVERRIDE_LABELS } from './labels.ts'
import { toDateKey } from './time.ts'
import type { DailySummary, WorkSession } from '../types/api.ts'

export type DayStatus = {
  tone: 'neutral' | 'success' | 'warning' | 'danger'
  label: string
}

export function sessionsOfDay(sessions: WorkSession[], date: string) {
  return sessions.filter((session) => toDateKey(new Date(session.start_time)) === date)
}

function minutesBetween(start: string, end: string) {
  return Math.round((new Date(end).getTime() - new Date(start).getTime()) / 60000)
}

// Pauses = les trous entre la fin d'une session et le début de la suivante
export function pauseMinutes(daySessions: WorkSession[]) {
  let total = 0
  for (let index = 1; index < daySessions.length; index++) {
    const previousEnd = daySessions[index - 1].end_time
    if (previousEnd) total += minutesBetween(previousEnd, daySessions[index].start_time)
  }
  return total
}

export function getDayStatus(day: DailySummary, daySessions: WorkSession[], todayKey: string): DayStatus {
  if (day.override_type && day.override_type !== 'custom_hours') {
    return { tone: 'neutral', label: OVERRIDE_LABELS[day.override_type] }
  }
  if (day.date > todayKey) return { tone: 'neutral', label: 'À venir' }
  if (daySessions.some((session) => session.end_time === null)) return { tone: 'success', label: 'En cours' }
  if (daySessions.length === 0) return { tone: 'danger', label: 'Absent' }
  if (day.late_minutes > 0) return { tone: 'warning', label: `Retard ${day.late_minutes} min` }
  return { tone: 'success', label: "À l'heure" }
}
