import horizontal from '../../assets/logo-horizontal.svg'
import horizontalWhite from '../../assets/logo-horizontal-white.svg'
import horizontalDark from '../../assets/logo-horizontal-dark.svg'
import symbol from '../../assets/logo-symbol.svg'
import stacked from '../../assets/logo-stacked.svg'
import './ui.css'

const LOGOS = { horizontal, horizontalWhite, horizontalDark, symbol, stacked }

type LogoProps = {
  // horizontalDark = texte noir
  variant?: keyof typeof LOGOS
  size?: 'small' | 'medium' | 'large'
}

export default function Logo({ variant = 'horizontal', size = 'medium' }: LogoProps) {
  return <img className={`logo logo-${size}`} src={LOGOS[variant]} alt="MyBrigitte" />
}
