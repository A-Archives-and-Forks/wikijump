import defaults from "$lib/defaults"
import { parseAcceptLangHeader } from "$lib/locales"
import { getFileByHash } from "$lib/server/deepwell/file"
import { translate } from "$lib/server/deepwell/translate"
import { userView } from "$lib/server/deepwell/user"
import { loadSiteInfo } from "$lib/server/load/site-info"
import type { TranslateKeys } from "$lib/types"
import { error, redirect } from "@sveltejs/kit"

export async function loadUser(username?: string, request, cookies) {
  const { siteId } = loadSiteInfo(request.headers)
  const sessionToken = cookies.get("wikijump_token")
  let locales = parseAcceptLangHeader(request)

  const response = await userView(
    siteId,
    [...locales, defaults.fallbackLocale],
    sessionToken,
    username
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
    case "user_found":
      break
    case "user_missing":
      viewData.user = null
      errorStatus = 404
      break
    default:
      // Unexpected response type!
      // There is an inconsistency between here / DEEPWELL
      errorStatus = 500
  }

  if (errorStatus === null && username && viewData.user.slug !== username) {
    redirect(308, `/-/user/${viewData.user.slug}`)
  }

  if (errorStatus !== null) {
    translateKeys = {
      ...translateKeys,
      "user-not-exist": {},
      "user-not-logged-in": {}
    }
  } else {
    // Remove sensitive information
    let sensitiveKeys = ["password", "multi_factor_secret", "multi_factor_recovery_codes"]
    if (viewData.user_session?.user?.user_id !== viewData.user.user_id) {
      // Currently viewing another user's profile
      sensitiveKeys = [...sensitiveKeys, "email", "email_is_alias", "email_verified_at"]
    }
    for (let i = 0; i < sensitiveKeys.length; i++) {
      delete viewData.user[sensitiveKeys[i]]
    }

    // Get user avatar image
    if (viewData.user.avatar_s3_hash !== null) {
      const avatar = await getFileByHash(new Uint8Array(viewData.user.avatar_s3_hash))
      const dataurl = `data:${avatar.type};base64,${Buffer.from(
        await avatar.arrayBuffer()
      ).toString("base64")}`
      viewData.user.avatar = dataurl
    }

    translateKeys = {
      ...translateKeys,

      // Edit actions
      "edit": {},
      "save": {},
      "cancel": {},

      // User profile attributes
      "avatar": {},
      "user-profile-info.name": {},
      "user-profile-info.real-name": {},
      "user-profile-info.email": {},
      "user-profile-info.avatar": {},
      "user-profile-info.gender": {},
      "user-profile-info.birthday": {},
      "user-profile-info.location": {},
      "user-profile-info.biography": {},
      "user-profile-info.user-page": {},
      "user-profile-info.locales": {}
    }
  }

  const translated = await translate(locales, translateKeys)

  viewData.internationalization = translated

  if (errorStatus !== null) {
    error(errorStatus, viewData)
  }

  return viewData
}
