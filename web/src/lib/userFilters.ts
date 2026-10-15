import type { PresenceStatus, User } from '../types/api.ts'

export type UserFilters = {
  search: string
  teamId: string
  status: string
}

export const EMPTY_FILTERS: UserFilters = { search: '', teamId: '', status: '' }

export function hasActiveFilters(filters: UserFilters) {
  return filters.search !== '' || filters.teamId !== '' || filters.status !== ''
}

function matchesSearch(user: User, search: string) {
  const text = `${user.first_name} ${user.last_name} ${user.email}`.toLowerCase()
  return text.includes(search.trim().toLowerCase())
}

export function filterUsers(users: User[], filters: UserFilters, presenceByUser: Map<string, PresenceStatus>) {
  return users.filter(
    (user) =>
      matchesSearch(user, filters.search) &&
      (filters.teamId === '' || user.team?.id === filters.teamId) &&
      (filters.status === '' || presenceByUser.get(user.id) === filters.status),
  )
}
