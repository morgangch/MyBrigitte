import { Card, ProgressBar } from '../ui'
import type { TeamStats } from '../../types/api.ts'
import './PunctualityByTeam.css'

const OBJECTIVE_RATE = 95

type PunctualityByTeamProps = {
  teams: TeamStats[]
}

export default function PunctualityByTeam({ teams }: PunctualityByTeamProps) {
  // Une équipe sans jour travaillé (taux null) n'a rien à afficher
  const rankedTeams = teams
    .filter((team) => team.stats.punctuality_rate !== null)
    .sort((a, b) => (b.stats.punctuality_rate ?? 0) - (a.stats.punctuality_rate ?? 0))

  return (
    <Card title="Ponctualité par équipe" className="punctuality-body" action={<span className="report-hint">Objectif : {OBJECTIVE_RATE} %</span>}>
      {rankedTeams.length === 0 && <p className="punctuality-empty">Aucun jour travaillé sur la période.</p>}
      <ul className="punctuality-list">
        {rankedTeams.map(({ team, stats }) => {
          const rate = Math.round(stats.punctuality_rate ?? 0)
          return (
            <li key={team.id} className="punctuality-row">
              <span className="punctuality-team">{team.name}</span>
              <ProgressBar label={`Ponctualité ${team.name}`} value={rate} />
              <span className={rate < OBJECTIVE_RATE ? 'punctuality-rate punctuality-rate-below' : 'punctuality-rate'}>
                {rate} %
              </span>
            </li>
          )
        })}
      </ul>
    </Card>
  )
}
