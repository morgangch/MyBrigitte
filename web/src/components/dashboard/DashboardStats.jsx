import Card from '../ui/Card.jsx'
import ProgressBar from '../ui/ProgressBar.jsx'
import Stat from '../ui/Stat.jsx'
import useNow from '../../hooks/useNow.js'
import { elapsedSeconds, formatDuration } from '../../lib/time.js'
import './DashboardStats.css'

export default function DashboardStats({ currentSession, todaySchedule, summary }) {
  const now = useNow()
  const workedSeconds = currentSession ? elapsedSeconds(currentSession.start_time, now) : 0
  const expectedTodaySeconds = (todaySchedule?.expected_minutes ?? 0) * 60

  return (
    <Card className="dashboard-stats">
      <div className="dashboard-stat stack-small">
        <Stat
          label="Aujourd'hui"
          value={formatDuration(workedSeconds)}
          unit={todaySchedule ? `/ ${formatDuration(expectedTodaySeconds)}` : ''}
        />
        <ProgressBar label="Progression de la journée" value={workedSeconds} max={expectedTodaySeconds || 1} />
      </div>

      <div className="dashboard-stat stack-small">
        <Stat
          label="Cette semaine"
          value={formatDuration(summary.worked_minutes * 60)}
          unit={`/ ${formatDuration(summary.expected_minutes * 60)}`}
        />
        <ProgressBar label="Progression de la semaine" value={summary.worked_minutes} max={summary.expected_minutes || 1} />
      </div>

      <div className="dashboard-stat">
        <Stat
          label="Heures sup."
          value={`+${formatDuration(summary.overtime_minutes * 60)}`}
          detail="Sur les jours terminés"
        />
      </div>

      <div className="dashboard-stat">
        <Stat
          label="Ponctualité"
          value={`${summary.worked_days - summary.late_days}/${summary.worked_days}`}
          detail={`Jours à l'heure (tolérance ${summary.late_tolerance_minutes} min)`}
        />
      </div>
    </Card>
  )
}
