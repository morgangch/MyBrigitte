# MyBrigitte : contrat d'API

Contrat **cible** (routes pas encore implémentées). À garder aligné avec [`web/src/types/api.ts`](../web/src/types/api.ts) et [`fakeData.ts`](../web/src/lib/fakeData.ts).

## Conventions

- `uuid` : string · `datetime` : ISO 8601 UTC (`"2026-10-15T06:58:00.000Z"`) · `date` : `"YYYY-MM-DD"` · `time` : `"HH:MM:SS"` · `int` : entier · `float` : décimal
- Durées en **minutes**. Champs en `snake_case`. Jours en **Europe/Paris**. `?from=&to=` inclus.
- Auth : `Authorization: Bearer <token>` sur tout sauf `/auth/*`. Routes manager → `403` pour un employé.
- Erreurs : `{ error: string }` avec `400` invalide · `401` non connecté · `403` interdit · `404` introuvable · `409` conflit.
- `worked_minutes` = somme des sessions **fermées** (`end_time` non null). Une session ouverte n'est pas comptée : le front ajoute lui-même le temps en cours.
- Une session est attribuée au **jour de son début** (heure de Paris), même si elle se termine après minuit.

```ts
Role           = "employee" | "manager"
WorkPolicyMode = "fixed" | "flexible"
TodayStatus    = "present" | "late" | "not_arrived" | "absent"
AlertType      = "missing_clock_out" | "late" | "absent" | "overtime_exceeded"  // à valider

Team = { id: uuid, name: string }
```

## Routes

| Méthode | Route | Body | Réponse |
|---|---|---|---|
| **Auth** ||||
| POST | `/auth/login` | `{ email, password }` | `TokenResponse` + cookie refresh |
| POST | `/auth/refresh` | cookie | `TokenResponse` |
| POST | `/auth/logout` | cookie | `204` |
| GET | `/auth/microsoft` · `/callback` | — | `302` |
| **Moi** ||||
| GET | `/me` | — | `Me` |
| PATCH | `/me` | `{ first_name?, last_name?, phone_number? }` | `Me` |
| GET | `/me/work-sessions?from&to` | — | `WorkSession[]` (ordre croissant) |
| POST | `/me/clock-in` | — | `201 WorkSession` · `409` si déjà ouverte |
| POST | `/me/clock-out` | — | `WorkSession` · `409` si rien d'ouvert |
| GET | `/me/schedule?from&to` | — | `ScheduleDay[]` (1 par jour) |
| GET | `/me/summary?from&to` | — | `Summary` |
| GET | `/me/summary?from&to&group_by=day` | — | `DailySummary[]` |
| **Utilisateurs** (manager) ||||
| GET | `/users?team_id&role&q` | — | `User[]` |
| POST | `/users` | `NewUser` | `201 User` · `409` email pris |
| GET / PATCH | `/users/{id}` | `UpdateUser` | `User` |
| DELETE | `/users/{id}` | — | `204` (désactive, ne supprime pas) |
| GET | `/users/{id}/work-sessions` · `/summary` | — | comme `/me` |
| GET / PUT | `/users/{id}/work-policy` | `NewWorkPolicy` | `WorkPolicy` |
| GET / POST | `/users/{id}/work-schedules?from&to` | `NewWorkSchedule` | `WorkSchedule[]` / `201 WorkSchedule` |
| DELETE | `/users/{id}/work-schedules/{sid}` | — | `204` |
| **Équipes** (manager) ||||
| GET / POST | `/teams` | `NewTeam` | `TeamDetail[]` / `201` |
| GET / PATCH / DELETE | `/teams/{id}` | `Partial<NewTeam>` | `TeamDetail` / `204` · `409` si cycle ou non vide |
| POST | `/teams/{id}/members` | `{ user_id }` | `201` |
| DELETE | `/teams/{id}/members/{userId}` | — | `204` |
| GET | `/teams/{id}/summary?from&to` | — | `TeamSummary` |
| **Rapports** (manager) — `?from&to&team_id` (sans `team_id` = tout) ||||
| GET | `/reports/summary` | — | `ReportSummary` |
| GET | `/reports/punctuality-by-team` | — | `TeamPunctuality[]` |
| GET | `/reports/weekly-hours` | — | `WeeklyHours[]` |
| GET | `/reports/alerts` | — | `Alert[]` |
| GET | `/reports/export?format=csv` | — | fichier CSV (`Content-Disposition: attachment`) |

## Structures

```ts
// Auth
TokenResponse = { access_token: string, token_type: "Bearer", expires_in: int }  // 900 s

// Moi
Me = {
  id: uuid, first_name: string, last_name: string, email: string,
  phone_number: string | null,
  has_password: boolean,  // password_hash IS NOT NULL
  role: Role, team: Team | null
}  // email = adresse du compte Microsoft (liaison à la 1re connexion)
WorkSession = { id: uuid, start_time: datetime, end_time: datetime | null }  // null = en cours
ScheduleDay = {
  date: date, start_time: time | null, end_time: time | null,  // null = rien de prévu
  break_minutes: int | null, expected_minutes: int
}  // la règle work_schedules qui s'applique ce jour-là (la plus récente gagne)
Summary = {
  from: date, to: date, worked_minutes: int, expected_minutes: int,
  overtime_minutes: int,  // somme des heures sup. de chaque jour
  worked_days: int, late_days: int, late_tolerance_minutes: int
}
DailySummary = {
  date: date, expected_minutes: int, worked_minutes: int,
  overtime_minutes: int, late_minutes: int
}

// Utilisateurs
User = {
  id: uuid, first_name: string, last_name: string, email: string,
  phone_number: string | null, role: Role, team: Team | null, today_status: TodayStatus
}
NewUser = {
  first_name: string, last_name: string, email: string, phone_number: string | null,
  team_id: uuid | null, role: Role, work_policy: NewWorkPolicy
}
UpdateUser = Partial<Omit<NewUser, "work_policy">> & { is_active?: boolean }

WorkPolicy = {
  id: uuid, mode: WorkPolicyMode, expected_minutes_per_week: int | null,
  default_break_minutes: int, late_tolerance_minutes: int,
  effective_from: date, effective_until: date | null,  // fin exclue
  weekly_schedule: { day_of_week: int /* 1=lun…7=dim */, start_time: time | null,
                     end_time: time | null, expected_break_minutes: int }[]
}
NewWorkPolicy = {
  mode: WorkPolicyMode, expected_minutes_per_week: int | null,
  default_break_minutes: int, late_tolerance_minutes: int, effective_from: date,
  weekday_start_time: time | null, weekday_end_time: time | null  // lun → ven
}
// Une ligne de work_schedules = un jour de la semaine sur une période.
// Exception (congé, horaires d'un jour) = nouvelle ligne pour ce jour, plus récente :
// valid_from = la date, valid_until = le lendemain, horaires null = pas de travail.
WorkSchedule = {
  id: uuid, day_of_week: int /* 1=lun…7=dim */,
  valid_from: date, valid_until: date | null,  // fin exclue
  start_time: time | null, end_time: time | null,  // null = pas de travail
  expected_break_minutes: int, created_at: datetime
}
NewWorkSchedule = Omit<WorkSchedule, "id" | "created_at">  // rattachée à la policy en vigueur à valid_from

// Équipes
TeamDetail = { id: uuid, name: string, description: string | null, parent_team_id: uuid | null, member_count: int }
NewTeam    = { name: string, description: string | null, parent_team_id: uuid | null }
TeamSummary = {
  team: Team, from: date, to: date, member_count: int,
  worked_minutes: int, expected_minutes: int, overtime_minutes: int,
  punctuality_rate: float, presence_rate: float  // en %
}

// Rapports
Kpi = { value: float, previous_value: float | null, delta: float | null }  // vs période précédente
ReportSummary = {
  from: date, to: date, punctuality_rate: Kpi, average_weekly_minutes: Kpi,
  overtime_minutes: Kpi, presence_rate: Kpi
}
TeamPunctuality = { team: Team, punctuality_rate: float }
WeeklyHours = { year: int, week: int, week_start: date, average_worked_minutes: int, average_overtime_minutes: int }
Alert = { type: AlertType, user: { id: uuid, first_name: string, last_name: string }, team: Team | null, date: date, message: string }
```
