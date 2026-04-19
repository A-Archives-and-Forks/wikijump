import { loadPage } from "$lib/server/load/page"
import { actions as pageActions } from "./[slug]/[...extra]/+page.server"

export async function load({ request, cookies, parent }) {
  return loadPage(undefined, undefined, request, cookies, parent)
}

export const actions = pageActions
