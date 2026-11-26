import type { SubmitEvent } from 'react'
import { Button, Input } from '../ui'
import type { Me } from '../../types/api.ts'
import './ProfileForm.css'

type PersonalInfoFormProps = {
  user: Me
}

export default function PersonalInfoForm({ user }: PersonalInfoFormProps) {

  // TODO(profile-api) : PATCH /me
  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault()
  }

  return (
    <form className="profile-form" onSubmit={handleSubmit}>
      <div className="profile-form-grid">
        <Input label="Prénom" name="first_name" autoComplete="given-name" defaultValue={user.first_name} required />
        <Input label="Nom" name="last_name" autoComplete="family-name" defaultValue={user.last_name} required />
        <Input label="Email (compte Microsoft)" type="email" defaultValue={user.email} readOnly />
        <Input label="Téléphone" name="phone_number" type="tel" autoComplete="tel" defaultValue={user.phone_number ?? ''} />
      </div>
      <div className="profile-form-actions">
        <Button type="submit" variant="primary">Enregistrer</Button>
      </div>
    </form>
  )
}
