import { loadLoginPage, loginAction } from "$lib/server/load/login"

export async function load({ request, cookies }) {
  return loadLoginPage(request, cookies)
}

export const actions = { default: loginAction }
