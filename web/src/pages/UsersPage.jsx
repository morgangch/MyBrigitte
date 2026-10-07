import Button from '../components/ui/Button.jsx'
import PageLayout from '../layout/PageLayout.jsx'

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
