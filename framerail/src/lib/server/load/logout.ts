import defaults from "$lib/defaults"
import { parseAcceptLangHeader } from "$lib/locales"
import { translate } from "$lib/server/deepwell/translate"
import { loadSiteInfo } from "$lib/server/load/site-info"
import type { TranslateKeys } from "$lib/types"

export async function loadLogoutPage(request, cookies) {
  // Set up parameters
  const { siteId } = loadSiteInfo(request.headers)
  const sessionToken = cookies.get("wikijump_token")
  let locales = parseAcceptLangHeader(request)

  let viewData: Record<string, any> = {
    isLoggedIn: Boolean(sessionToken)
  }

  if (!locales.includes(defaults.fallbackLocale)) locales.push(defaults.fallbackLocale)

  const translateKeys: TranslateKeys = {
    ...defaults.translateKeys,

    // Page actions
    "cancel": {},
    "logout": {},

    // misc
    "logout.toast": {}
  }

  const translated = await translate(locales, translateKeys)

  viewData.internationalization = translated

  // Return to page for rendering
  return viewData
}
