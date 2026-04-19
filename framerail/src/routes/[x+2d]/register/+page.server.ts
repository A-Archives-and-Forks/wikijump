import { loadRegisterPage, registerAction } from "$lib/server/load/register"

export async function load({ request, cookies, parent }) {
  return loadRegisterPage(request, cookies, parent)
}

export const actions = { default: registerAction }
