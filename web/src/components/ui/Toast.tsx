import { useEffect, useRef } from 'react'
import { CircleAlert, CircleCheck } from 'lucide-react'
import './Toast.css'

const TOAST_DURATION_MS = 4000
const SLIDE_UP: Keyframe[] = [
  { opacity: 0, transform: 'translateY(calc(100% + 24px))' },
  { opacity: 1, transform: 'translateY(0)' },
]
const ANIMATION: KeyframeAnimationOptions = { duration: 250, easing: 'ease' }

type ToastProps = {
  message: string
  tone: 'success' | 'danger'
  onClose: () => void
}

export default function Toast({ message, tone, onClose }: ToastProps) {
  const toastRef = useRef<HTMLDivElement>(null)

  useEffect(() => {
    const toast = toastRef.current
    if (!toast) return

    toast.showPopover()
    toast.animate(SLIDE_UP, ANIMATION)

    let slideDown: Animation | null = null
    const timer = setTimeout(() => {
      slideDown = toast.animate(SLIDE_UP, { ...ANIMATION, direction: 'reverse', fill: 'forwards' })
      slideDown.finished.then(onClose).catch(() => {})
    }, TOAST_DURATION_MS)

    return () => {
      clearTimeout(timer)
      slideDown?.cancel()
    }
  }, [onClose])

  const Icon = tone === 'success' ? CircleCheck : CircleAlert

  return (
    <div ref={toastRef} popover="manual" role="status" className={`toast toast-${tone}`}>
      <Icon size={18} />
      {message}
    </div>
  )
}
