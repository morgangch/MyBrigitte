import Button from '../components/ui/Button.tsx'
import PageLayout from '../layout/PageLayout.tsx'


export default function TeamsPage() {
  return (
    <PageLayout
      title="Équipes"
      subtitle="Les équipes et leurs membres"
      actions={<Button variant="primary">Créer une équipe</Button>}
    >
    </PageLayout>
  )
}
