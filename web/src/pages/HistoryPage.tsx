import { useState } from 'react'
import PageLayout from '../layout/PageLayout.tsx'
import HistoryTable from '../components/history/HistoryTable.tsx'
import PeriodPicker from '../components/shared/PeriodPicker.tsx'
import { Card } from '../components/ui'
import { addDays, startOfWeek, toDateKey } from '../lib/time.ts'
import { MY_DAILY_SUMMARY, MY_WORK_SESSIONS } from '../lib/fakeData.ts'

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
        title="Historique"
        className="card-flush"
        action={<PeriodPicker weekStart={weekStart} onChange={setWeekStart} />}
      >
        <HistoryTable days={days} sessions={MY_WORK_SESSIONS} todayKey={todayKey} />
      </Card>
    </PageLayout>
  )
}
