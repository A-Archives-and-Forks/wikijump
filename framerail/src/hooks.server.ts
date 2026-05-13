// Hook that runs on every request, including form actions.

import { storeRequestContext } from "$lib/server/load/request-ctx"
import { loadSiteInfo } from "$lib/server/load/site-info"
import type { Handle } from "@sveltejs/kit"

export const handle: Handle = async ({ event, resolve }) => {
  const { request, cookies, locals, params } = event

  // Gather common request metadata into a shared context.
  const { siteId } = loadSiteInfo(request.headers)
  const page_slug = params.slug
  const sessionToken = cookies.get("wikijump_token")

  storeRequestContext(locals, sessionToken, siteId, page_slug)

  // Continue processing the request
  const response = await resolve(event)
  return response
}
