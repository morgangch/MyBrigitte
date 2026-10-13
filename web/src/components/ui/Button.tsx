import type { ComponentProps } from 'react'
import './ui.css'

// Toutes les props d'un <button> HTML (onClick, type, disabled…) + variant
type ButtonProps = ComponentProps<'button'> & {
  variant?: 'primary' | 'secondary' | 'danger'
}

export default function Button({ variant = 'secondary', className = '', children, ...props }: ButtonProps) {
  return (
    <button type="button" className={`button button-${variant} ${className}`} {...props}>
      {children}
    </button>
  )
}
