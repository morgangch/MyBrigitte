import { Avatar } from '../ui'
import { ROLE_LABELS } from '../../lib/labels.ts'
import type { Me } from '../../types/api.ts'
import './ProfileHeader.css'

type ProfileHeaderProps = {
  user: Me
}

export default function ProfileHeader({ user }: ProfileHeaderProps) {
  const fullName = `${user.first_name} ${user.last_name}`

  return (
    <header className="profile-header">
      <Avatar name={fullName} size="large" />
      <div className="profile-header-text">
        <h2>{fullName}</h2>
        <p className="text-secondary">
          {ROLE_LABELS[user.role]}
          {user.team && ` · ${user.team.name}`}
        </p>
      </div>
    </header>
  )
}
