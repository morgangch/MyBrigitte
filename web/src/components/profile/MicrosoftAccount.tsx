import { Badge } from '../ui'
import microsoftLogo from '../../assets/microsoft.svg'
import './MicrosoftAccount.css'

type MicrosoftAccountProps = {
  email: string
}

export default function MicrosoftAccount({ email }: MicrosoftAccountProps) {
  return (
    <div className="microsoft-account">
      <img src={microsoftLogo} alt="" className="microsoft-account-logo" />
      <div className="microsoft-account-text">
        <p className="microsoft-account-title">Compte Microsoft connecté</p>
        <p className="microsoft-account-email">{email}</p>
      </div>
      <Badge tone="success">Actif</Badge>
    </div>
  )
}
