import Button from '../components/ui/Button.jsx'
import PageLayout from '../layout/PageLayout.jsx'

export default function ReportsPage() {
  return (
    <PageLayout
      title="Rapports"
      subtitle="Vue d'ensemble de Trinity Market"
      actions={<Button>Exporter le rapport</Button>}
    >
    </PageLayout>
  )
}
