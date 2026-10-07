import Avatar from './components/ui/Avatar.jsx'
import Badge from './components/ui/Badge.jsx'
import Button from './components/ui/Button.jsx'
import Card from './components/ui/Card.jsx'
import Input from './components/ui/Input.jsx'
import Logo from './components/ui/Logo.jsx'
import ProgressBar from './components/ui/ProgressBar.jsx'
import Select from './components/ui/Select.jsx'
import Stat from './components/ui/Stat.jsx'

const TEAMS = [
  { value: 'design', label: 'Design' },
  { value: 'rh', label: 'RH' },
]

export default function App() {
  return (
    <main className="page stack">
      <Logo />

      <Card title="Boutons">
        <div className="row">
          <Button variant="primary">Pointer mon arrivée</Button>
          <Button>Annuler</Button>
          <Button variant="danger">Supprimer</Button>
        </div>
      </Card>

      <Card title="Formulaire">
        <div className="stack">
          <Input label="Email" type="email" placeholder="prenom.nom@entreprise.com" />
          <Input label="Mot de passe" type="password" error="Mot de passe incorrect" />
          <Select label="Équipe" options={TEAMS} />
        </div>
      </Card>

      <Card title="Badges et avatars">
        <div className="stack">
          <div className="row">
            <Badge>Neutre</Badge>
            <Badge tone="success">En poste</Badge>
            <Badge tone="warning">En retard</Badge>
            <Badge tone="danger">Absent</Badge>
          </div>
          <div className="row">
            <Avatar name="Léa Martin" size="small" />
            <Avatar name="Hugo Bernard" />
            <Avatar name="Sarah Petit" size="large" />
          </div>
        </div>
      </Card>

      <Card title="Statistiques">
        <div className="stack">
          <Stat label="Cette semaine" value="13h31" unit="/ 35h" detail="39 % de l'objectif" />
          <ProgressBar label="Progression de la semaine" value={39} />
        </div>
      </Card>
    </main>
  )
}
