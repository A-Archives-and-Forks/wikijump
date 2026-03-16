import defaults from "$lib/defaults"

import { parseAcceptLangHeader } from "$lib/locales"
import { authGetSession } from "$lib/server/auth/getSession"
import { getFileByHash } from "$lib/server/deepwell/file"
import { translate } from "$lib/server/deepwell/translate"
import { userEdit, userView } from "$lib/server/deepwell/user"
import { loadSiteInfo } from "$lib/server/load/site-info"
import { error, redirect } from "@sveltejs/kit"
import { fail, superValidate, withFiles } from "sveltekit-superforms"
import { valibot } from "sveltekit-superforms/adapters"
import { file, object, string } from "valibot"

import type { Viewer } from "$lib/server/deepwell/views"
import type { TranslateKeys, UserModel } from "$lib/types"
import type { Cookies, RequestEvent } from "@sveltejs/kit"

export async function loadUser(request: Request, cookies: Cookies, username?: string) {
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
        (locale) => !response.data.user_session?.user.locales.includes(locale)
      )
    ]
  }

  if (response.data?.site?.locale && !locales.includes(response.data.site.locale)) {
    locales.push(response.data.site.locale)
  }

  if (!locales.includes(defaults.fallbackLocale)) locales.push(defaults.fallbackLocale)

  let translateKeys: TranslateKeys = {
    ...defaults.translateKeys,
    "footer-license-unless": {
      license: response.data.license_name,
      "license_url": response.data.license_url
    }
  }

  let errorStatus = null

  switch (response.type) {
    case "user_found":
      break
    case "user_missing":
      errorStatus = 404
      break
    default:
      // Unexpected response type!
      // There is an inconsistency between here / DEEPWELL
      errorStatus = 500
  }

  // If the username is not the same as the slug, redirect to the slug
  if (
    errorStatus === null &&
    username &&
    response.type === "user_found" &&
    response.data.user.slug !== username
  ) {
    redirect(308, `/-/user/${response.data.user.slug}`)
  }

  const viewData: Partial<
    Viewer & {
      user: Partial<UserModel & { avatar: string }>
    }
  > = response.data

  if (errorStatus !== null && response.type === "user_missing") {
    translateKeys = {
      ...translateKeys,
      "user-not-exist": {},
      "user-not-logged-in": {}
    }
  } else if (errorStatus === null && response.type === "user_found") {
    const isViewingAnotherUser =
      response.data.user_session?.user?.user_id !== response.data.user.user_id

    viewData.user = sanitizeUserData(response.data.user, isViewingAnotherUser)

    // Get user avatar image
    if (response.data.user.avatar_s3_hash !== null) {
      const avatar = await getFileByHash(
        new Uint8Array(response.data.user.avatar_s3_hash)
      )
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

  const internationalization = await translate(locales, translateKeys)

  const userEditForm = await superValidate(request, valibot(userEditSchema))

  if (errorStatus !== null) {
    error(errorStatus, { ...viewData, view: response.type, internationalization })
  }

  return { ...viewData, view: response.type, internationalization, userEditForm }
}

function sanitizeUserData(
  user: UserModel,
  isViewingAnotherUser: boolean
): Partial<UserModel> {
  const baseSafeKeys = [
    "user_id",
    "user_type",
    "created_at",
    "updated_at",
    "deleted_at",
    "from_wikidot",
    "name",
    "slug",
    "avatar_s3_hash",
    "user_page"
  ]
  if (isViewingAnotherUser) {
    const safeKeys = [...baseSafeKeys]
    return Object.fromEntries(
      Object.entries(user).filter(([key]) => safeKeys.includes(key))
    )
  } else {
    const safeKeys = [
      ...baseSafeKeys,
      "name_changes_left",
      "last_name_change_added_at",
      "last_renamed_at",
      "email",
      "email_verified_at",
      "email_validation_info",
      "email_validation_at",
      "locales",
      "real_name",
      "gender",
      "birthday",
      "location",
      "biography"
    ]
    return Object.fromEntries(
      Object.entries(user).filter(([key]) => safeKeys.includes(key))
    )
  }
}

export async function userEditAction({
  request,
  cookies,
  getClientAddress
}: RequestEvent) {
  const form = await superValidate(request, valibot(userEditSchema))
  if (!form.valid) {
    return fail(400, { form })
  }

  const sessionToken = cookies.get("wikijump_token")
  const session = await authGetSession(sessionToken)

  const ipAddress = getClientAddress()

  try {
    const {
      name,
      realName,
      email,
      avatar,
      gender,
      birthday,
      location,
      biography,
      userPage,
      locales
    } = form.data

    const res = await userEdit(session?.user_id, ipAddress, {
      name,
      email,
      locales: locales
        .replaceAll("_", "-")
        .replaceAll(",", " ")
        .split(" ")
        .filter((v) => v.trim()),
      avatar,
      realName,
      gender,
      birthday,
      location,
      biography,
      userPage,
      bypassFilter: false
    })

    return withFiles({ form, res })
  } catch (error) {
    return fail(500, {
      form,
      message: error.message,
      code: error.code,
      data: error.data
    })
  }
}

export const userEditSchema = object({
  name: string(),
  realName: string(),
  email: string(),
  avatar: file(),
  gender: string(),
  birthday: string(),
  location: string(),
  userPage: string(),
  biography: string(),
  locales: string()
})
