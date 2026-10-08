import Button from '../components/ui/Button.jsx'
import PageLayout from '../layout/PageLayout.jsx'


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
