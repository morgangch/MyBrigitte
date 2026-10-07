import './ui.css'

// variant : "primary" | "danger"
export default function Button({ variant = 'secondary', children, ...props }) {
  return (
    <button type="button" className={`button button-${variant}`} {...props}>
      {children}
    </button>
  )
}
