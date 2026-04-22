import { loadPreload } from "$lib/server/load/preload"

export async function load({ request, cookies }) {
  return loadPreload(request, cookies)
}
