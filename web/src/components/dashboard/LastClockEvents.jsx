import { ChevronLeft, ChevronRight } from 'lucide-react'
import { Link } from 'react-router'
import Card from '../ui/Card.jsx'
import { formatDay, formatTime } from '../../lib/time.js'
import './LastClockEvents.css'

function getLastEvents(sessions, limit) {
  return sessions
    .flatMap((session) => [
      { sessionId: session.id, type: 'arrival', time: session.start_time },
      ...(session.end_time ? [{ sessionId: session.id, type: 'departure', time: session.end_time }] : []),
    ])
    .sort((a, b) => new Date(b.time) - new Date(a.time))
    .slice(0, limit)
}

export default function LastClockEvents({ sessions, limit = 4 }) {
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
