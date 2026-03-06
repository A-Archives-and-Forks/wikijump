import { client } from "$lib/server/deepwell"
import { startBlobUpload, uploadToPresignUrl } from "./file"

import type { Optional, UserType } from "$lib/types"

export async function userView(
  siteId: number,
  locales: string[],
  sessionToken: Optional<string>,
  username?: string
): Promise<object> {
  return client.request("user_view", {
    site_id: siteId,
    session_token: sessionToken,
    locales,
    user: username
  })
}

export async function userEdit(
  userId: number,
  userIpAddr: string,
  params: Record<string, any>
): Promise<object> {
  const data: Record<string, any> = {}
  if (params.name !== undefined && typeof params.name === "string") {
    data.name = params.name
  }
  if (params.email !== undefined && typeof params.email === "string") {
    data.email = params.email
  }
  if (params.real_name !== undefined && typeof params.real_name === "string") {
    if (params.real_name) data.real_name = params.real_name
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
  if (params.user_page !== undefined && typeof params.user_page === "string") {
    if (params.user_page) data.user_page = params.user_page
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

export async function userCreate(
  userType: UserType,
  name: string,
  email: string,
  locales: string[],
  password: string,
  ipAddress: string,
  bypassFilter = false,
  bypassEmailVerification = false
): Promise<{
  user_id: number
  slug: string
}> {
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
