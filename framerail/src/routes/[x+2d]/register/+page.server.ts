import { loadRegisterPage } from "$lib/server/load/register"

export async function load({ request, cookies }) {
  return loadRegisterPage(request, cookies)
}
