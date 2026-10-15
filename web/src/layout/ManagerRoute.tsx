import { Navigate, Outlet } from 'react-router'
import { ME } from '../lib/fakeData.ts'

export default function ManagerRoute() {
  return ME.role === 'manager' ? <Outlet /> : <Navigate to="/" replace />
}
