import { loadLoginPage, loginAction } from "$lib/server/load/login"

export async function load({ request, cookies, parent }) {
  return loadLoginPage(request, cookies, parent)
}

export const actions = { default: loginAction }
