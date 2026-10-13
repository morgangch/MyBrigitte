import Button from '../components/ui/Button.tsx'
import PageLayout from '../layout/PageLayout.tsx'

export default function UsersPage() {
  return (
    <PageLayout
      title="Utilisateurs"
      subtitle="Les employés de l'entreprise"
      actions={<Button variant="primary">Ajouter un employé</Button>}
    >
    </PageLayout>
  )
}
