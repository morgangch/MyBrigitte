import type { SubmitEvent } from 'react'
import { Button, Drawer, Input, Select } from '../ui'
import type { NewTeam, Team, User } from '../../types/api.ts'
import './AddTeamDrawer.css'

function readText(formData: FormData, name: string) {
  return String(formData.get(name) ?? '').trim()
}

type AddTeamDrawerProps = {
  open: boolean
  teams: Team[]
  managers: User[]
  onClose: () => void
  onCreate: (newTeam: NewTeam) => boolean
}

export default function AddTeamDrawer({ open, teams, managers, onClose, onCreate }: AddTeamDrawerProps) {
  const parentOptions = [
    { value: '', label: 'Choisir une équipe parente' },
    ...teams.map((team) => ({ value: team.id, label: team.name })),
  ]

  const managerOptions = [
    { value: '', label: 'Choisir plus tard' },
    ...managers.map((manager) => ({
      value: manager.id,
      label: `${manager.first_name} ${manager.last_name}${manager.team ? ` (${manager.team.name})` : ''}`,
    })),
  ]

  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault()
    const form = event.currentTarget
    const formData = new FormData(form)

    const managerId = readText(formData, 'manager_id')
    const isCreated = onCreate({
      name: readText(formData, 'name'),
      description: readText(formData, 'description') || null,
      parent_team_id: readText(formData, 'parent_team_id'),
      member_ids: managerId ? [managerId] : [],
    })

    if (isCreated) {
      form.reset()
      onClose()
    }
  }

  return (
    <Drawer
      open={open}
      title="Créer une équipe"
      description="Vous pourrez ajouter les membres depuis la page de l'équipe."
      onClose={onClose}
    >
      <form className="add-team-form" onSubmit={handleSubmit}>
        <div className="drawer-body">
          <Input label="Nom de l'équipe" name="name" placeholder="Service client" maxLength={100} autoComplete="off" required />
          <Input
            label="Description (facultatif)"
            name="description"
            placeholder="Support et relation clients"
            maxLength={255}
            autoComplete="off"
          />
          <Select label="Équipe parente" name="parent_team_id" options={parentOptions} required />
          <div className="add-team-field">
            <Select label="Manager" name="manager_id" options={managerOptions} />
            <p className="add-team-hint">
              Un manager n'appartient qu'à une seule équipe : il quittera son équipe actuelle.
            </p>
          </div>
        </div>
        <div className="drawer-footer">
          <Button onClick={onClose}>Annuler</Button>
          <Button type="submit" variant="primary">Créer l'équipe</Button>
        </div>
      </form>
    </Drawer>
  )
}
