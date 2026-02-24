import defaults from "$lib/defaults"
import { parseAcceptLangHeader } from "$lib/locales"
import { translate } from "$lib/server/deepwell/translate"
import { loadSiteInfo } from "$lib/server/load/site-info"
import type { TranslateKeys } from "$lib/types"

export async function loadLoginPage(request, cookies) {
  // Set up parameters
  const { siteId } = loadSiteInfo(request.headers)
  const sessionToken = cookies.get("wikijump_token")
  const locales = parseAcceptLangHeader(request)

  const viewData: Record<string, any> = {
    isLoggedIn: Boolean(sessionToken)
  }

  if (!locales.includes(defaults.fallbackLocale)) locales.push(defaults.fallbackLocale)

  const translateKeys: TranslateKeys = {
    ...defaults.translateKeys,

    // Page actions
    "cancel": {},
    "login": {},

    // misc
    "specifier": {},
    "password": {},
    "login.toast": {},
    "forgot-password": {},
    "remember-me": {},
    "create-account": {}
  }

  const translated = await translate(locales, translateKeys)

  viewData.internationalization = translated

  // Return to page for rendering
  return viewData
}
