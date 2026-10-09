import './ui.css'

// variant : "primary" | "secondary" | "danger"
export default function Button({ variant = 'secondary', className = '', children, ...props }) {
  return (
    <button type="button" className={`button button-${variant} ${className}`} {...props}>
      {children}
    </button>
  )
}
