import './ui.css'

// Nombre de couleurs définies dans styles/theme.css (--avatar-1 à --avatar-6)
const AVATAR_COLOR_COUNT = 6

// size : "small" | "medium" | "large"
export default function Avatar({ name, size = 'medium' }) {
  const initials = name
    .split(' ')
    .map((part) => part[0])
    .join('')
    .slice(0, 2)
    .toUpperCase()

  const avatarColorNumber = (initials.charCodeAt(0) + initials.charCodeAt(1)) % AVATAR_COLOR_COUNT + 1;

  return (
    <span className={`avatar avatar-${size} avatar-color-${avatarColorNumber}`} aria-label={name} role="img">
      {initials}
    </span>
  )
}
