import UserRow from './UserRow.tsx'
import type { PresenceStatus, User } from '../../types/api.ts'
import './UsersTable.css'

type UsersTableProps = {
  users: User[]
  presenceByUser: Map<string, PresenceStatus>
  totalCount: number
}

export default function UsersTable({ users, presenceByUser, totalCount }: UsersTableProps) {
  return (
    <>
      <table className="table">
        <thead>
          <tr>
            <th scope="col">Nom</th>
            <th scope="col">Téléphone</th>
            <th scope="col">Équipe</th>
            <th scope="col">Rôle</th>
            <th scope="col">Aujourd'hui</th>
            <th scope="col"></th>
          </tr>
        </thead>
        <tbody>
          {users.map((user) => (
            <UserRow key={user.id} user={user} status={presenceByUser.get(user.id)} />
          ))}
          {users.length === 0 && (
            <tr>
              <td colSpan={6} className="table-empty">Aucun employé trouvé</td>
            </tr>
          )}
        </tbody>
      </table>
      <p className="users-table-count">
        {users.length} sur {totalCount} employés
      </p>
    </>
  )
}
