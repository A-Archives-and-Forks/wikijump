import { loadLogoutPage, logoutAction } from "$lib/server/load/logout"

export async function load({ request, cookies, parent }) {
  return loadLogoutPage(request, cookies, parent)
}

export const actions = { logout: logoutAction }
