import './ui.css'

export default function ProgressBar({ label, value, max = 100 }) {
  return <progress className="progress" aria-label={label} value={value} max={max} />
}
