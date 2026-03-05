import defaults from "$lib/defaults"

import { parseAcceptLangHeader } from "$lib/locales"
import { translate } from "$lib/server/deepwell/translate"
import { loadSiteInfo } from "$lib/server/load/site-info"

import type { TranslateKeys } from "$lib/types"
import type { Cookies } from "@sveltejs/kit"

export async function loadRegisterPage(request: Request, cookies: Cookies) {
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
    "register": {},

    // misc
    "username": {},
    "username.placeholder": {},
    "username.info": {},
    "email": {},
    "email.placeholder": {},
    "email.info": {},
    "password": {},
    "password.placeholder": {},
    "confirm-password": {},
    "register.toast": {},
    "create-account": {},

    // errors
    "error-form.password-mismatch": {}
  }

  const translated = await translate(locales, translateKeys)

  viewData.internationalization = translated

  // Return to page for rendering
  return viewData
}
