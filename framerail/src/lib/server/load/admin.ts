import defaults from "$lib/defaults"
import { parseAcceptLangHeader } from "$lib/locales"
import { translate } from "$lib/server/deepwell/translate"
import { adminView } from "$lib/server/deepwell/views"
import { loadSiteInfo } from "$lib/server/load/site-info"
import type { TranslateKeys } from "$lib/types"
import { error } from "@sveltejs/kit"

export async function loadAdminPage(request, cookies) {
  const { siteId } = loadSiteInfo(request.headers)
  const sessionToken = cookies.get("wikijump_token")
  let locales = parseAcceptLangHeader(request)

  const response = await adminView(
    siteId,
    [...locales, defaults.fallbackLocale],
    sessionToken
  )

  if (response.data?.user_session?.user?.locales) {
    locales = [
      ...response.data.user_session.user.locales,
      ...locales.filter(
        (locale) => !response.data.user_session.user.locales.includes(locale)
      )
    ]
  }

  if (response.data?.site?.locale && !locales.includes(response.data.site.locale)) {
    locales.push(response.data.site.locale)
  }

  if (!locales.includes(defaults.fallbackLocale)) locales.push(defaults.fallbackLocale)

  let translateKeys: TranslateKeys = {
    ...defaults.translateKeys
  }

  const viewData = response.data
  viewData.view = response.type

  translateKeys["footer-license-unless"] = {
    license: viewData.license_name,
    "license_url": viewData.license_url
  }

  let errorStatus = null

  switch (response.type) {
    case "site_found":
      break
    case "admin_permissions":
      errorStatus = 401
      break
    case "site_missing":
      errorStatus = 404
      break
    default:
      // Unexpected response type!
      // There is an inconsistency between here / DEEPWELL
      errorStatus = 500
  }

  if (errorStatus !== null) {
    translateKeys = {
      ...translateKeys,
      "site-not-exist": {}
    }
  } else {
    translateKeys = {
      ...translateKeys,

      // Edit actions
      "edit": {},
      "save": {},
      "cancel": {},

      // Site info attributes
      "site-info.name": {},
      "site-info.slug": {},
      "site-info.tagline": {},
      "site-info.description": {},
      "site-info.default-page": {},
      "site-info.locale": {},
      "site-info.layout": {},
      "wiki-page-layout.default": {},
      "wiki-page-layout.wikidot": {},
      "wiki-page-layout.wikijump": {}
    }
  }

  const translated = await translate(locales, translateKeys)

  viewData.internationalization = translated

  if (errorStatus !== null) {
    error(errorStatus, viewData)
  }

  return viewData
}
