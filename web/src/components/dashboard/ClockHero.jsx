import { Play, Square } from 'lucide-react'
import Badge from '../ui/Badge.jsx'
import Button from '../ui/Button.jsx'
import useNow from '../../hooks/useNow.js'
import { elapsedSeconds, formatTime, splitDuration } from '../../lib/time.js'
import './ClockHero.css'

function pad(value) {
  return value.toString().padStart(2, '0')
}

export default function ClockHero({ currentSession, onClock }) {
  const now = useNow()
  const isClockedIn = currentSession !== undefined
  const { hours, minutes, seconds } = splitDuration(isClockedIn ? elapsedSeconds(currentSession.start_time, now) : 0)

  return (
    <section className="clock-hero" aria-label="Pointage du jour">
      <div className="clock-hero-time">
        {isClockedIn ? (
          <Badge tone="success">En poste depuis {formatTime(currentSession.start_time)}</Badge>
        ) : (
          <Badge tone="neutral">Pas en poste</Badge>
        )}
        <p className={isClockedIn ? 'clock-time' : 'clock-time text-secondary'} role="timer">
          {pad(hours)}:{pad(minutes)}<span className="text-secondary">:{pad(seconds)}</span>
        </p>
        <p className="clock-caption">
          {isClockedIn ? 'Temps travaillé durant cette session' : 'Veuillez pointer votre arrivée'}
        </p>
      </div>

      <Button variant="primary" onClick={onClock} className="clock-button">
        {isClockedIn ? <Square size={32} /> : <Play size={32} />}
        {isClockedIn ? 'Pointer mon départ' : 'Pointer mon arrivée'}
      </Button>
    </section>
  )
}
