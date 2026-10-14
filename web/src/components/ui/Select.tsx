import { useId, type ComponentProps } from 'react'
import './ui.css'

type SelectOption = { value: string; label: string }

// Toutes les props d'un <select> HTML + label et options
type SelectProps = ComponentProps<'select'> & {
  label?: string
  options: SelectOption[]
}

export default function Select({ label, options, ...props }: SelectProps) {
  const id = useId()
  return (
    <div className="field">
      {label && <label htmlFor={id}>{label}</label>}
      <select id={id} className="input" {...props}>
        {options.map((option) => (
          <option key={option.value} value={option.value}>{option.label}</option>
        ))}
      </select>
    </div>
  )
}
