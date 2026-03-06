import { userCreate } from "$lib/server/deepwell/user.js"
import { UserType } from "$lib/types.js"

export async function POST(event) {
  const data = await event.request.formData()

  const userAgent = event.request.headers.get("User-Agent")
  const ipAddress = event.getClientAddress()

  const username = data.get("username")?.toString()
  const email = data.get("email")?.toString()
  const password = data.get("password")?.toString()

  try {
    if (!username) throw new Error("Username is required")
    if (!email) throw new Error("Email is required")
    if (!password) throw new Error("Password is required")

    const res = await userCreate(
      UserType.Regular,
      username,
      email,
      ["en"],
      password,
      ipAddress
    )

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
