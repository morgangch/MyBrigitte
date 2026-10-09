import { useState } from 'react'
import ClockHero from '../components/dashboard/ClockHero.jsx'
import DashboardStats from '../components/dashboard/DashboardStats.jsx'
import LastClockEvents from '../components/dashboard/LastClockEvents.jsx'
import WeekChart from '../components/dashboard/WeekChart.jsx'
import PageLayout from '../layout/PageLayout.jsx'
import { formatDay, toDateKey } from '../lib/time.js'
import { ME, MY_DAILY_SUMMARY, MY_SCHEDULE, MY_SUMMARY, MY_WORK_SESSIONS } from '../lib/fakeData.js'
import './DashboardPage.css'

export default function DashboardPage() {
  const [today] = useState(() => new Date())
  const [sessions, setSessions] = useState(MY_WORK_SESSIONS)

  const currentSession = sessions.find((session) => session.end_time === null)
  const todaySchedule = MY_SCHEDULE.find((schedule) => schedule.date === toDateKey(today))

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
      <DashboardStats currentSession={currentSession} todaySchedule={todaySchedule} summary={MY_SUMMARY} />
      <div className="dashboard-info-cards">
        <WeekChart days={MY_DAILY_SUMMARY} />
        <LastClockEvents sessions={sessions} />
      </div>
    </PageLayout>
  )
}
