import type { ReactNode } from 'react'

type PageLayoutProps = {
  title: string
  subtitle?: string
  actions?: ReactNode
  children?: ReactNode
}

export default function PageLayout({ title, subtitle, actions, children }: PageLayoutProps) {
  return (
    <div className="page-layout">
      <header className="page-header">
        <div className="page-heading">
          <h1>{title}</h1>
          {subtitle && <p className="page-subtitle">{subtitle}</p>}
        </div>
        {actions && <div className="row">{actions}</div>}
      </header>
      {children}
    </div>
  )
}
