import { useParams } from 'react-router'
import PageLayout from '../layout/PageLayout.tsx'

export default function EmployeePage() {
  const { id } = useParams()

  return (
    <PageLayout title={`Employé n°${id}`} subtitle="Heures par jour et par semaine sur une période">
    </PageLayout>
  )
}
