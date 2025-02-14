// Helper to extract site information headers from wws
//
// These headers are set by wws - their values can be trusted.
// If the headers are first set by the client, those values
// get erased.

const SITE_ID_HEADER = "x-wikijump-site-id"
const SITE_SLUG_HEADER = "x-wikijump-site-slug"

export interface SiteInfo {
  siteId: number
  siteSlug: string
}

function getHeader(headers: Headers, key: string): string {
  const value = headers.get(key)
  if (value === null) {
    throw new Error(`Missing wws internal header '${key}'`)
  }

  return value
}

export function loadSiteInfo(headers: Headers): SiteInfo {
  const siteSlug = getHeader(headers, SITE_SLUG_HEADER)
  const siteId = parseInt(getHeader(headers, SITE_ID_HEADER))
  return { domain, siteId, siteSlug }
}
