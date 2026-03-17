import { client } from "$lib/server/deepwell"

export async function authLogout(sessionToken: string): Promise<void> {
  return client.request("logout", [sessionToken])
}
