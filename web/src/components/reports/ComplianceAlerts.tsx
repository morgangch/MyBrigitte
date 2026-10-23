import type { ComponentProps } from 'react'
import { Link } from 'react-router'
import { ChevronRight } from 'lucide-react'
import { Avatar, Badge, Card } from '../ui'
import { ALERT_TYPE_LABELS } from '../../lib/labels.ts'
import { formatDay } from '../../lib/time.ts'
import type { Alert, AlertType } from '../../types/api.ts'
import './ComplianceAlerts.css'

type BadgeTone = ComponentProps<typeof Badge>['tone']

// Rouge : à corriger (pointage faux ou limite légale) · orange : à surveiller
const ALERT_TYPE_TONES: Record<AlertType, BadgeTone> = {
  missing_clock_out: 'danger',
  overtime_exceeded: 'danger',
  absent: 'warning',
  late: 'warning',
}

type ComplianceAlertsProps = {
  alerts: Alert[]
  total: number
}

export default function ComplianceAlerts({ alerts, total }: ComplianceAlertsProps) {
  return (
    <Card
      title="Alertes de conformité"
      className="card-flush"
      action={
        <div className="row">
          <span className="report-hint">Code du travail : 48h par semaine</span>
          <Badge tone={total > 0 ? 'danger' : 'success'}>{total} à traiter</Badge>
        </div>
      }
    >
      {alerts.length === 0 ? (
        <p className="alerts-empty">Aucune alerte sur la période.</p>
      ) : (
        <ul>
          {alerts.map((alert) => {
            const fullName = `${alert.user.first_name} ${alert.user.last_name}`
            return (
              <li key={`${alert.type}-${alert.user.id}-${alert.date}`} className="alert-row">
                <Avatar name={fullName} />
                <div className="alert-row-text">
                  <p>
                    <span className="alert-row-name">{fullName}</span> · {alert.message}
                  </p>
                  <p className="alert-row-details">
                    {alert.team?.name ?? 'Sans équipe'} · {formatDay(`${alert.date}T12:00`)}
                  </p>
                </div>
                <Badge tone={ALERT_TYPE_TONES[alert.type]}>{ALERT_TYPE_LABELS[alert.type]}</Badge>
                <Link to={`/users/${alert.user.id}`} className="alert-row-action" aria-label={`Voir ${fullName}`}>
                  <ChevronRight size={18} />
                </Link>
              </li>
            )
          })}
        </ul>
      )}
    </Card>
  )
}
