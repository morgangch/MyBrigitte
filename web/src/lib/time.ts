// "2026-10-05T08:05:00" -> "Lundi 5 octobre"
// Avec relative = true : "Aujourd'hui" ou "Hier"
export function formatDay(date: string | Date, relative = false) {
  if (relative) {
    const now = new Date()
    const yesterday = new Date(now)
    yesterday.setDate(yesterday.getDate() - 1)

    if (toDateKey(new Date(date)) === toDateKey(now)) return "Aujourd'hui"
    if (toDateKey(new Date(date)) === toDateKey(yesterday)) return 'Hier'
  }

  const text = new Date(date).toLocaleDateString('fr-FR', { weekday: 'long', day: 'numeric', month: 'long' })
  return text.charAt(0).toUpperCase() + text.slice(1)
}

// Secondes écoulées entre `start` et `now`
export function elapsedSeconds(start: string, now: Date) {
  return Math.max(0, Math.floor((now.getTime() - new Date(start).getTime()) / 1000))
}

// 12003 -> { hours: 3, minutes: 20, seconds: 3 }
export function splitDuration(totalSeconds: number) {
  const hours = Math.floor(totalSeconds / 3600)
  const minutes = Math.floor((totalSeconds % 3600) / 60)
  const seconds = totalSeconds % 60
  return { hours, minutes, seconds }
}

// "2026-10-09T06:52:00.000Z" -> "08:52"
export function formatTime(date: string | Date) {
  return new Date(date).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' })
}

// 25200 -> "7h00"
export function formatDuration(totalSeconds: number) {
  const { hours, minutes } = splitDuration(totalSeconds)
  return `${hours}h${minutes.toString().padStart(2, '0')}`
}

// "2026-10-09T06:52:00.000Z" -> "2026-10-09"
export function toDateKey(date: Date) {
  const year = date.getFullYear().toString().padStart(2, '0')
  const month = (date.getMonth() + 1).toString().padStart(2, '0')
  const day = date.getDate().toString().padStart(2, '0')
  return year + '-' + month + '-' + day
}

// Lundi de la semaine de `date`, à minuit (getDay() : 0 = dimanche, 1 = lundi…)
export function startOfWeek(date: Date) {
  const monday = new Date(date)
  const daysSinceMonday = (monday.getDay() + 6) % 7
  monday.setDate(monday.getDate() - daysSinceMonday)
  monday.setHours(0, 0, 0, 0)
  return monday
}

// `date` décalée de `days` jours (négatif pour reculer), sans modifier l'originale
export function addDays(date: Date, days: number) {
  const result = new Date(date)
  result.setDate(result.getDate() + days)
  return result
}

// 2026-10-12 -> "12 oct."
export function formatShortDate(date: Date) {
  return date.toLocaleDateString('fr-FR', { day: 'numeric', month: 'short' })
}

export function timeToMinutes(time: string) {
  const [hours, minutes] = time.split(':').map(Number)
  return hours * 60 + minutes
}
