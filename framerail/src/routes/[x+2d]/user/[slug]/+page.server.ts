import { loadUser } from "$lib/server/load/user"

export async function load({ params, request, cookies, parent }) {
  return loadUser(request, cookies, parent, params.slug)
}
