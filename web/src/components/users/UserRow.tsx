import type { ComponentProps } from 'react'
import { Link } from 'react-router'
import { ChevronRight } from 'lucide-react'
import { Avatar, Badge } from '../ui'
import { ROLE_LABELS, TODAY_STATUS_LABELS } from '../../lib/labels.ts'
import type { TodayStatus, User } from '../../types/api.ts'
import './UserRow.css'

type BadgeTone = ComponentProps<typeof Badge>['tone']

const TODAY_STATUS_TONES: Record<TodayStatus, BadgeTone> = {
  present: 'success',
  late: 'warning',
  not_arrived: 'neutral',
  absent: 'danger',
}

type UserRowProps = {
  user: User
}

export default function UserRow({ user }: UserRowProps) {
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
        <Badge tone={TODAY_STATUS_TONES[user.today_status]}>{TODAY_STATUS_LABELS[user.today_status]}</Badge>
      </td>
      <td className="user-row-action">
        <Link to={`/users/${user.id}`} aria-label={`Voir ${fullName}`}>
          <ChevronRight size={18} />
        </Link>
      </td>
    </tr>
  )
}
