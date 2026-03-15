import { client } from "$lib/server/deepwell"
import { startBlobUpload, uploadToPresignUrl } from "./file"

import type { Maybe, Optional, UserModel, UserType } from "$lib/types"
import type { Viewer } from "./views"

/* ----- User View ----- */
interface UserViewFound {
  type: "user_found"
  data: Viewer & {
    user: UserModel
  }
}
interface UserViewMissing {
  type: "user_missing"
  data: Viewer
}
export async function userView(
  siteId: number,
  locales: string[],
  sessionToken: Optional<string>,
  username?: string
): Promise<UserViewFound | UserViewMissing> {
  return client.request("user_view", {
    site_id: siteId,
    session_token: sessionToken,
    locales,
    user: username
  })
}

/* ----- User Edit ----- */
interface UserEditParams {
  name?: Maybe<string>
  email?: Maybe<string>
  emailVerified?: Maybe<boolean>
  password?: Maybe<string>
  locales?: Maybe<string[]>
  avatar?: Maybe<File>
  realName?: Maybe<Optional<string>>
  gender?: Maybe<Optional<string>>
  birthday?: Maybe<Optional<string>>
  location?: Maybe<Optional<string>>
  biography?: Maybe<Optional<string>>
  userPage?: Maybe<Optional<string>>
  bypassFilter?: boolean
}
export async function userEdit(
  userId: number,
  userIpAddr: string,
  params: UserEditParams
): Promise<UserModel> {
  const data: Record<string, any> = {}
  if (params.name !== undefined && typeof params.name === "string") {
    data.name = params.name
  }
  if (params.email !== undefined && typeof params.email === "string") {
    data.email = params.email
  }
  if (params.realName !== undefined && typeof params.realName === "string") {
    if (params.realName) data.real_name = params.realName
    else data.real_name = null
  }
  if (params.gender !== undefined && typeof params.gender === "string") {
    if (params.gender) data.gender = params.gender
    else data.gender = null
  }
  if (params.birthday !== undefined && typeof params.birthday === "string") {
    if (isNaN(Date.parse(params.birthday))) data.birthday = null
    else data.birthday = params.birthday
  }
  if (params.location !== undefined && typeof params.location === "string") {
    if (params.location) data.location = params.location
    else data.location = null
  }
  if (params.biography !== undefined && typeof params.biography === "string") {
    if (params.biography) data.biography = params.biography
    else data.biography = null
  }
  if (params.userPage !== undefined && typeof params.userPage === "string") {
    if (params.userPage) data.user_page = params.userPage
    else data.user_page = null
  }
  if (
    Array.isArray(params.locales) &&
    params.locales.every((v) => typeof v === "string")
  ) {
    data.locales = params.locales
  }
  if (params.avatar instanceof File && params.avatar.type.startsWith("image/")) {
    const presign = await startBlobUpload(userId, params.avatar.size)
    await uploadToPresignUrl(presign.presign_url, params.avatar)
    data.avatar_uploaded_blob_id = presign.pending_blob_id
  } else if (params.avatar !== undefined && params.avatar === null) data.avatar = null

  return client.request("user_edit", {
    user: userId,
    ip_address: userIpAddr,
    ...data
  })
}

/* ----- User Create ----- */
interface UserCreate {
  user_id: number
  slug: string
}
export async function userCreate(
  userType: UserType,
  name: string,
  email: string,
  locales: string[],
  password: string,
  ipAddress: string,
  bypassFilter = false,
  bypassEmailVerification = false
): Promise<UserCreate> {
  return client.request("user_create", {
    user_type: userType,
    name,
    email,
    locales,
    password,
    ip_address: ipAddress,
    bypass_filter: bypassFilter,
    bypass_email_verification: bypassEmailVerification
  })
}
