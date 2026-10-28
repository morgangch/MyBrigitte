import type { SubmitEvent } from 'react'
import { Button, Input } from '../ui'
import './ProfileForm.css'

type PasswordFormProps = {
  hasPassword: boolean
}

export default function PasswordForm({ hasPassword }: PasswordFormProps) {

  // TODO(profile-api) : POST /me/password
  const handleSubmit = (event: SubmitEvent<HTMLFormElement>) => {
    event.preventDefault()
  }

  return (
    <form className="profile-form" onSubmit={handleSubmit}>
      {hasPassword ? (
        <Input label="Mot de passe actuel" name="current_password" type="password" autoComplete="current-password" placeholder="*************" required />
      ) : (
        <p className="text-secondary">Vous vous connectez avec Microsoft. Définissez un mot de passe pour pouvoir aussi vous connecter avec votre email.</p>
      )}
      <div className="profile-form-grid">
        <Input label="Nouveau mot de passe" name="new_password" type="password" autoComplete="new-password" placeholder="*************" minLength={12} required />
        <Input label="Confirmer" name="confirm_password" type="password" autoComplete="new-password" placeholder="*************" minLength={12} required />
      </div>
      <div className="profile-form-actions">
        <Button type="submit">{hasPassword ? 'Mettre à jour le mot de passe' : 'Définir le mot de passe'}</Button>
      </div>
    </form>
  )
}
