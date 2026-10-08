import { Outlet } from 'react-router'
import Sidebar from './Sidebar.jsx'

// Structure commune à toutes les pages connectées : la sidebar à gauche, la page à droite
export default function AppLayout() {
  return (
    <div className="app-layout">
      <Sidebar />
      <main className="app-content">
        <Outlet />
      </main>
    </div>
  )
}
