import { loadRegisterPage, registerAction } from "$lib/server/load/register"

export async function load({ request, cookies }) {
  return loadRegisterPage(request, cookies)
}

export const actions = { default: registerAction }
