import { useId, type ReactNode } from 'react'
import './ProfileSection.css'

type ProfileSectionProps = {
  title: string
  description: string
  children: ReactNode
}

export default function ProfileSection({ title, description, children }: ProfileSectionProps) {
  const titleId = useId()

  return (
    <section className="profile-section" aria-labelledby={titleId}>
      <div className="profile-section-heading">
        <h2 id={titleId}>{title}</h2>
        <p className="profile-section-description">{description}</p>
      </div>
      <div className="profile-section-content">{children}</div>
    </section>
  )
}
