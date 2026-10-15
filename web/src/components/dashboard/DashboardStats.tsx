import { Card, ProgressBar, Stat } from '../ui'
import useNow from '../../hooks/useNow.ts'
import { elapsedSeconds, formatDuration } from '../../lib/time.ts'
import type { ScheduleDay, Stats, WorkSession } from '../../types/api.ts'
import './DashboardStats.css'

type DashboardStatsProps = {
  currentSession: WorkSession | undefined
  todaySchedule: ScheduleDay | undefined
  todayStats: Stats | undefined
  weekStats: Stats
  lateToleranceMinutes: number
}

export default function DashboardStats({
  currentSession,
  todaySchedule,
  todayStats,
  weekStats,
  lateToleranceMinutes,
}: DashboardStatsProps) {
  const now = useNow()
  const currentSessionSeconds = currentSession ? elapsedSeconds(currentSession.start_time, now) : 0
  const workedTodaySeconds = (todayStats?.worked_minutes ?? 0) * 60 + currentSessionSeconds
  const workedWeekSeconds = weekStats.worked_minutes * 60 + currentSessionSeconds
  const expectedTodaySeconds = (todaySchedule?.expected_minutes ?? 0) * 60
  const expectedWeekSeconds = weekStats.expected_minutes * 60

  return (
    <Card className="dashboard-stats">
      <div className="dashboard-stat stack-small">
        <Stat
          label="Aujourd'hui"
          value={formatDuration(workedTodaySeconds)}
          unit={todaySchedule ? `/ ${formatDuration(expectedTodaySeconds)}` : ''}
        />
        <ProgressBar label="Progression de la journée" value={workedTodaySeconds} max={expectedTodaySeconds || 1} />
      </div>

      <div className="dashboard-stat stack-small">
        <Stat
          label="Cette semaine"
          value={formatDuration(workedWeekSeconds)}
          unit={`/ ${formatDuration(expectedWeekSeconds)}`}
        />
        <ProgressBar label="Progression de la semaine" value={workedWeekSeconds} max={expectedWeekSeconds || 1} />
      </div>

      <div className="dashboard-stat">
        <Stat
          label="Heures sup."
          value={`+${formatDuration(weekStats.overtime_minutes * 60)}`}
          detail="Sur les jours terminés"
        />
      </div>

      <div className="dashboard-stat">
        <Stat
          label="Ponctualité"
          value={`${weekStats.worked_days - weekStats.late_days}/${weekStats.worked_days}`}
          detail={`Jours à l'heure (tolérance ${lateToleranceMinutes} min)`}
        />
      </div>
    </Card>
  )
}
