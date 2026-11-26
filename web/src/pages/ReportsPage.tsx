import { useCallback, useState, type ComponentProps } from 'react'
import { Download } from 'lucide-react'
import PageLayout from '../layout/PageLayout.tsx'
import PeriodPicker from '../components/shared/PeriodPicker.tsx'
import ComplianceAlerts from '../components/reports/ComplianceAlerts.tsx'
import PunctualityByTeam from '../components/reports/PunctualityByTeam.tsx'
import ReportKpis from '../components/reports/ReportKpis.tsx'
import WeeklyHoursChart from '../components/reports/WeeklyHoursChart.tsx'
import { Button, Card, Select, Toast } from '../components/ui'
import { ALERTS, REPORT_STATS, REPORT_WEEKLY_STATS, TEAM_STATS, TEAMS } from '../lib/fakeData.ts'
import { addDays, addMonths, startOfMonth, toDateKey } from '../lib/time.ts'
import './ReportsPage.css'

// '' = toutes les équipes (pas de team_id dans la requête)
const TEAM_OPTIONS = [
  { value: '', label: 'Toutes les équipes' },
  ...TEAMS.items.map((team) => ({ value: team.id, label: team.name })),
]

// L'équipe choisie et ses sous-équipes directes (Direction → toutes les autres)
function isInScope(scopeTeamId: string, teamId: string | undefined) {
  if (scopeTeamId === '') return true
  if (teamId === scopeTeamId) return true
  return TEAMS.items.some((team) => team.id === teamId && team.parent?.id === scopeTeamId)
}

type ToastState = {
  id: number
  message: string
  tone: ComponentProps<typeof Toast>['tone']
}

export default function ReportsPage() {
  const [teamId, setTeamId] = useState('')
  const [monthStart, setMonthStart] = useState(() => startOfMonth(new Date()))
  const [isExporting, setIsExporting] = useState(false)
  const [toast, setToast] = useState<ToastState | null>(null)

  const closeToast = useCallback(() => setToast(null), [])

  const from = toDateKey(monthStart)
  const to = toDateKey(addDays(addMonths(monthStart, 1), -1))
  // "août" pour "+1,2 % vs août"
  const previousMonthLabel = addMonths(monthStart, -1).toLocaleDateString('fr-FR', { month: 'long' })

  // TODO(reports-api) : GET /stats?compare=previous, /stats?group_by=week, /stats?group_by=team et /alerts
  // avec from, to et team_id. En attendant, seuls les blocs par équipe suivent le filtre.
  const teamStats = TEAM_STATS.filter((stats) => isInScope(teamId, stats.team.id))
  const alerts = ALERTS.items.filter((alert) => isInScope(teamId, alert.team?.id))

  // TODO(auth-api) : ajouter l'access token quand l'authentification sera branchée
  async function handleExport() {
    const params = new URLSearchParams({ from, to, format: 'csv' })
    if (teamId) params.set('team_id', teamId)

    setIsExporting(true)
    try {
      const response = await fetch(`/api/export/work-sessions?${params}`)
      if (!response.ok) throw new Error(`HTTP ${response.status}`)

      // Le navigateur ne télécharge pas tout seul une réponse de fetch : on passe par un lien temporaire
      const link = document.createElement('a')
      link.href = URL.createObjectURL(await response.blob())
      link.download = `rapport-${from}_${to}.csv`
      link.click()
      URL.revokeObjectURL(link.href)
    } catch {
      setToast({ id: Date.now(), message: "L'export n'a pas pu être généré.", tone: 'danger' })
    } finally {
      setIsExporting(false)
    }
  }

  return (
    <PageLayout
      title="Rapports"
      subtitle="Vue d'ensemble de Trinity Market"
      actions={
        <Button variant="primary" onClick={handleExport} disabled={isExporting}>
          <Download size={16} />
          {isExporting ? 'Export en cours…' : 'Exporter le rapport'}
        </Button>
      }
    >
      <Card className="reports-filters">
        <Select aria-label="Filtrer par équipe" options={TEAM_OPTIONS} value={teamId} onChange={(event) => setTeamId(event.target.value)} />
        <PeriodPicker unit="month" start={monthStart} onChange={setMonthStart} />
      </Card>

      <ReportKpis comparison={REPORT_STATS} previousLabel={previousMonthLabel} />

      <div className="reports-charts">
        <PunctualityByTeam teams={teamStats} />
        <WeeklyHoursChart weeks={REPORT_WEEKLY_STATS} />
      </div>

      <ComplianceAlerts alerts={alerts} total={alerts.length} />

      {toast && <Toast key={toast.id} message={toast.message} tone={toast.tone} onClose={closeToast} />}
    </PageLayout>
  )
}
