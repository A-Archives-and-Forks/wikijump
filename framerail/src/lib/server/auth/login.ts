import { client } from "$lib/server/deepwell"

export async function authLogin(
  nameOrEmail: string,
  password: string,
  ipAddress: string,
  userAgent: string
): Promise<{
  session_token: string
  needs_mfa: boolean
}> {
  return client.request("login", {
    name_or_email: nameOrEmail,
    password,
    ip_address: ipAddress,
    user_agent: userAgent
  })
}
