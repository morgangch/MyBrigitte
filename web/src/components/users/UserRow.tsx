import type { ComponentProps } from 'react'
import { Link } from 'react-router'
import { ChevronRight } from 'lucide-react'
import { Avatar, Badge } from '../ui'
import { PRESENCE_STATUS_LABELS, ROLE_LABELS } from '../../lib/labels.ts'
import type { PresenceStatus, User } from '../../types/api.ts'
import './UserRow.css'

type BadgeTone = ComponentProps<typeof Badge>['tone']

const PRESENCE_STATUS_TONES: Record<PresenceStatus, BadgeTone> = {
  present: 'success',
  late: 'warning',
  away: 'neutral',
  not_arrived: 'neutral',
  absent: 'danger',
  day_off: 'neutral',
}

type UserRowProps = {
  user: User
  status: PresenceStatus | undefined
}

export default function UserRow({ user, status }: UserRowProps) {
  const fullName = `${user.first_name} ${user.last_name}`

  return (
    <tr>
      <td>
        <div className="user-row-identity">
          <Avatar name={fullName} />
          <div>
            <p className="user-row-name">{fullName}</p>
            <p className="user-row-email">{user.email}</p>
          </div>
        </div>
      </td>
      <td className="text-secondary">{user.phone_number ?? '—'}</td>
      <td>{user.team?.name ?? '—'}</td>
      <td>
        <Badge tone={user.role === 'manager' ? 'primary' : 'neutral'}>{ROLE_LABELS[user.role]}</Badge>
      </td>
      <td>
        {status ? <Badge tone={PRESENCE_STATUS_TONES[status]}>{PRESENCE_STATUS_LABELS[status]}</Badge> : '—'}
      </td>
      <td className="user-row-action">
        <Link to={`/users/${user.id}`} aria-label={`Voir ${fullName}`}>
          <ChevronRight size={18} />
        </Link>
      </td>
    </tr>
  )
}
