import { useId } from 'react'
import './ui.css'

export default function Input({ label, error, icon: Icon = null, ...props }) {
  const id = useId()
  return (
    <div className="field">
      <label htmlFor={id}>{label}</label>
      <div className="input-wrapper">
        {Icon && <Icon className="input-icon" size={16} />}
        <input id={id} className="input-control" aria-invalid={Boolean(error)} {...props} />
      </div>
      {error && <p className="field-error" role="alert">{error}</p>}
    </div>
  )
}