import { loadUser, userEditAction } from "$lib/server/load/user"

export async function load({ request, cookies, parent }) {
  return loadUser(request, cookies, parent)
}

export const actions = {
  userEdit: userEditAction
}
