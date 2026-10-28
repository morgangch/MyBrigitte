import { CalendarDays, ChevronLeft, ChevronRight } from 'lucide-react'
import { Button } from '../ui'
import { addDays, addMonths, formatMonth, formatShortDate } from '../../lib/time.ts'
import './PeriodPicker.css'

type PeriodUnit = 'week' | 'month'

type PeriodPickerProps = {
  // 'week' : `start` est un lundi ; 'month' : `start` est le 1er du mois
  unit?: PeriodUnit
  start: Date
  onChange: (start: Date) => void
}

const ARROW_LABELS = {
  week: { previous: 'Semaine précédente', next: 'Semaine suivante' },
  month: { previous: 'Mois précédent', next: 'Mois suivant' },
}

function shiftPeriod(start: Date, unit: PeriodUnit, steps: number) {
  return unit === 'week' ? addDays(start, steps * 7) : addMonths(start, steps)
}

// "5 oct. – 11 oct. 2026" ou "Octobre 2026"
function formatPeriod(start: Date, unit: PeriodUnit) {
  if (unit === 'month') return formatMonth(start)
  const end = addDays(start, 6)
  return `${formatShortDate(start)} – ${formatShortDate(end)} ${end.getFullYear()}`
}

export default function PeriodPicker({ unit = 'week', start, onChange }: PeriodPickerProps) {
  return (
    <div className="period-picker">
      <Button className="period-arrow" aria-label={ARROW_LABELS[unit].previous} onClick={() => onChange(shiftPeriod(start, unit, -1))}>
        <ChevronLeft size={16} />
      </Button>
      <p className="period-label">
        <CalendarDays size={16} />
        {formatPeriod(start, unit)}
      </p>
      <Button className="period-arrow" aria-label={ARROW_LABELS[unit].next} onClick={() => onChange(shiftPeriod(start, unit, 1))}>
        <ChevronRight size={16} />
      </Button>
    </div>
  )
}
