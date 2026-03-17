import { loadUser, userEditAction } from "$lib/server/load/user"

export async function load({ request, cookies }) {
  return loadUser(request, cookies)
}

export const actions = {
  userEdit: userEditAction
}
