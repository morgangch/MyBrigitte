import { Trash2 } from 'lucide-react'
import PageLayout from '../layout/PageLayout.tsx'
import MicrosoftAccount from '../components/profile/MicrosoftAccount.tsx'
import PasswordForm from '../components/profile/PasswordForm.tsx'
import PersonalInfoForm from '../components/profile/PersonalInfoForm.tsx'
import ProfileHeader from '../components/profile/ProfileHeader.tsx'
import ProfileSection from '../components/profile/ProfileSection.tsx'
import { Button, Card } from '../components/ui'
import { ME } from '../lib/fakeData.ts'

export default function ProfilePage() {
  return (
    <PageLayout title="Mon profil" subtitle="Gérez vos informations et la sécurité de votre compte">
      <Card className="card-flush">
        <ProfileHeader user={ME} />

        <ProfileSection
          title="Informations personnelles"
          description="Ces informations sont visibles par votre manager et votre équipe."
        >
          <PersonalInfoForm user={ME} />
        </ProfileSection>

        <ProfileSection
          title="Sécurité"
          description="Utilisez au moins 12 caractères. Ne réutilisez pas un mot de passe d'un autre site."
        >
          <MicrosoftAccount email={ME.email} />
          <PasswordForm hasPassword={ME.has_password} />
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
