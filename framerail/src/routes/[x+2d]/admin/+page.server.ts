import { adminAction, loadAdminPage } from "$lib/server/load/admin"

export async function load({ request, cookies, parent }) {
  return loadAdminPage(request, cookies, parent)
}

export const actions = { default: adminAction }
