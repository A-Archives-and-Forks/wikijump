import { authGetSession } from "$lib/server/auth/getSession"
import { siteUpdate } from "$lib/server/deepwell/admin.js"

// Handling of server events from client

export async function POST(event) {
  const data = await event.request.formData()

  const sessionToken = event.cookies.get("wikijump_token")
  const ipAddr = event.getClientAddress()
  const userAgent = event.cookies.get("User-Agent")

  const session = await authGetSession(sessionToken)

  const action = data.get("action")?.toString().toLowerCase()

  const siteIdVal = data.get("site-id")?.toString()
  const siteId = siteIdVal ? parseInt(siteIdVal) : null

  let res: object = {}

  try {
    if (action === "edit") {
      /** Edit site settings. */
      const name = data.get("name")?.toString()
      const slug = data.get("slug")?.toString()
      const tagline = data.get("tagline")?.toString()
      const description = data.get("description")?.toString()
      const defaultPage = data.get("default-page")?.toString()
      const locale = data.get("locale")?.toString()
      const layout = data.get("layout")?.toString().trim()

      res = await siteUpdate(
        siteId,
        session?.user_id,
        ipAddr,
        name,
        slug,
        tagline,
        description,
        defaultPage,
        locale,
        layout
      )
    }

    return new Response(JSON.stringify(res))
  } catch (error) {
    return new Response(
      JSON.stringify({
        message: error.message,
        code: error.code,
        data: error.data
      })
    )
  }
}
