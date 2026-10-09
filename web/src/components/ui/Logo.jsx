import horizontal from '../../assets/logo-horizontal.svg'
import horizontalWhite from '../../assets/logo-horizontal-white.svg'
import horizontalDark from '../../assets/logo-horizontal-dark.svg'
import symbol from '../../assets/logo-symbol.svg'
import stacked from '../../assets/logo-stacked.svg'
import './ui.css'

const LOGOS = { horizontal, horizontalWhite, horizontalDark, symbol, stacked }

// variant : "horizontal" | "horizontalWhite" | "horizontalDark" (texte noir) | "symbol" | "stacked"
// size : "small" | "medium" | "large"
export default function Logo({ variant = 'horizontal', size = 'medium' }) {
  return <img className={`logo logo-${size}`} src={LOGOS[variant]} alt="MyBrigitte" />
}
