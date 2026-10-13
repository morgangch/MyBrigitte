import type { ReactNode } from 'react'
import './ui.css'

type CardProps = {
  title?: string
  // élément optionnel à droite du titre (lien, bouton…)
  action?: ReactNode
  className?: string
  children: ReactNode
}

export default function Card({ title, action, className = '', children }: CardProps) {
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
