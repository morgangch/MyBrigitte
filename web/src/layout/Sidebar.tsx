import { ChartColumn, CircleUser, History, LayoutDashboard, LogOut, UserCog, Users } from 'lucide-react'
import { NavLink, useNavigate } from 'react-router'
import { Avatar, Logo } from '../components/ui'
import { ME } from '../lib/fakeData.ts'
import { ROLE_LABELS } from '../lib/labels.ts'
import './Sidebar.css'

const MENU = [
  {
    title: 'Mon espace',
    managerOnly: false,
    items: [
      { label: 'Tableau de bord', path: '/', icon: LayoutDashboard },
      { label: 'Mon historique', path: '/history', icon: History },
      { label: 'Mon profil', path: '/profile', icon: CircleUser },
    ],
  },
  {
    title: 'Management',
    managerOnly: true,
    items: [
      { label: 'Utilisateurs', path: '/users', icon: UserCog },
      { label: 'Équipes', path: '/teams', icon: Users },
      { label: 'Rapports', path: '/reports', icon: ChartColumn },
    ],
  },
]

const VISIBLE_MENU = MENU.filter((section) => !section.managerOnly || ME.role === 'manager')

export default function Sidebar() {
  const navigate = useNavigate()

  // TODO(auth-api) : POST /auth/logout (révoque le refresh token), puis oublier l'access token
  function handleLogout() {
    navigate('/login')
  }

  return (
    <nav className="sidebar" aria-label="Navigation principale">
      <NavLink to="/" end className="sidebar-logo">
        <Logo variant="horizontalDark" />
      </NavLink>
      {VISIBLE_MENU.map((section) => (
        <div key={section.title} className="menu-section">
          <p className="menu-title">{section.title}</p>
          <ul className="menu-list">
            {section.items.map((item) => {
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
      {/* Le bouton est à côté du lien, pas dedans : un <button> ne peut pas être dans un <a> */}
      <div className="sidebar-profile">
        <NavLink to="/profile" className="sidebar-profile-link">
          <Avatar name={`${ME.first_name} ${ME.last_name}`} />
          <div className="sidebar-profile-text">
            <p className="sidebar-profile-name">{`${ME.first_name} ${ME.last_name}`}</p>
            <p className="sidebar-profile-role">
              {ROLE_LABELS[ME.role]}
              {ME.team && ` · ${ME.team.name}`}
            </p>
          </div>
        </NavLink>
        <button type="button" className="sidebar-logout" aria-label="Se déconnecter" title="Se déconnecter" onClick={handleLogout}>
          <LogOut size={18} />
        </button>
      </div>
    </nav>
  )
}
