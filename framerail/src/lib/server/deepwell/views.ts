import { client } from "$lib/server/deepwell"
import type { Optional } from "$lib/types"

export interface PageRoute {
  slug: string
  extra: string
}

export async function pageView(
  siteId: number,
  locales: string[],
  route: Optional<PageRoute>,
  sessionToken: Optional<string>
): Promise<object> {
  return client.request("page_view", {
    site_id: siteId,
    locales,
    session_token: sessionToken,
    route
  })
}

export async function adminView(
  siteId: number,
  locales: string[],
  sessionToken: Optional<string>
): Promise<object> {
  return client.request("admin_view", {
    site_id: siteId,
    locales,
    session_token: sessionToken
  })
}
