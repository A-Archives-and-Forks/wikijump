import { authGetSession } from "$lib/server/auth/getSession"
import { authLogin } from "$lib/server/auth/login"

export async function POST(event) {
  console.log("POST HIT")
  const data = await event.request.formData()

  const userAgent = event.request.headers.get("User-Agent")
  const ipAddress = event.getClientAddress()

  const nameOrEmail = data.get("name-or-email")?.toString()
  const password = data.get("password")?.toString()

  try {
    const res = await authLogin(nameOrEmail, password, ipAddress, userAgent)

    if (res.session_token) {
      const session = await authGetSession(res.session_token)
      event.cookies.set("wikijump_token", res.session_token, {
        path: "/",
        httpOnly: true,
        secure: true,
        sameSite: "lax",
        expires: new Date(session.expires_at)
      })
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
