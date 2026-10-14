import type { ReactNode } from 'react'
import './ui.css'

type BadgeProps = {
  tone?: 'neutral' | 'primary' | 'success' | 'warning' | 'danger'
  children: ReactNode
}

export default function Badge({ tone = 'neutral', children }: BadgeProps) {
  return <span className={`badge badge-${tone}`}>{children}</span>
}
