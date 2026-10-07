import { useParams } from 'react-router'
import PageLayout from '../layout/PageLayout.jsx'

export default function TeamDetailPage() {
  const { id } = useParams()

  return (
    <PageLayout title={`Équipe n°${id}`} subtitle="Moyennes de l'équipe sur la période">
    </PageLayout>
  )
}
