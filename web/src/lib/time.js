// "2026-10-05T08:05:00" -> "Lundi 5 octobre"
// Avec relative = true : "Aujourd'hui" ou "Hier"
export function formatDay(date, relative = false) {
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
export function elapsedSeconds(start, now) {
  return Math.max(0, Math.floor((now - new Date(start)) / 1000))
}

// 12003 -> { hours: 3, minutes: 20, seconds: 3 }
export function splitDuration(totalSeconds) {
  const hours = Math.floor(totalSeconds / 3600)
  const minutes = Math.floor((totalSeconds % 3600) / 60)
  const seconds = totalSeconds % 60
  return { hours, minutes, seconds }
}

// "2026-10-09T06:52:00.000Z" -> "08:52"
export function formatTime(date) {
  return new Date(date).toLocaleTimeString('fr-FR', { hour: '2-digit', minute: '2-digit' })
}

// 25200 -> "7h00"
export function formatDuration(totalSeconds) {
  const { hours, minutes } = splitDuration(totalSeconds)
  return `${hours}h${minutes.toString().padStart(2, '0')}`
}

// "2026-10-09T06:52:00.000Z" -> "2026-10-09"
export function toDateKey(date) {
  const year = date.getFullYear().toString().padStart(2, '0')
  const month = (date.getMonth() + 1).toString().padStart(2, '0')
  const day = date.getDate().toString().padStart(2, '0')
  return year + '-' + month + '-' + day
}
