import { Link } from 'react-router'

export default function NotFoundPage() {
  return (
    <main className="login-page">
      <div className="login-form">
        <h1>Page introuvable</h1>
        <p className="page-subtitle">Cette adresse n'existe pas ou a été déplacée.</p>
        <Link to="/">Revenir au tableau de bord</Link>
      </div>
    </main>
  )
}
