import { useId } from 'react'
import './ui.css'

// options : [{ value: "rh", label: "RH" }, ...]
export default function Select({ label, options, ...props }) {
  const id = useId()
  return (
    <div className="field">
      <label htmlFor={id}>{label}</label>
      <select id={id} className="input" {...props}>
        {options.map((option) => (
          <option key={option.value} value={option.value}>{option.label}</option>
        ))}
      </select>
    </div>
  )
}
