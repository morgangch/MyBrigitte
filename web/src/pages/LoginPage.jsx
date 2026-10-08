import Logo from '../components/ui/Logo.jsx'
import Input from '../components/ui/Input.jsx'
import Button from '../components/ui/Button.jsx'
import { Mail, Lock } from 'lucide-react'
import { useNavigate } from 'react-router'
import microsoftLogo from '../assets/microsoft.svg'
import { useState } from 'react'

export default function LoginPage() {
  const [errorMessage, setErrorMessage] = useState(null)
  const navigate = useNavigate()

  const handleSubmit = (event) => {
    event.preventDefault()
    setErrorMessage(null)
    const formData = new FormData(event.currentTarget)
    const email = formData.get('email').trim()
    const password = formData.get('password')
    if (!email || !password) {
      setErrorMessage('Veuillez remplir tous les champs.')
      return
    }
    // TODO: appeler l'API de connexion puis rediriger
    navigate('/')
  }

  const handleMicrosoftLogin = () => {
    setErrorMessage(null)
    // TODO: implémenter la connexion avec Microsoft
    setErrorMessage('Connexion avec Microsoft non implémentée.')
  }

  return (
    <main className="login-page">
      <form className="login-form" onSubmit={handleSubmit}>
        <Logo size="large" />
        <div className="page-heading">
          <h1>Bon retour parmi nous!</h1>
          <p className="page-subtitle">Connectez-vous pour pointer et suivre votre temps.</p>
        </div>

        <Button onClick={handleMicrosoftLogin}>
          <img src={microsoftLogo} alt="" className="button-icon" />
          Continuer avec Microsoft
        </Button>

        <Input label="Adresse e-mail" type="email" name="email" placeholder="Entrez votre adresse e-mail" icon={Mail} required />
        <Input label="Mot de passe" type="password" name="password" placeholder="Entrez votre mot de passe" icon={Lock} required />

        {errorMessage && <p className="field-error" role="alert">{errorMessage}</p>}

        <Button type="submit" variant="primary">Se connecter</Button>
        <p className="page-subtitle text-center">Vous n'avez pas de compte ? Demandez-en un à votre manager.</p>

      </form>
    </main>
  )
}