import { Trash2 } from 'lucide-react'
import PageLayout from '../layout/PageLayout.tsx'
import MicrosoftAccount from '../components/profile/MicrosoftAccount.tsx'
import ProfileSection from '../components/profile/ProfileSection.tsx'
import Avatar from '../components/ui/Avatar.tsx'
import Button from '../components/ui/Button.tsx'
import Card from '../components/ui/Card.tsx'
import Input from '../components/ui/Input.tsx'
import { ME } from '../lib/fakeData.ts'
import type { Role } from '../types/api.ts'
import './ProfilePage.css'

const ROLE_LABELS: Record<Role, string> = {
  employee: 'Employé',
  manager: 'Manager',
}

export default function ProfilePage() {
  return (
    <PageLayout title="Mon profil" subtitle="Gérez vos informations et la sécurité de votre compte">
      <Card className="card-flush">
        <header className="profile-header">
          <Avatar name={`${ME.first_name} ${ME.last_name}`} size="large" />
          <div className="profile-header-text">
            <h2>{`${ME.first_name} ${ME.last_name}`}</h2>
            <p className="text-secondary">
              {ROLE_LABELS[ME.role]}
              {ME.team && ` · ${ME.team.name}`}
            </p>
          </div>
        </header>
        <ProfileSection
          title="Informations personnelles"
          description="Ces informations sont visibles par votre manager et votre équipe."
        >
          <form className="profile-form">
            <div className="profile-form-grid">
              <Input label="Prénom" name="first_name" autoComplete="given-name" defaultValue={ME.first_name} required />
              <Input label="Nom" name="last_name" autoComplete="family-name" defaultValue={ME.last_name} required />
              <Input label="Email" name="email" type="email" autoComplete="email" defaultValue={ME.email} required />
              <Input label="Téléphone" name="phone_number" type="tel" autoComplete="tel" defaultValue={ME.phone_number ?? ''} />
            </div>
            <div className="profile-form-actions">
              <Button type="submit" variant="primary">Enregistrer</Button>
            </div>
          </form>
        </ProfileSection>

        <ProfileSection
          title="Sécurité"
          description="Utilisez au moins 12 caractères. Ne réutilisez pas un mot de passe d'un autre site."
        >
          <MicrosoftAccount email={ME.microsoft_email} />
          <form className="profile-form">
            <Input label="Mot de passe actuel" name="current_password" type="password" autoComplete="current-password" placeholder="*************" required />
            <div className="profile-form-grid">
              <Input label="Nouveau mot de passe" name="new_password" type="password" autoComplete="new-password" placeholder="*************" minLength={12} required />
              <Input label="Confirmer" name="confirm_password" type="password" autoComplete="new-password" placeholder="*************" minLength={12} required />
            </div>
            <div className="profile-form-actions">
              <Button type="submit">Mettre à jour le mot de passe</Button>
            </div>
          </form>
        </ProfileSection>

        <ProfileSection
          title="Supprimer mon compte"
          description="Vos données personnelles seront effacées sous 30 jours, conformément au RGPD. Les heures déjà validées restent anonymisées pour la paie."
        >
          <div>
            <Button variant="danger">
              <Trash2 size={16} />
              Supprimer mon compte
            </Button>
          </div>
        </ProfileSection>
      </Card>
    </PageLayout>
  )
}
