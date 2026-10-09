import './ui.css'

// action : élément optionnel à droite du titre (lien, bouton…)
export default function Card({ title, action, className = '', children }) {
  return (
    <section className="card">
      {title && (
        <header className="card-header">
          <h2>{title}</h2>
          {action}
        </header>
      )}
      <div className={`card-body ${className}`}>{children}</div>
    </section>
  )
}
