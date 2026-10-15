import { Badge, Button } from '../ui'
import microsoftLogo from '../../assets/microsoft.svg'
import './MicrosoftAccount.css'

type MicrosoftAccountProps = {
  email: string | null
}

export default function MicrosoftAccount({ email }: MicrosoftAccountProps) {
  return (
    <div className="microsoft-account">
      <img src={microsoftLogo} alt="" className="microsoft-account-logo" />
      <div className="microsoft-account-text">
        <p className="microsoft-account-title">
          {email ? 'Compte Microsoft connecté' : 'Compte Microsoft non connecté'}
        </p>
        <p className="microsoft-account-email">
          {email ?? 'Connectez-vous sans mot de passe avec votre compte professionnel.'}
        </p>
      </div>
      {email ? <Badge tone="success">Actif</Badge> : <Button>Connecter</Button>}
    </div>
  )
}
