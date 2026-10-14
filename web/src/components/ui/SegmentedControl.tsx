import './SegmentedControl.css'

type SegmentedOption = { value: string; label: string }

type SegmentedControlProps = {
  label: string
  name: string
  options: SegmentedOption[]
  defaultValue?: string
  hint?: string
}

export default function SegmentedControl({ label, name, options, defaultValue, hint }: SegmentedControlProps) {
  return (
    <fieldset className="field segmented-field">
      <legend>{label}</legend>
      <div className="segmented">
        {options.map((option) => (
          <label key={option.value} className="segmented-option">
            <input type="radio" name={name} value={option.value} defaultChecked={option.value === defaultValue} />
            {option.label}
          </label>
        ))}
      </div>
      {hint && <p className="segmented-hint">{hint}</p>}
    </fieldset>
  )
}
