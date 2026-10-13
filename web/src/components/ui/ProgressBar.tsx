import './ui.css'

type ProgressBarProps = {
  label: string
  value: number
  max?: number
}

export default function ProgressBar({ label, value, max = 100 }: ProgressBarProps) {
  return <progress className="progress" aria-label={label} value={value} max={max} />
}
