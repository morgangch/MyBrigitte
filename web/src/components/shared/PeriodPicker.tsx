import { CalendarDays, ChevronLeft, ChevronRight } from 'lucide-react'
import { Button } from '../ui'
import { addDays, formatShortDate } from '../../lib/time.ts'
import './PeriodPicker.css'

type PeriodPickerProps = {
  weekStart: Date
  onChange: (weekStart: Date) => void
}

export default function PeriodPicker({ weekStart, onChange }: PeriodPickerProps) {
  const weekEnd = addDays(weekStart, 6)

  return (
    <div className="period-picker">
      <Button className="period-arrow" aria-label="Semaine précédente" onClick={() => onChange(addDays(weekStart, -7))}>
        <ChevronLeft size={16} />
      </Button>
      <p className="period-label">
        <CalendarDays size={16} />
        {formatShortDate(weekStart)} – {formatShortDate(weekEnd)} {weekEnd.getFullYear()}
      </p>
      <Button className="period-arrow" aria-label="Semaine suivante" onClick={() => onChange(addDays(weekStart, 7))}>
        <ChevronRight size={16} />
      </Button>
    </div>
  )
}
