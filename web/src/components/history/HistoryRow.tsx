import { Badge } from '../ui'
import { getDayStatus, pauseMinutes } from '../../lib/history.ts'
import { formatDay, formatDuration, formatTime } from '../../lib/time.ts'
import type { DailySummary, WorkSession } from '../../types/api.ts'

type HistoryRowProps = {
  day: DailySummary
  daySessions: WorkSession[]
  todayKey: string
}

export default function HistoryRow({ day, daySessions, todayKey }: HistoryRowProps) {
  const firstSession = daySessions.at(0)
  const lastSession = daySessions.at(-1)
  const status = getDayStatus(day, daySessions, todayKey)

  return (
    <tr>
      <td>{formatDay(day.date)}</td>
      <td>{firstSession ? formatTime(firstSession.start_time) : '—'}</td>
      <td>{lastSession?.end_time ? formatTime(lastSession.end_time) : '—'}</td>
      <td>{daySessions.length > 1 ? formatDuration(pauseMinutes(daySessions) * 60) : '—'}</td>
      <td>{day.worked_minutes > 0 ? <strong>{formatDuration(day.worked_minutes * 60)}</strong> : '—'}</td>
      <td><Badge tone={status.tone}>{status.label}</Badge></td>
    </tr>
  )
}
