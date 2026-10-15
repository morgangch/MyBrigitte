# MyBrigitte : contrat d'API

Contrat **cible** (routes pas encore implémentées). À garder aligné avec [`web/src/types/api.ts`](../web/src/types/api.ts) et [`fakeData.ts`](../web/src/lib/fakeData.ts).

Principe : les **entités** (utilisateur, équipe, contrat, pointage) ne contiennent que des faits stables. Tout ce qui dépend d'une période (heures, retards, ponctualité) passe par **`/stats`**, avec un seul type `Stats` quel que soit le périmètre. La présence du jour passe par **`/presence`**.

## Conventions

- `uuid` : string · `datetime` : ISO 8601 UTC (`"2026-10-15T06:58:00.000Z"`) · `date` : `"YYYY-MM-DD"` · `time` : `"HH:MM:SS"` · `int` : entier · `float` : décimal
- Durées en **minutes**. Pourcentages en `float` de 0 à 100. Champs en `snake_case`. Jours en **Europe/Paris**. `?from=&to=` inclus.
- Auth : `Authorization: Bearer <token>` sur tout sauf `/auth/*`.
- `worked_minutes` = somme des sessions **fermées** (`end_time` non null). Une session ouverte n'est pas comptée : le front ajoute lui-même le temps en cours.
- Une session est attribuée au **jour de son début** (heure de Paris), même si elle se termine après minuit.
- L'`email` d'un utilisateur est l'adresse de son compte Microsoft. La 1re connexion se fait par Microsoft : le compte est relié par email, puis par `issuer` + `subject` (`oauth_identities`), uniquement pour le tenant de l'entreprise.

### Listes

- **Listes d'entités** (`/users`, `/teams`, `/…/work-sessions`, `/…/work-policies`, `/…/work-schedules`, `/alerts`) → `Page<T>`, paramètres `?limit=` (défaut 50, max 200) et `?offset=` (défaut 0).
- **Séries calculées** (`/…/schedule`, `/…/stats?group_by=…`, `/presence`) → tableau simple, un élément par jour / semaine / équipe / personne.

### Droits

- Un **employé** n'accède qu'à `/auth/*` et `/me/*`. Toute autre route → `403`.
- Un **manager** voit son équipe **et toutes les équipes en dessous** (son « périmètre »). Les listes sont filtrées automatiquement sur ce périmètre.
- Ressource hors périmètre (`/users/{id}`, `/teams/{id}`…) → `404`, pour ne pas révéler qu'elle existe.
- Les équipes racines (sans parent) sont créées par le seed, pas par l'API. Les managers d'une équipe racine voient donc toute l'entreprise.
- Tout utilisateur appartient à une équipe : création avec `team_id` obligatoire, et pas de retrait d'équipe sans déplacement vers une autre.

### Erreurs

```ts
ApiError = { error: { code: string, message: string, fields?: Record<string, string> } }
```

`400` invalide · `401` non connecté · `403` interdit · `404` introuvable · `409` conflit. `fields` donne l'erreur de chaque champ d'un formulaire.

| `code` | Statut | Quand |
|---|---|---|
| `validation_failed` | 400 | Un ou plusieurs champs invalides (détail dans `fields`) |
| `invalid_credentials` | 401 | Email ou mot de passe faux |
| `unauthenticated` | 401 | Token absent ou expiré |
| `forbidden` | 403 | Route réservée aux managers |
| `not_found` | 404 | Ressource inexistante ou hors périmètre |
| `email_taken` | 409 | Email déjà utilisé |
| `team_name_taken` | 409 | Nom d'équipe déjà utilisé |
| `session_already_open` | 409 | Pointage d'arrivée alors qu'une session est ouverte |
| `no_open_session` | 409 | Pointage de départ sans session ouverte |
| `session_overlap` | 409 | Correction qui chevauche une autre session |
| `team_not_empty` | 409 | Suppression d'une équipe qui a des membres ou des sous-équipes |
| `team_cycle` | 409 | Déplacement d'équipe qui créerait une boucle |
| `policy_overlap` | 409 | Contrat qui chevauche un autre contrat |

## Types communs

```ts
Role           = "employee" | "manager"
WorkPolicyMode = "fixed" | "flexible"
PresenceStatus = "present" | "late" | "away" | "not_arrived" | "absent" | "day_off"
AlertType      = "missing_clock_out" | "late" | "absent" | "overtime_exceeded"

Page<T> = { items: T[], total: int }

TeamRef = { id: uuid, name: string }
UserRef = { id: uuid, first_name: string, last_name: string }
```

## Routes

| Méthode | Route | Body | Réponse |
|---|---|---|---|
| **Auth** ||||
| POST | `/auth/login` | `{ email, password }` | `TokenResponse` + cookie refresh |
| POST | `/auth/refresh` | cookie | `TokenResponse` |
| POST | `/auth/logout` | cookie | `204` |
| GET | `/auth/microsoft` | — | `302` vers Microsoft |
| GET | `/auth/microsoft/callback` | — | `302` vers le front + cookie refresh |
| **Moi** ||||
| GET | `/me` | — | `Me` |
| PATCH | `/me` | `UpdateMe` | `Me` |
| POST | `/me/password` | `ChangePassword` | `204` |
| DELETE | `/me` | — | `204` (données personnelles effacées sous 30 jours) |
| POST | `/me/clock-in` | — | `201 WorkSession` · `409 session_already_open` |
| POST | `/me/clock-out` | — | `WorkSession` · `409 no_open_session` |
| GET | `/me/work-sessions?from&to` | — | `Page<WorkSession>` (ordre croissant) |
| GET | `/me/schedule?from&to` | — | `ScheduleDay[]` |
| GET | `/me/work-policy` | — | `WorkPolicy` (contrat en vigueur aujourd'hui) |
| GET | `/me/stats?from&to[&group_by]` | — | voir **Statistiques** |
| **Utilisateurs** (manager) ||||
| GET | `/users?team_id&role&is_active&q` | — | `Page<User>` (`q` cherche dans le nom et l'email) |
| POST | `/users` | `NewUser` | `201 User` · `409 email_taken` |
| GET | `/users/{id}` | — | `User` |
| PATCH | `/users/{id}` | `UpdateUser` | `User` |
| DELETE | `/users/{id}` | — | `204` (désactive : `is_active = false`) |
| GET | `/users/{id}/work-sessions?from&to` | — | `Page<WorkSession>` |
| POST | `/users/{id}/work-sessions` | `NewWorkSession` | `201 WorkSession` (pointage oublié) |
| PATCH | `/users/{id}/work-sessions/{sid}` | `CorrectWorkSession` | `WorkSession` |
| DELETE | `/users/{id}/work-sessions/{sid}?reason=` | — | `204` |
| GET | `/users/{id}/schedule?from&to` | — | `ScheduleDay[]` |
| GET | `/users/{id}/work-policies` | — | `Page<WorkPolicy>` (historique, plus récent d'abord) |
| POST | `/users/{id}/work-policies` | `NewWorkPolicy` | `201 WorkPolicy` (ferme le contrat précédent) |
| GET | `/users/{id}/work-schedules?from&to` | — | `Page<WorkSchedule>` |
| POST | `/users/{id}/work-schedules` | `NewWorkSchedule` | `201 WorkSchedule` |
| DELETE | `/users/{id}/work-schedules/{sid}` | — | `204` |
| GET | `/users/{id}/stats?from&to[&group_by]` | — | voir **Statistiques** |
| **Équipes** (manager) ||||
| GET | `/teams?q` | — | `Page<Team>` |
| POST | `/teams` | `NewTeam` | `201 Team` · `409 team_name_taken` |
| GET | `/teams/{id}` | — | `Team` |
| PATCH | `/teams/{id}` | `UpdateTeam` | `Team` · `409 team_cycle` |
| DELETE | `/teams/{id}` | — | `204` · `409 team_not_empty` |
| POST | `/teams/{id}/members` | `{ user_id }` | `204` (déplace l'utilisateur dans cette équipe) |
| GET | `/teams/{id}/stats?from&to[&group_by]` | — | voir **Statistiques** |
| **Présence, statistiques et rapports** (manager) ||||
| GET | `/presence?date&team_id` | — | `Presence[]` (`date` par défaut : aujourd'hui) |
| GET | `/stats?from&to&team_id[&group_by][&compare]` | — | voir **Statistiques** (tout le périmètre sans `team_id`) |
| GET | `/alerts?from&to&team_id` | — | `Page<Alert>` |
| GET | `/export/work-sessions?from&to&team_id&format=csv` | — | fichier CSV (`Content-Disposition: attachment`) |

Les membres d'une équipe se lisent avec `GET /users?team_id={id}`.

## Statistiques

Une seule forme, `Stats`, pour tous les périmètres : `/me/stats`, `/users/{id}/stats`, `/teams/{id}/stats`, `/stats`.

| Paramètre | Réponse |
|---|---|
| (aucun) | `Stats` |
| `group_by=day` | `{ date: date, stats: Stats }[]` |
| `group_by=week` | `{ week_start: date, stats: Stats }[]` (semaine du lundi au dimanche) |
| `group_by=team` | `{ team: TeamRef, stats: Stats }[]` (`/teams/{id}/stats` et `/stats` seulement) |
| `group_by=user` | `{ user: UserRef, stats: Stats }[]` (`/teams/{id}/stats` et `/stats` seulement) |
| `compare=previous` | `{ current: Stats, previous: Stats }` (période précédente de même durée, sans `group_by`) |

```ts
Stats = {
  worked_minutes: int,          // sessions fermées
  expected_minutes: int,        // prévu par les contrats et work_schedules
  overtime_minutes: int,        // somme, jour par jour, de max(0, travaillé - prévu)
  late_minutes: int,            // somme des retards au-delà de la tolérance
  expected_days: int,           // jours où du travail était prévu
  worked_days: int,             // jours avec au moins une session
  late_days: int,
  absent_days: int,             // jours prévus et passés, sans aucune session
  average_weekly_minutes: int | null,   // par personne et par semaine
  punctuality_rate: float | null,       // (worked_days - late_days) / worked_days
  presence_rate: float | null           // worked_days / expected_days
}  // null = rien à mesurer (aucun jour travaillé ou prévu)
```

Le retard n'existe que pour un contrat `fixed` : arrivée après `start_time + late_tolerance_minutes`.

## Structures

```ts
// Auth
TokenResponse = { access_token: string, token_type: "Bearer", expires_in: int }  // 900 s

// Utilisateurs
User = {
  id: uuid, first_name: string, last_name: string, email: string,
  phone_number: string | null, role: Role, team: TeamRef | null,
  is_active: boolean, created_at: datetime
}
Me = User & { has_password: boolean }  // password_hash IS NOT NULL

UpdateMe       = { first_name?: string, last_name?: string, phone_number?: string | null }  // pas d'email
ChangePassword = { current_password?: string, new_password: string }  // current_password obligatoire si has_password ; 12 caractères min.

NewUser = {
  first_name: string, last_name: string, email: string, phone_number: string | null,
  team_id: uuid, role: Role, work_policy: NewWorkPolicy
}  // users + team_members + user_work_policies (+ work_schedules) en une transaction
UpdateUser = {
  first_name?: string, last_name?: string, email?: string, phone_number?: string | null,
  role?: Role, team_id?: uuid, is_active?: boolean
}

// Présence du jour
Presence = { user: UserRef, team: TeamRef | null, status: PresenceStatus, first_clock_in: datetime | null }
// present     : session ouverte, arrivé à l'heure
// late        : session ouverte, arrivé après la tolérance
// away        : a travaillé aujourd'hui, aucune session ouverte (pause ou parti)
// not_arrived : travail prévu, aucune session, encore dans la tolérance (ou contrat flexible)
// absent      : travail prévu, aucune session, tolérance dépassée
// day_off     : aucun travail prévu, aucune session

// Pointages
WorkSession        = { id: uuid, start_time: datetime, end_time: datetime | null }  // null = en cours
NewWorkSession     = { start_time: datetime, end_time: datetime, reason: string }
CorrectWorkSession = { start_time?: datetime, end_time?: datetime | null, reason: string }
// Toute création, correction ou suppression par un manager écrit dans audit_log
// (event_type WORK_SESSION_CORRECTED, reason obligatoire).

// Planning
ScheduleDay = {
  date: date, start_time: time | null, end_time: time | null,  // null = rien de prévu à heure fixe
  break_minutes: int, expected_minutes: int                    // expected_minutes = 0 : jour non travaillé
}  // la règle work_schedules qui s'applique ce jour-là (la plus récente gagne)

WorkPolicy = {
  id: uuid, mode: WorkPolicyMode, expected_minutes_per_week: int | null,
  default_break_minutes: int, late_tolerance_minutes: int,
  effective_from: date, effective_until: date | null,  // fin exclue
  weekly_schedule: { day_of_week: int /* 1=lun…7=dim */, start_time: time | null,
                     end_time: time | null, expected_break_minutes: int }[]
}
NewWorkPolicy = {
  mode: WorkPolicyMode, expected_minutes_per_week: int | null,  // obligatoire si flexible
  default_break_minutes: int, late_tolerance_minutes: int, effective_from: date,
  weekday_start_time: time | null, weekday_end_time: time | null  // obligatoires si fixed ; lun → ven
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
NewWorkSchedule = Omit<WorkSchedule, "id" | "created_at">  // rattachée au contrat en vigueur à valid_from

// Équipes
Team = {
  id: uuid, name: string, description: string | null, parent: TeamRef | null,
  managers: UserRef[],        // membres avec role = manager
  member_count: int,
  member_preview: UserRef[]   // 5 premiers membres, pour les avatars
}
NewTeam    = { name: string, description: string | null, parent_team_id: uuid, member_ids: uuid[] }
// member_ids peut être vide ; chaque membre est déplacé depuis son équipe actuelle, en une transaction
UpdateTeam = { name?: string, description?: string | null, parent_team_id?: uuid }

// Alertes
Alert = { type: AlertType, user: UserRef, team: TeamRef | null, date: date, message: string }
// missing_clock_out : session encore ouverte le lendemain de son début
// late / absent     : mêmes règles que Presence, pour un jour passé
// overtime_exceeded : plus de 48 h travaillées sur une semaine (maximum légal)
```
