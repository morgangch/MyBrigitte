import { Link } from 'react-router'
import { ChevronRight } from 'lucide-react'
import { Avatar, AvatarGroup, ProgressBar } from '../ui'
import { formatDuration } from '../../lib/time.ts'
import type { Stats, Team, UserRef } from '../../types/api.ts'
import './TeamRow.css'

function fullName(user: UserRef) {
  return `${user.first_name} ${user.last_name}`
}

type TeamRowProps = {
  team: Team
  stats: Stats | undefined
}

export default function TeamRow({ team, stats }: TeamRowProps) {
  const averageWeeklyMinutes = stats?.average_weekly_minutes ?? null
  const punctualityRate = stats?.punctuality_rate ?? null

  return (
    <tr>
      <td>
        <p className="team-row-name">{team.name}</p>
        {team.description && <p className="team-row-description">{team.description}</p>}
      </td>
      <td>
        {team.managers.length === 0 ? (
          <span className="text-secondary">Aucun manager</span>
        ) : (
          <ul className="team-row-managers">
            {team.managers.map((manager) => (
              <li key={manager.id} className="team-row-manager">
                <Avatar name={fullName(manager)} size="small" />
                {fullName(manager)}
              </li>
            ))}
          </ul>
        )}
      </td>
      <td>
        {team.member_count === 0 ? (
          <span className="text-secondary">Aucun membre</span>
        ) : (
          <AvatarGroup
            people={team.member_preview.map((member) => ({ id: member.id, name: fullName(member) }))}
            total={team.member_count}
          />
        )}
      </td>
      <td className="team-row-hours">
        {averageWeeklyMinutes === null ? '—' : formatDuration(averageWeeklyMinutes * 60)}
      </td>
      <td>
        {punctualityRate === null ? (
          '—'
        ) : (
          <div className="team-row-punctuality">
            <ProgressBar label={`Ponctualité de l'équipe ${team.name}`} value={punctualityRate} />
            {Math.round(punctualityRate)} %
          </div>
        )}
      </td>
      <td className="team-row-action">
        <Link to={`/teams/${team.id}`} aria-label={`Voir l'équipe ${team.name}`}>
          <ChevronRight size={18} />
        </Link>
      </td>
    </tr>
  )
}
