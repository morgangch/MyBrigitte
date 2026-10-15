import { useState } from 'react'
import ClockHero from '../components/dashboard/ClockHero.tsx'
import DashboardStats from '../components/dashboard/DashboardStats.tsx'
import LastClockEvents from '../components/dashboard/LastClockEvents.tsx'
import WeekChart from '../components/dashboard/WeekChart.tsx'
import PageLayout from '../layout/PageLayout.tsx'
import { formatDay, toDateKey } from '../lib/time.ts'
import { ME, MY_DAILY_STATS, MY_SCHEDULE, MY_WEEK_STATS, MY_WORK_POLICY, MY_WORK_SESSIONS } from '../lib/fakeData.ts'
import './DashboardPage.css'

export default function DashboardPage() {
  const [today] = useState(() => new Date())
  const [sessions, setSessions] = useState(MY_WORK_SESSIONS.items)

  const currentSession = sessions.find((session) => session.end_time === null)
  const todaySchedule = MY_SCHEDULE.find((schedule) => schedule.date === toDateKey(today))
  const todayStats = MY_DAILY_STATS.find((day) => day.date === toDateKey(today))?.stats

  // TODO(clock-api) : POST /me/clock-in ou POST /me/clock-out
  function handleClock() {
    if (currentSession) {
      setSessions(sessions.map((session) =>
        session.id === currentSession.id ? { ...session, end_time: new Date().toISOString() } : session,
      ))
    } else {
      setSessions([...sessions, { id: crypto.randomUUID(), start_time: new Date().toISOString(), end_time: null }])
    }
  }

  return (
    <PageLayout title={`Bonjour ${ME.first_name} ${ME.last_name}`} subtitle={formatDay(today)}>
      <ClockHero currentSession={currentSession} onClock={handleClock} />
      <DashboardStats
        currentSession={currentSession}
        todaySchedule={todaySchedule}
        todayStats={todayStats}
        weekStats={MY_WEEK_STATS}
        lateToleranceMinutes={MY_WORK_POLICY.late_tolerance_minutes}
      />
      <div className="dashboard-info-cards">
        <WeekChart days={MY_DAILY_STATS} />
        <LastClockEvents sessions={sessions} />
      </div>
    </PageLayout>
  )
}
