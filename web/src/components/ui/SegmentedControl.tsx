import './SegmentedControl.css'

type SegmentedOption = { value: string; label: string }

type SegmentedControlProps = {
  label: string
  name: string
  options: SegmentedOption[]
  defaultValue?: string
  hint?: string
  onChange?: (value: string) => void
}

export default function SegmentedControl({ label, name, options, defaultValue, hint, onChange }: SegmentedControlProps) {
  return (
    <fieldset className="field segmented-field">
      <legend>{label}</legend>
      <div className="segmented">
        {options.map((option) => (
          <label key={option.value} className="segmented-option">
            <input
              type="radio"
              name={name}
              value={option.value}
              defaultChecked={option.value === defaultValue}
              onChange={() => onChange?.(option.value)}
            />
            {option.label}
          </label>
        ))}
      </div>
      {hint && <p className="segmented-hint">{hint}</p>}
    </fieldset>
  )
}
