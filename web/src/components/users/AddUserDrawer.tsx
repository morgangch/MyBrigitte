import { useState, type SubmitEvent } from 'react'
import { Button, Drawer, Input, SegmentedControl, Select } from '../ui'
import { TEAMS } from '../../lib/fakeData.ts'
import { ROLE_LABELS, WORK_POLICY_MODE_LABELS } from '../../lib/labels.ts'
import { timeToMinutes, toDateKey } from '../../lib/time.ts'
import type { NewUser, NewWorkPolicy, Role, WorkPolicyMode } from '../../types/api.ts'
import './AddUserDrawer.css'

const TEAM_OPTIONS = [
  { value: '', label: 'Aucune équipe' },
  ...TEAMS.map((team) => ({ value: team.id, label: team.name })),
]

const ROLE_OPTIONS = Object.entries(ROLE_LABELS).map(([value, label]) => ({ value, label }))

const MODE_OPTIONS = Object.entries(WORK_POLICY_MODE_LABELS).map(([value, label]) => ({ value, label }))

const MODE_HINTS: Record<WorkPolicyMode, string> = {
  flexible: "Un objectif d'heures par semaine, sans horaire imposé.",
  fixed: 'Les mêmes horaires du lundi au vendredi. Le détail par jour se règle dans la fiche employé.',
}

const DEFAULT_ROLE: Role = 'employee'
const DEFAULT_MODE: WorkPolicyMode = 'flexible'
const DEFAULT_LATE_TOLERANCE_MINUTES = 5
const MINUTES_PER_DAY = 24 * 60

function workdayMinutes(startTime: string, endTime: string) {
  const minutes = timeToMinutes(endTime) - timeToMinutes(startTime)
  return minutes > 0 ? minutes : minutes + MINUTES_PER_DAY
}

function getScheduleError(startTime: string, endTime: string, breakMinutes: number) {
  if (startTime === endTime) return "L'heure de fin doit être différente de l'heure de début."
  if (breakMinutes >= workdayMinutes(startTime, endTime)) return 'La pause doit être plus courte que la journée.'
  return null
}

function readText(formData: FormData, name: string) {
  return String(formData.get(name) ?? '').trim()
}

type AddUserDrawerProps = {
  open: boolean
  onClose: () => void
  onCreate: (newUser: NewUser) => boolean
}

export default function AddUserDrawer({ open, onClose, onCreate }: AddUserDrawerProps) {
  const [role, setRole] = useState<Role>(DEFAULT_ROLE)
  const [mode, setMode] = useState<WorkPolicyMode>(DEFAULT_MODE)
  const [scheduleError, setScheduleError] = useState<string | null>(null)
  const [today] = useState(() => toDateKey(new Date()))

  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault()
    const form = event.currentTarget
    const formData = new FormData(form)
    const breakMinutes = Number(formData.get('default_break_minutes'))
    const effectiveFrom = readText(formData, 'effective_from')

    let workPolicy: NewWorkPolicy
    if (mode === 'fixed') {
      const startTime = readText(formData, 'weekday_start_time')
      const endTime = readText(formData, 'weekday_end_time')
      const error = getScheduleError(startTime, endTime, breakMinutes)
      setScheduleError(error)
      if (error) return

      workPolicy = {
        mode,
        expected_minutes_per_week: null,
        default_break_minutes: breakMinutes,
        late_tolerance_minutes: Number(formData.get('late_tolerance_minutes')),
        effective_from: effectiveFrom,
        weekday_start_time: `${startTime}:00`,
        weekday_end_time: `${endTime}:00`,
      }
    } else {
      workPolicy = {
        mode,
        expected_minutes_per_week: Number(formData.get('weekly_hours')) * 60 + Number(formData.get('weekly_minutes')),
        default_break_minutes: breakMinutes,
        late_tolerance_minutes: DEFAULT_LATE_TOLERANCE_MINUTES,
        effective_from: effectiveFrom,
        weekday_start_time: null,
        weekday_end_time: null,
      }
    }

    const isCreated = onCreate({
      first_name: readText(formData, 'first_name'),
      last_name: readText(formData, 'last_name'),
      email: readText(formData, 'email'),
      phone_number: readText(formData, 'phone_number') || null,
      team_id: readText(formData, 'team_id') || null,
      role,
      work_policy: workPolicy,
    })

    if (isCreated) {
      form.reset()
      setRole(DEFAULT_ROLE)
      setMode(DEFAULT_MODE)
      onClose()
    }
  }

  return (
    <Drawer
      open={open}
      title="Ajouter un employé"
      description="Il se connectera avec son compte Microsoft professionnel."
      onClose={onClose}
    >
      <form className="add-user-form" onSubmit={handleSubmit}>
        <div className="drawer-body">
          <div className="add-user-row">
            <Input label="Prénom" name="first_name" placeholder="Nora" maxLength={100} autoComplete="off" required />
            <Input label="Nom" name="last_name" placeholder="Benali" maxLength={100} autoComplete="off" required />
          </div>
          <Input
            label="Email Microsoft professionnel"
            name="email"
            type="email"
            placeholder="prenom.nom@mybrigitte.fr"
            maxLength={254}
            autoComplete="off"
            required
          />
          <Input
            label="Téléphone (facultatif)"
            name="phone_number"
            type="tel"
            placeholder="06 00 00 00 00"
            maxLength={20}
            autoComplete="off"
          />
          <SegmentedControl
            label="Rôle"
            name="role"
            options={ROLE_OPTIONS}
            defaultValue={DEFAULT_ROLE}
            hint="Un manager peut gérer son équipe et consulter les rapports."
            onChange={(value) => setRole(value as Role)}
          />
          <Select
            label={role === 'manager' ? 'Équipe gérée' : 'Équipe (facultatif)'}
            name="team_id"
            options={TEAM_OPTIONS}
            required={role === 'manager'}
          />

          <fieldset className="add-user-section">
            <legend>Contrat de travail</legend>
            <SegmentedControl
              label="Type de contrat"
              name="mode"
              options={MODE_OPTIONS}
              defaultValue={DEFAULT_MODE}
              hint={MODE_HINTS[mode]}
              onChange={(value) => setMode(value as WorkPolicyMode)}
            />

            {mode === 'fixed' ? (
              <div key="fixed" className="add-user-row">
                <Input label="Début" name="weekday_start_time" type="time" defaultValue="09:00" required />
                <Input
                  label="Fin"
                  name="weekday_end_time"
                  type="time"
                  defaultValue="17:00"
                  error={scheduleError}
                  required
                />
              </div>
            ) : (
              <div key="flexible" className="add-user-row">
                <Input label="Heures / semaine" name="weekly_hours" type="number" min={0} max={60} defaultValue={35} required />
                <Input label="Minutes" name="weekly_minutes" type="number" min={0} max={59} step={5} defaultValue={0} required />
              </div>
            )}

            <div className="add-user-row">
              <Input label="Pause par jour (min)" name="default_break_minutes" type="number" min={0} step={5} defaultValue={60} required />
              {mode === 'fixed' && (
                <Input
                  label="Tolérance de retard (min)"
                  name="late_tolerance_minutes"
                  type="number"
                  min={0}
                  defaultValue={DEFAULT_LATE_TOLERANCE_MINUTES}
                  required
                />
              )}
            </div>
            <Input label="En vigueur à partir du" name="effective_from" type="date" defaultValue={today} required />
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
