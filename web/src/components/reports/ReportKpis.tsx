import { Card, Stat } from '../ui'
import { formatDuration } from '../../lib/time.ts'
import type { StatsComparison } from '../../types/api.ts'
import './ReportKpis.css'

// true : une hausse est une bonne nouvelle · false : une baisse est une bonne nouvelle · null : ni l'un ni l'autre
type Direction = boolean | null

type KpiProps = {
  label: string
  value: string
  delta: number | null
  deltaText: string
  higherIsBetter: Direction
  previousLabel: string
}

const formatNumber = (value: number) => value.toLocaleString('fr-FR', { maximumFractionDigits: 1 })
const formatRate = (rate: number | null) => (rate === null ? '—' : `${formatNumber(rate)} %`)

// Différence entre deux valeurs qui peuvent être null (rien à mesurer sur la période)
const difference = (now: number | null, before: number | null) => (now === null || before === null ? null : now - before)

function trendTone(delta: number, higherIsBetter: Direction) {
  if (delta === 0 || higherIsBetter === null) return 'neutral'
  return delta > 0 === higherIsBetter ? 'good' : 'bad'
}

function Kpi({ label, value, delta, deltaText, higherIsBetter, previousLabel }: KpiProps) {
  return (
    <div className="report-kpi">
      <Stat label={label} value={value} />
      {delta !== null && (
        // "−" typographique (et pas "-") pour qu'il ait la même largeur que "+"
        <p className={`report-kpi-trend report-kpi-trend-${trendTone(delta, higherIsBetter)}`}>
          {delta < 0 ? '−' : '+'}{deltaText} vs {previousLabel}
        </p>
      )}
    </div>
  )
}

type ReportKpisProps = {
  comparison: StatsComparison
  // "août" : nom du mois précédent
  previousLabel: string
}

export default function ReportKpis({ comparison, previousLabel }: ReportKpisProps) {
  const { current, previous } = comparison

  const punctualityDelta = difference(current.punctuality_rate, previous.punctuality_rate)
  const weeklyDelta = difference(current.average_weekly_minutes, previous.average_weekly_minutes)
  const presenceDelta = difference(current.presence_rate, previous.presence_rate)
  const overtimeDelta = previous.overtime_minutes > 0 ? (current.overtime_minutes / previous.overtime_minutes - 1) * 100 : null

  return (
    <Card className="report-kpis">
      <Kpi
        label="Taux de ponctualité"
        value={formatRate(current.punctuality_rate)}
        delta={punctualityDelta}
        deltaText={`${formatNumber(Math.abs(punctualityDelta ?? 0))} %`}
        higherIsBetter
        previousLabel={previousLabel}
      />
      <Kpi
        label="Heures moyennes / semaine"
        value={current.average_weekly_minutes === null ? '—' : formatDuration(current.average_weekly_minutes * 60)}
        delta={weeklyDelta}
        deltaText={formatDuration(Math.abs(weeklyDelta ?? 0) * 60)}
        higherIsBetter={null}
        previousLabel={previousLabel}
      />
      <Kpi
        label="Heures supplémentaires"
        value={`${Math.round(current.overtime_minutes / 60)}h`}
        delta={overtimeDelta}
        deltaText={`${Math.round(Math.abs(overtimeDelta ?? 0))} %`}
        higherIsBetter={false}
        previousLabel={previousLabel}
      />
      <Kpi
        label="Taux de présence"
        value={formatRate(current.presence_rate)}
        delta={presenceDelta}
        deltaText={`${formatNumber(Math.abs(presenceDelta ?? 0))} %`}
        higherIsBetter
        previousLabel={previousLabel}
      />
    </Card>
  )
}
