import { useState } from 'react'
import PageLayout from '../layout/PageLayout.tsx'
import PeriodPicker from '../components/shared/PeriodPicker.tsx'
import Badge from '../components/ui/Badge.tsx'
import Card from '../components/ui/Card.tsx'
import { addDays, formatDay, formatDuration, formatTime, startOfWeek, toDateKey } from '../lib/time.ts'
import { MY_DAILY_SUMMARY, MY_WORK_SESSIONS } from '../lib/fakeData.ts'
import type { DailySummary, OverrideType, WorkSession } from '../types/api.ts'

const OVERRIDE_LABELS: Record<OverrideType, string> = {
  paid_leave: 'Congé payé',
  rtt: 'RTT',
  day_off: 'Repos',
  custom_hours: 'Horaires aménagés',
}

type DayStatus = {
  tone: 'neutral' | 'success' | 'warning' | 'danger'
  label: string
}

function sessionsOfDay(sessions: WorkSession[], date: string) {
  return sessions.filter((session) => toDateKey(new Date(session.start_time)) === date)
}

function minutesBetween(start: string, end: string) {
  return Math.round((new Date(end).getTime() - new Date(start).getTime()) / 60000)
}

// Pauses = les trous entre la fin d'une session et le début de la suivante
function pauseMinutes(daySessions: WorkSession[]) {
  let total = 0
  for (let index = 1; index < daySessions.length; index++) {
    const previousEnd = daySessions[index - 1].end_time
    if (previousEnd) total += minutesBetween(previousEnd, daySessions[index].start_time)
  }
  return total
}

function getDayStatus(day: DailySummary, daySessions: WorkSession[], todayKey: string): DayStatus {
  if (day.override_type && day.override_type !== 'custom_hours') {
    return { tone: 'neutral', label: OVERRIDE_LABELS[day.override_type] }
  }
  if (day.date > todayKey) return { tone: 'neutral', label: 'À venir' }
  if (daySessions.some((session) => session.end_time === null)) return { tone: 'success', label: 'En cours' }
  if (daySessions.length === 0) return { tone: 'danger', label: 'Absent' }
  if (day.late_minutes > 0) return { tone: 'warning', label: `Retard ${day.late_minutes} min` }
  return { tone: 'success', label: "À l'heure" }
}

export default function HistoryPage() {
  const [todayKey] = useState(() => toDateKey(new Date()))
  const [weekStart, setWeekStart] = useState(() => startOfWeek(new Date()))

  const firstDay = toDateKey(weekStart)
  const lastDay = toDateKey(addDays(weekStart, 6))

  // Les jours de la semaine affichée, du plus récent au plus ancien
  const days = MY_DAILY_SUMMARY
    .filter((day) => day.date >= firstDay && day.date <= lastDay)
    .sort((a, b) => b.date.localeCompare(a.date))

  return (
    <PageLayout title="Mon historique" subtitle="Vos arrivées et départs">
      <Card 
        title='Historique'
        className="card-flush"
        action={<PeriodPicker weekStart={weekStart} onChange={setWeekStart} />}
      >
        <table className="table">
          <thead>
            <tr>
              <th scope="col">Date</th>
              <th scope="col">Arrivée</th>
              <th scope="col">Départ</th>
              <th scope="col">Pauses</th>
              <th scope="col">Total</th>
              <th scope="col">Statut</th>
            </tr>
          </thead>
          <tbody>
            {days.length === 0 && (
              <tr>
                <td colSpan={6} className="table-empty">Aucun pointage cette semaine</td>
              </tr>
            )}
            {days.map((day) => {
              const daySessions = sessionsOfDay(MY_WORK_SESSIONS, day.date)
              const firstSession = daySessions.at(0)
              const lastSession = daySessions.at(-1)
              const status = getDayStatus(day, daySessions, todayKey)

              return (
                <tr key={day.date}>
                  <td>{formatDay(day.date)}</td>
                  <td>{firstSession ? formatTime(firstSession.start_time) : '—'}</td>
                  <td>{lastSession?.end_time ? formatTime(lastSession.end_time) : '—'}</td>
                  <td>{daySessions.length > 1 ? formatDuration(pauseMinutes(daySessions) * 60) : '—'}</td>
                  <td>{day.worked_minutes > 0 ? <strong>{formatDuration(day.worked_minutes * 60)}</strong> : '—'}</td>
                  <td><Badge tone={status.tone}>{status.label}</Badge></td>
                </tr>
              )
            })}
          </tbody>
        </table>
      </Card>
    </PageLayout>
  )
}
