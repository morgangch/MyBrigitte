import { ChartColumn, CircleUser, History, LayoutDashboard, UserCog, Users } from 'lucide-react'
import { NavLink } from 'react-router'
import { Avatar, Logo } from '../components/ui'
import { ME } from '../lib/fakeData.ts'
import { ROLE_LABELS } from '../lib/labels.ts'
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
      <NavLink to="/" end className="sidebar-logo">
        <Logo variant="horizontalDark" />
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
      <NavLink to="/profile" className="sidebar-profile">
        <Avatar name={`${ME.first_name} ${ME.last_name}`} />
        <div className="sidebar-profile-text">
          <p className="sidebar-profile-name">{`${ME.first_name} ${ME.last_name}`}</p>
          <p className="sidebar-profile-role">
            {ROLE_LABELS[ME.role]}
            {ME.team && ` · ${ME.team.name}`}
          </p>
        </div>
      </NavLink>
    </nav>
  )
}
