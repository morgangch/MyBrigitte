import { useEffect, useId, useRef, type MouseEvent, type ReactNode, type SyntheticEvent } from 'react'
import { X } from 'lucide-react'
import './Drawer.css'

const SLIDE_IN: Keyframe[] = [{ transform: 'translateX(100%)' }, { transform: 'translateX(0)' }]
const FADE_IN: Keyframe[] = [{ opacity: 0 }, { opacity: 1 }]
const ANIMATION: KeyframeAnimationOptions = { duration: 250, easing: 'ease' }

type DrawerProps = {
  open: boolean
  title: string
  description?: string
  onClose: () => void
  children: ReactNode
}

export default function Drawer({ open, title, description, onClose, children }: DrawerProps) {
  const dialogRef = useRef<HTMLDialogElement>(null)
  const titleId = useId()

  useEffect(() => {
    const dialog = dialogRef.current
    if (!dialog) return

    if (open && !dialog.open) {
      dialog.showModal()
      dialog.animate(SLIDE_IN, ANIMATION)
      dialog.animate(FADE_IN, { ...ANIMATION, pseudoElement: '::backdrop' })
    }

    if (!open && dialog.open) {
      const exitOptions: KeyframeAnimationOptions = { ...ANIMATION, direction: 'reverse', fill: 'forwards' }
      const slideOut = dialog.animate(SLIDE_IN, exitOptions)
      const fadeOut = dialog.animate(FADE_IN, { ...exitOptions, pseudoElement: '::backdrop' })
      slideOut.finished
        .then(() => {
          dialog.close()
          slideOut.cancel()
          fadeOut.cancel()
        })
        .catch(() => {})
      return () => {
        slideOut.cancel()
        fadeOut.cancel()
      }
    }
  }, [open])

  const handleCancel = (event: SyntheticEvent<HTMLDialogElement>) => {
    event.preventDefault()
    onClose()
  }

  const handleBackdropClick = (event: MouseEvent<HTMLDialogElement>) => {
    if (event.target === event.currentTarget) onClose()
  }

  return (
    <dialog
      ref={dialogRef}
      className="drawer"
      aria-labelledby={titleId}
      onCancel={handleCancel}
      onClose={onClose}
      onClick={handleBackdropClick}
    >
      <header className="drawer-header">
        <div>
          <h2 id={titleId}>{title}</h2>
          {description && <p className="text-secondary">{description}</p>}
        </div>
        <button type="button" className="drawer-close" aria-label="Fermer" onClick={onClose}>
          <X size={18} />
        </button>
      </header>
      {children}
    </dialog>
  )
}
