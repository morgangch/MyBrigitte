import { Route, Routes } from 'react-router'
import AppLayout from './layout/AppLayout.jsx'
import DashboardPage from './pages/DashboardPage.jsx'
import EmployeePage from './pages/EmployeePage.jsx'
import HistoryPage from './pages/HistoryPage.jsx'
import LoginPage from './pages/LoginPage.jsx'
import NotFoundPage from './pages/NotFoundPage.jsx'
import ProfilePage from './pages/ProfilePage.jsx'
import ReportsPage from './pages/ReportsPage.jsx'
import TeamDetailPage from './pages/TeamDetailPage.jsx'
import TeamsPage from './pages/TeamsPage.jsx'
import UsersPage from './pages/UsersPage.jsx'

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
        <Route path="/users" element={<UsersPage />} />
        <Route path="/users/:id" element={<EmployeePage />} />
        <Route path="/teams" element={<TeamsPage />} />
        <Route path="/teams/:id" element={<TeamDetailPage />} />
        <Route path="/reports" element={<ReportsPage />} />
      </Route>

      {/* Toute autre URL : page 404 */}
      <Route path="*" element={<NotFoundPage />} />
    </Routes>
  )
}
