import Logo from '../components/ui/Logo.jsx'
import Input from '../components/ui/Input.jsx'
import Button from '../components/ui/Button.jsx'
import { Mail, Lock } from 'lucide-react'
import { useNavigate } from 'react-router'
import microsoftLogo from '../assets/microsoft.svg'

export default function LoginPage() {
  const navigate = useNavigate()

  const handleSubmit = (event) => {
    event.preventDefault()
    // TODO: appeler l'API de connexion puis rediriger
    navigate("/")
  }

  return (
    <main className="login-page">
      <form className="login-form" onSubmit={handleSubmit}>
        <Logo size="large" />
        <div className="page-heading">
          <h1>Bon retour parmi nous!</h1>
          <p className="page-subtitle">Connectez-vous pour pointer et suivre votre temps.</p>
        </div>

        <Button>
          <img src={microsoftLogo} alt="" className="button-icon" />
          Continuer avec Microsoft
        </Button>

        <Input label="Adresse e-mail" type="email" name="email" placeholder="Entrez votre adresse e-mail" icon={Mail} />
        <Input label="Mot de passe" type="password" name="password" placeholder="Entrez votre mot de passe" icon={Lock} />

        <Button type="submit" variant="primary">Se connecter</Button>
        <p className="page-subtitle" style={{ textAlign: 'center' }}>
          Vous n'avez pas de compte ? Demandez-en un à votre manager.
        </p>
      </form>
    </main>
  )
}