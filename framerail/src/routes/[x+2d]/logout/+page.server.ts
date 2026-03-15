import { loadLogoutPage, logoutAction } from "$lib/server/load/logout"

export async function load({ request, cookies }) {
  return loadLogoutPage(request, cookies)
}

export const actions = { logout: logoutAction }
