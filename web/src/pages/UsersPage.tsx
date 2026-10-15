import { useCallback, useState, type ComponentProps } from 'react'
import { UserPlus } from 'lucide-react'
import PageLayout from '../layout/PageLayout.tsx'
import { Button, Card, Toast } from '../components/ui'
import AddUserDrawer from '../components/users/AddUserDrawer.tsx'
import UsersFilters from '../components/users/UsersFilters.tsx'
import UsersTable from '../components/users/UsersTable.tsx'
import { TEAMS, USERS } from '../lib/fakeData.ts'
import { EMPTY_FILTERS, filterUsers } from '../lib/userFilters.ts'
import type { NewUser, User } from '../types/api.ts'

type ToastState = {
  id: number
  message: string
  tone: ComponentProps<typeof Toast>['tone']
}

export default function UsersPage() {
  const [users, setUsers] = useState(USERS)
  const [filters, setFilters] = useState(EMPTY_FILTERS)
  const [isAddUserOpen, setIsAddUserOpen] = useState(false)
  const [toast, setToast] = useState<ToastState | null>(null)

  const closeToast = useCallback(() => setToast(null), [])

  const handleCreateUser = (newUser: NewUser) => {
    const emailTaken = users.some((user) => user.email.toLowerCase() === newUser.email.toLowerCase())
    if (emailTaken) {
      setToast({ id: Date.now(), message: 'Un compte existe déjà avec cet email.', tone: 'danger' })
      return false
    }

    const createdUser: User = {
      id: crypto.randomUUID(),
      first_name: newUser.first_name,
      last_name: newUser.last_name,
      email: newUser.email,
      phone_number: newUser.phone_number,
      role: newUser.role,
      team: TEAMS.find((team) => team.id === newUser.team_id) ?? null,
      today_status: 'not_arrived',
    }
    setUsers((currentUsers) => [...currentUsers, createdUser])
    setToast({ id: Date.now(), message: `Compte de ${newUser.first_name} ${newUser.last_name} créé.`, tone: 'success' })
    return true
  }

  return (
    <PageLayout
      title="Utilisateurs"
      subtitle={`${users.length} employés répartis dans ${TEAMS.length} équipes`}
      actions={
        <Button variant="primary" onClick={() => setIsAddUserOpen(true)}>
          <UserPlus size={16} />
          Ajouter un employé
        </Button>
      }
    >
      <Card className="card-flush">
        <UsersFilters filters={filters} onChange={setFilters} />
        <UsersTable users={filterUsers(users, filters)} totalCount={users.length} />
      </Card>

      <AddUserDrawer
        open={isAddUserOpen}
        onClose={() => setIsAddUserOpen(false)}
        onCreate={handleCreateUser}
      />

      {toast && <Toast key={toast.id} message={toast.message} tone={toast.tone} onClose={closeToast} />}
    </PageLayout>
  )
}
