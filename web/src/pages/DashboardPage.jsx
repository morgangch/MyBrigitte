import Button from '../components/ui/Button.jsx'
import PageLayout from '../layout/PageLayout.jsx'

export default function DashboardPage() {
  return (
    <PageLayout
      title="Bonjour Jhon Doe"
      subtitle="Mardi 6 octobre _ Bonne journée !"
      actions={<Button variant="primary">Pointer mon départ</Button>}
    >
    </PageLayout>
  )
}
