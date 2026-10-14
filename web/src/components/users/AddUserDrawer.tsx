import type { SubmitEvent } from 'react'
import Button from '../ui/Button.tsx'
import Drawer from '../ui/Drawer.tsx'
import Input from '../ui/Input.tsx'
import SegmentedControl from '../ui/SegmentedControl.tsx'
import Select from '../ui/Select.tsx'
import { TEAMS } from '../../lib/fakeData.ts'
import { ROLE_LABELS } from '../../lib/labels.ts'
import type { NewUser, Role } from '../../types/api.ts'
import './AddUserDrawer.css'

const TEAM_OPTIONS = [
  { value: '', label: 'Aucune équipe' },
  ...TEAMS.map((team) => ({ value: team.id, label: team.name })),
]

const ROLE_OPTIONS = Object.entries(ROLE_LABELS).map(([value, label]) => ({ value, label }))

type AddUserDrawerProps = {
  open: boolean
  onClose: () => void
  onCreate: (newUser: NewUser) => boolean
}

export default function AddUserDrawer({ open, onClose, onCreate }: AddUserDrawerProps) {
  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault()
    const form = event.currentTarget
    const formData = new FormData(form)
    const phoneNumber = String(formData.get('phone_number') ?? '').trim()
    const teamId = String(formData.get('team_id') ?? '')
    const weeklyHours = Number(formData.get('weekly_hours'))
    const weeklyMinutes = Number(formData.get('weekly_minutes'))

    const isCreated = onCreate({
      first_name: String(formData.get('first_name') ?? '').trim(),
      last_name: String(formData.get('last_name') ?? '').trim(),
      email: String(formData.get('email') ?? '').trim(),
      phone_number: phoneNumber || null,
      team_id: teamId || null,
      role: String(formData.get('role')) as Role,
      weekly_minutes: weeklyHours * 60 + weeklyMinutes,
    })

    if (isCreated) {
      form.reset()
      onClose()
    }
  }

  return (
    <Drawer
      open={open}
      title="Ajouter un employé"
      description="Renseignez les informations du nouvel employé."
      onClose={onClose}
    >
      <form className="add-user-form" onSubmit={handleSubmit}>
        <div className="drawer-body">
          <div className="add-user-row">
            <Input label="Prénom" name="first_name" placeholder="Nora" autoComplete="off" required />
            <Input label="Nom" name="last_name" placeholder="Benali" autoComplete="off" required />
          </div>
          <Input
            label="Email professionnel"
            name="email"
            type="email"
            placeholder="prenom.nom@mybrigitte.fr"
            autoComplete="off"
            required
          />
          <Input
            label="Téléphone (facultatif)"
            name="phone_number"
            type="tel"
            placeholder="06 00 00 00 00"
            autoComplete="off"
          />
          <Select label="Équipe" name="team_id" options={TEAM_OPTIONS} />
          <SegmentedControl
            label="Rôle"
            name="role"
            options={ROLE_OPTIONS}
            defaultValue="employee"
            hint="Un manager peut gérer son équipe et consulter les rapports."
          />
          <fieldset className="add-user-contract">
            <legend>Contrat hebdomadaire</legend>
            <div className="add-user-row">
              <Input label="Heures" name="weekly_hours" type="number" min={0} max={60} defaultValue={35} required />
              <Input label="Minutes" name="weekly_minutes" type="number" min={0} max={59} step={5} defaultValue={0} required />
            </div>
          </fieldset>
        </div>
        <div className="drawer-footer">
          <Button onClick={onClose}>Annuler</Button>
          <Button type="submit" variant="primary">Créer le compte</Button>
        </div>
      </form>
    </Drawer>
  )
}
