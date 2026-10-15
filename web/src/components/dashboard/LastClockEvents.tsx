import { ChevronLeft, ChevronRight } from 'lucide-react'
import { Link } from 'react-router'
import { Card } from '../ui'
import { formatDay, formatTime } from '../../lib/time.ts'
import type { WorkSession } from '../../types/api.ts'
import './LastClockEvents.css'

type ClockEvent = {
  sessionId: string
  type: 'arrival' | 'departure'
  time: string
}

function getLastEvents(sessions: WorkSession[], limit: number): ClockEvent[] {
  return sessions
    .flatMap((session): ClockEvent[] => [
      { sessionId: session.id, type: 'arrival', time: session.start_time },
      ...(session.end_time ? [{ sessionId: session.id, type: 'departure' as const, time: session.end_time }] : []),
    ])
    .sort((a, b) => new Date(b.time).getTime() - new Date(a.time).getTime())
    .slice(0, limit)
}

type LastClockEventsProps = {
  sessions: WorkSession[]
  limit?: number
}

export default function LastClockEvents({ sessions, limit = 4 }: LastClockEventsProps) {
  const lastEvents = getLastEvents(sessions, limit)

  return (
    <Card
      title="Derniers pointages"
      className="clock-events-card"
      action={<Link to="/history" className="link">Tout voir</Link>}
    >
      <ul>
        {lastEvents.map((event) => (
          <li key={`${event.sessionId}-${event.type}`} className="clock-event">
            {event.type === 'arrival' ? <ChevronRight size={18} /> : <ChevronLeft size={18} />}
            <div>
              <p>{event.type === 'arrival' ? 'Arrivée' : 'Départ'}</p>
              <p className="text-secondary">{formatDay(event.time, true)}</p>
            </div>
            <span className="clock-event-time">{formatTime(event.time)}</span>
          </li>
        ))}
      </ul>
    </Card>
  )
}
