import { Bar, BarChart, Cell, ReferenceLine, ResponsiveContainer, Tooltip, XAxis, YAxis } from 'recharts'
import Card from '../ui/Card.jsx'
import { formatDuration } from '../../lib/time.js'
import './WeekChart.css'

const workedHours = (day) => (day.worked_minutes - day.overtime_minutes) / 60
const overtimeHours = (day) => day.overtime_minutes / 60
const formatWeekday = (date) => new Date(`${date}T12:00`).toLocaleDateString('fr-FR', { weekday: 'short' })

export default function WeekChart({ days }) {
  return (
    <Card
      title="Ma semaine"
      action={
        <ul className="chart-legend">
          <li className="legend-worked">Travaillé</li>
          <li className="legend-overtime">Heures sup.</li>
        </ul>
      }
    >
      <ResponsiveContainer width="100%" height={240}>
        <BarChart data={days} margin={{ top: 16, right: 0, bottom: 0, left: 0 }}>
          <XAxis
            dataKey="date"
            tickFormatter={formatWeekday}
            axisLine={false}
            tickLine={false}
            tick={{ fill: 'var(--color-text-secondary)', fontSize: 12 }}
          />
          <YAxis hide domain={[0, 9]} />
          <ReferenceLine
            y={7}
            stroke="var(--color-text-muted)"
            strokeDasharray="4 4"
            label={{ value: 'Contrat 7h', position: 'insideTopRight', fill: 'var(--color-text-muted)', fontSize: 11 }}
          />
          <Tooltip
            cursor={{ fill: 'var(--color-surface-alt)' }}
            formatter={(value) => formatDuration(Math.round(value * 3600))}
            wrapperClassName="chart-tooltip"
          />
          <Bar dataKey={workedHours} name="Travaillé" stackId="day" fill="var(--color-primary)" barSize={120}>
            {days.map((day) => (
              <Cell key={day.date} radius={day.overtime_minutes > 0 ? 0 : [4, 4, 0, 0]} />
            ))}
          </Bar>
          <Bar
            dataKey={overtimeHours}
            name="Heures sup."
            stackId="day"
            fill="var(--color-accent)"
            barSize={120}
            radius={[4, 4, 0, 0]}
          />
        </BarChart>
      </ResponsiveContainer>
    </Card>
  )
}
