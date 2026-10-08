import { ChartColumn, CircleUser, History, LayoutDashboard, UserCog, Users } from 'lucide-react'
import { NavLink } from 'react-router'
import Logo from '../components/ui/Logo.jsx'
import './Sidebar.css'

const MENU = {
  'Mon espace': [
    { label: 'Tableau de bord', path: '/', icon: LayoutDashboard },
    { label: 'Mon historique', path: '/history', icon: History },
    { label: 'Mon profil', path: '/profile', icon: CircleUser },
  ],
  'Management': [
    { label: 'Utilisateurs', path: '/users', icon: UserCog },
    { label: 'Équipes', path: '/teams', icon: Users },
    { label: 'Rapports', path: '/reports', icon: ChartColumn },
  ],
}

export default function Sidebar() {
  return (
    <nav className="sidebar" aria-label="Navigation principale">
      <NavLink to="/" end style={{ paddingBottom: '20px' }}>
        <Logo />
      </NavLink>
      {Object.entries(MENU).map(([sectionTitle, items]) => (
        <div key={sectionTitle} className="menu-section">
          <p className="menu-title">{sectionTitle}</p>
          <ul className="menu-list">
            {items.map((item) => {
              const Icon = item.icon
              return (
                <li key={item.path}>
                  {/* end seulement pour "/" : sinon "/" serait actif sur toutes les pages.
                      Sans end, "Équipes" reste actif sur /teams/2 */}
                  <NavLink to={item.path} end={item.path === '/'}>
                    <Icon size={18} />
                    {item.label}
                  </NavLink>
                </li>
              )
            })}
          </ul>
        </div>
      ))}
    </nav>
  )
}
