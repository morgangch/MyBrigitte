import { Bar, BarChart, Rectangle, ReferenceLine, ResponsiveContainer, Tooltip, XAxis, YAxis, type BarShapeProps } from 'recharts'
import { Card } from '../ui'
import { formatDuration, isoWeekNumber } from '../../lib/time.ts'
import type { WeekStats } from '../../types/api.ts'
import '../dashboard/WeekChart.css'

const CONTRACT_HOURS = 35

// `Stats` donne la moyenne travaillée par personne, mais les heures sup. au total.
// Nombre de personnes = total travaillé / moyenne par personne.
function averageOvertimeMinutes({ stats }: WeekStats) {
  if (!stats.average_weekly_minutes || stats.worked_minutes === 0) return 0
  const people = stats.worked_minutes / stats.average_weekly_minutes
  return stats.overtime_minutes / people
}

const overtimeHours = (week: WeekStats) => averageOvertimeMinutes(week) / 60
const workedHours = (week: WeekStats) => ((week.stats.average_weekly_minutes ?? 0) - averageOvertimeMinutes(week)) / 60
const formatWeek = (weekStart: string) => `S${isoWeekNumber(new Date(`${weekStart}T12:00`))}`

function WorkedBar(props: BarShapeProps) {
  const week = props.payload as WeekStats
  return <Rectangle {...props} radius={week.stats.overtime_minutes > 0 ? 0 : [4, 4, 0, 0]} />
}

type WeeklyHoursChartProps = {
  weeks: WeekStats[]
}

export default function WeeklyHoursChart({ weeks }: WeeklyHoursChartProps) {
  return (
    <Card
      title="Heures moyennes par semaine"
      action={
        <ul className="chart-legend">
          <li className="legend-worked">Travaillé</li>
          <li className="legend-overtime">Heures sup.</li>
        </ul>
      }
    >
      <ResponsiveContainer width="100%" height={240}>
        <BarChart data={weeks} margin={{ top: 16, right: 0, bottom: 0, left: 0 }}>
          <XAxis
            dataKey="week_start"
            tickFormatter={formatWeek}
            axisLine={false}
            tickLine={false}
            tick={{ fill: 'var(--color-text-secondary)', fontSize: 12 }}
          />
          <YAxis hide domain={[0, 40]} />
          <ReferenceLine
            y={CONTRACT_HOURS}
            stroke="var(--color-text-muted)"
            strokeDasharray="4 4"
            label={{ value: `Contrat ${CONTRACT_HOURS}h`, position: 'insideTopRight', fill: 'var(--color-text-muted)', fontSize: 11 }}
          />
          <Tooltip
            cursor={{ fill: 'var(--color-surface-alt)' }}
            labelFormatter={(weekStart) => formatWeek(String(weekStart))}
            formatter={(value) => formatDuration(Math.round(Number(value) * 3600))}
            wrapperClassName="chart-tooltip"
          />
          <Bar dataKey={workedHours} name="Travaillé" stackId="week" fill="var(--color-primary)" barSize={48} shape={WorkedBar} />
          <Bar dataKey={overtimeHours} name="Heures sup." stackId="week" fill="var(--color-accent)" barSize={48} radius={[4, 4, 0, 0]} />
        </BarChart>
      </ResponsiveContainer>
    </Card>
  )
}
