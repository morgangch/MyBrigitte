import HistoryRow from './HistoryRow.tsx'
import { sessionsOfDay } from '../../lib/history.ts'
import type { DayStats, WorkSession } from '../../types/api.ts'

type HistoryTableProps = {
  days: DayStats[]
  sessions: WorkSession[]
  todayKey: string
}

export default function HistoryTable({ days, sessions, todayKey }: HistoryTableProps) {
  return (
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
        {days.map((day) => (
          <HistoryRow key={day.date} day={day} daySessions={sessionsOfDay(sessions, day.date)} todayKey={todayKey} />
        ))}
      </tbody>
    </table>
  )
}
