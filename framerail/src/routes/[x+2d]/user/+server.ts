import { authGetSession } from "$lib/server/auth/getSession"
import { userEdit } from "$lib/server/deepwell/user"

export async function POST(event) {
  const data = await event.request.formData()
  const sessionToken = event.cookies.get("wikijump_token")

  try {
    const session = await authGetSession(sessionToken)

    const name = data.get("name")?.toString().trim()
    const email = data.get("email")?.toString().trim()
    const realName = data.get("real-name")?.toString().trim()
    const gender = data.get("gender")?.toString().trim()
    const birthday = data.get("birthday")?.toString().trim()
    const location = data.get("location")?.toString().trim()
    const biography = data.get("biography")?.toString().trim()
    const userPage = data.get("user-page")?.toString().trim()
    const locales = data.get("locales")?.toString().trim()
    const avatar = data.get("avatar")?.valueOf()

    const body: Record<string, any> = {
      name,
      email,
      real_name: realName,
      birthday,
      gender,
      location,
      biography,
      user_page: userPage,
      locales: locales
        ?.replaceAll("_", "-")
        .replaceAll(",", " ")
        .split(" ")
        .filter((v) => v.trim()),
      avatar
    }

    const res = await userEdit(session?.user_id, event.getClientAddress(), body)

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
