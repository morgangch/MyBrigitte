import { useCallback, useState, type ComponentProps } from 'react'
import { Plus, Search } from 'lucide-react'
import PageLayout from '../layout/PageLayout.tsx'
import PeriodPicker from '../components/shared/PeriodPicker.tsx'
import AddTeamDrawer from '../components/teams/AddTeamDrawer.tsx'
import TeamsTable from '../components/teams/TeamsTable.tsx'
import { Button, Card, Input, Toast } from '../components/ui'
import { TEAM_STATS, TEAMS, USERS } from '../lib/fakeData.ts'
import { startOfWeek } from '../lib/time.ts'
import type { NewTeam, Team, UserRef } from '../types/api.ts'
import './TeamsPage.css'

const MANAGERS = USERS.items.filter((user) => user.role === 'manager')
const STATS_BY_TEAM = new Map(TEAM_STATS.map((teamStats) => [teamStats.team.id, teamStats.stats]))

type ToastState = {
  id: number
  message: string
  tone: ComponentProps<typeof Toast>['tone']
}

function withoutMember(team: Team, userId: string): Team {
  if (!team.member_preview.some((member) => member.id === userId)) return team
  return {
    ...team,
    member_count: team.member_count - 1,
    managers: team.managers.filter((manager) => manager.id !== userId),
    member_preview: team.member_preview.filter((member) => member.id !== userId),
  }
}

export default function TeamsPage() {
  const [teams, setTeams] = useState(TEAMS.items)
  const [search, setSearch] = useState('')
  const [weekStart, setWeekStart] = useState(() => startOfWeek(new Date()))
  const [isAddTeamOpen, setIsAddTeamOpen] = useState(false)
  const [toast, setToast] = useState<ToastState | null>(null)

  const closeToast = useCallback(() => setToast(null), [])

  const normalizedSearch = search.trim().toLowerCase()
  const filteredTeams = teams.filter((team) =>
    `${team.name} ${team.description ?? ''}`.toLowerCase().includes(normalizedSearch),
  )
  const memberCount = teams.reduce((total, team) => total + team.member_count, 0)

  const handleCreateTeam = (newTeam: NewTeam) => {
    const nameTaken = teams.some((team) => team.name.toLowerCase() === newTeam.name.toLowerCase())
    if (nameTaken) {
      setToast({ id: Date.now(), message: 'Une équipe porte déjà ce nom.', tone: 'danger' })
      return false
    }

    const members: UserRef[] = USERS.items
      .filter((user) => newTeam.member_ids.includes(user.id))
      .map((user) => ({ id: user.id, first_name: user.first_name, last_name: user.last_name }))
    const parent = teams.find((team) => team.id === newTeam.parent_team_id)

    const createdTeam: Team = {
      id: crypto.randomUUID(),
      name: newTeam.name,
      description: newTeam.description,
      parent: parent ? { id: parent.id, name: parent.name } : null,
      managers: members.filter((member) => MANAGERS.some((manager) => manager.id === member.id)),
      member_count: members.length,
      member_preview: members,
    }

    setTeams((currentTeams) => [
      ...currentTeams.map((team) => members.reduce((result, member) => withoutMember(result, member.id), team)),
      createdTeam,
    ])
    setToast({ id: Date.now(), message: `Équipe ${newTeam.name} créée.`, tone: 'success' })
    return true
  }

  return (
    <PageLayout
      title="Équipes"
      subtitle={`${teams.length} équipes · ${memberCount} membres`}
      actions={
        <Button variant="primary" onClick={() => setIsAddTeamOpen(true)}>
          <Plus size={16} />
          Créer une équipe
        </Button>
      }
    >
      <Card className="card-flush">
        <div className="teams-toolbar">
          <Input
            aria-label="Rechercher une équipe"
            type="search"
            icon={Search}
            placeholder="Rechercher une équipe…"
            value={search}
            onChange={(event) => setSearch(event.target.value)}
          />
          <PeriodPicker start={weekStart} onChange={setWeekStart} />
        </div>
        <TeamsTable teams={filteredTeams} statsByTeam={STATS_BY_TEAM} totalCount={teams.length} />
      </Card>

      <AddTeamDrawer
        open={isAddTeamOpen}
        teams={teams}
        managers={MANAGERS}
        onClose={() => setIsAddTeamOpen(false)}
        onCreate={handleCreateTeam}
      />

      {toast && <Toast key={toast.id} message={toast.message} tone={toast.tone} onClose={closeToast} />}
    </PageLayout>
  )
}
