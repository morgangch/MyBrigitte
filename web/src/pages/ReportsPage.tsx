import { Button } from '../components/ui'
import PageLayout from '../layout/PageLayout.tsx'

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
