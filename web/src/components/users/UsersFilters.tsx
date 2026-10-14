import { RotateCcw, Search } from 'lucide-react'
import Button from '../ui/Button.tsx'
import Input from '../ui/Input.tsx'
import Select from '../ui/Select.tsx'
import { TEAMS } from '../../lib/fakeData.ts'
import { TODAY_STATUS_LABELS } from '../../lib/labels.ts'
import { EMPTY_FILTERS, hasActiveFilters, type UserFilters } from '../../lib/userFilters.ts'
import './UsersFilters.css'

const TEAM_OPTIONS = [
  { value: '', label: 'Toutes les équipes' },
  ...TEAMS.map((team) => ({ value: team.id, label: team.name })),
]

const STATUS_OPTIONS = [
  { value: '', label: 'Tous les états' },
  ...Object.entries(TODAY_STATUS_LABELS).map(([value, label]) => ({ value, label })),
]

type UsersFiltersProps = {
  filters: UserFilters
  onChange: (filters: UserFilters) => void
}

export default function UsersFilters({ filters, onChange }: UsersFiltersProps) {
  return (
    <div className="users-filters">
      <Input
        aria-label="Rechercher un employé"
        type="search"
        icon={Search}
        placeholder="Nom ou email…"
        value={filters.search}
        onChange={(event) => onChange({ ...filters, search: event.target.value })}
      />
      <Select
        aria-label="Filtrer par équipe"
        options={TEAM_OPTIONS}
        value={filters.teamId}
        onChange={(event) => onChange({ ...filters, teamId: event.target.value })}
      />
      <Select
        aria-label="Filtrer par état"
        options={STATUS_OPTIONS}
        value={filters.status}
        onChange={(event) => onChange({ ...filters, status: event.target.value })}
      />
      <Button
        className="users-filters-reset"
        onClick={() => onChange(EMPTY_FILTERS)}
        disabled={!hasActiveFilters(filters)}
      >
        <RotateCcw size={16} />
      </Button>
    </div>
  )
}
