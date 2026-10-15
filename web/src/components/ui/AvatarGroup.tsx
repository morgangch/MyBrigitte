import Avatar from './Avatar.tsx'
import './AvatarGroup.css'

type AvatarGroupPerson = { id: string; name: string }

type AvatarGroupProps = {
  people: AvatarGroupPerson[]
  total: number
  max?: number
}

export default function AvatarGroup({ people, total, max = 4 }: AvatarGroupProps) {
  const visiblePeople = people.slice(0, max)
  const hiddenCount = total - visiblePeople.length

  return (
    <div className="avatar-group">
      <ul className="avatar-group-list">
        {visiblePeople.map((person) => (
          <li key={person.id} className="avatar-group-item">
            <Avatar name={person.name} size="small" />
          </li>
        ))}
      </ul>
      {hiddenCount > 0 && <span className="avatar-group-more">+{hiddenCount}</span>}
    </div>
  )
}
