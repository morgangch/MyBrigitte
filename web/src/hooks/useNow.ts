import { useEffect, useState } from 'react'

// L'heure actuelle, mise à jour toutes les `intervalMs` millisecondes.
// Seul le composant qui appelle ce hook se re-rend à chaque tic.
export default function useNow(intervalMs = 1000) {
  const [now, setNow] = useState(() => new Date())

  useEffect(() => {
    const interval = setInterval(() => setNow(new Date()), intervalMs)
    return () => clearInterval(interval)
  }, [intervalMs])

  return now
}
