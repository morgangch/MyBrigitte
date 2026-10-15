import { Route, Routes } from 'react-router'
import AppLayout from './layout/AppLayout.tsx'
import ManagerRoute from './layout/ManagerRoute.tsx'
import DashboardPage from './pages/DashboardPage.tsx'
import EmployeePage from './pages/EmployeePage.tsx'
import HistoryPage from './pages/HistoryPage.tsx'
import LoginPage from './pages/LoginPage.tsx'
import NotFoundPage from './pages/NotFoundPage.tsx'
import ProfilePage from './pages/ProfilePage.tsx'
import ReportsPage from './pages/ReportsPage.tsx'
import TeamDetailPage from './pages/TeamDetailPage.tsx'
import TeamsPage from './pages/TeamsPage.tsx'
import UsersPage from './pages/UsersPage.tsx'

export default function App() {
  return (
    <Routes>
      {/* Page sans sidebar */}
      <Route path="/login" element={<LoginPage />} />

      {/* Toutes ces pages partagent la sidebar (AppLayout + <Outlet />) */}
      <Route element={<AppLayout />}>
        {/* Mon espace */}
        <Route path="/" element={<DashboardPage />} />
        <Route path="/history" element={<HistoryPage />} />
        <Route path="/profile" element={<ProfilePage />} />

        {/* Management */}
        <Route element={<ManagerRoute />}>
          <Route path="/users" element={<UsersPage />} />
          <Route path="/users/:id" element={<EmployeePage />} />
          <Route path="/teams" element={<TeamsPage />} />
          <Route path="/teams/:id" element={<TeamDetailPage />} />
          <Route path="/reports" element={<ReportsPage />} />
        </Route>
      </Route>

      {/* Toute autre URL : page 404 */}
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  )
}
