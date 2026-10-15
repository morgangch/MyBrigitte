import { useId, type ComponentProps } from 'react'
import type { LucideIcon } from 'lucide-react'
import './ui.css'

// Toutes les props d'un <input> HTML (type, name, required…) + label, error et icône
type InputProps = ComponentProps<'input'> & {
  label?: string
  error?: string | null
  icon?: LucideIcon | null
}

export default function Input({ label, error, icon: Icon = null, ...props }: InputProps) {
  const id = useId()
  return (
    <div className="field">
      {label && <label htmlFor={id}>{label}</label>}
      <div className="input-wrapper">
        {Icon && <Icon className="input-icon" size={16} />}
        <input id={id} className="input-control" aria-invalid={Boolean(error)} {...props} />
      </div>
      {error && <p className="field-error" role="alert">{error}</p>}
    </div>
  )
}
