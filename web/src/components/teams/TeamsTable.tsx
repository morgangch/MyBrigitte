import TeamRow from './TeamRow.tsx'
import type { Stats, Team } from '../../types/api.ts'
import './TeamsTable.css'

type TeamsTableProps = {
  teams: Team[]
  statsByTeam: Map<string, Stats>
  totalCount: number
}

export default function TeamsTable({ teams, statsByTeam, totalCount }: TeamsTableProps) {
  return (
    <>
      <table className="table">
        <thead>
          <tr>
            <th scope="col">Équipe</th>
            <th scope="col">Manager</th>
            <th scope="col">Membres</th>
            <th scope="col">Moy. hebdo</th>
            <th scope="col">Ponctualité</th>
            <th scope="col"></th>
          </tr>
        </thead>
        <tbody>
          {teams.map((team) => (
            <TeamRow key={team.id} team={team} stats={team.member_count > 0 ? statsByTeam.get(team.id) : undefined} />
          ))}
          {teams.length === 0 && (
            <tr>
              <td colSpan={6} className="table-empty">Aucune équipe trouvée</td>
            </tr>
          )}
        </tbody>
      </table>
      <p className="teams-table-count">
        {teams.length} sur {totalCount} équipes
      </p>
    </>
  )
}
