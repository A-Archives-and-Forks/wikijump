import defaults from "$lib/defaults"
import { parseAcceptLangHeader } from "$lib/locales"
import { info } from "$lib/server/deepwell"
import { translate } from "$lib/server/deepwell/translate"
import { loadSiteInfo } from "$lib/server/load/site-info"
import type { TranslateKeys } from "$lib/types"
import "$lib/vite-env.d.ts"
import process from "process"

export async function loadInfo(request, cookies) {
  const { siteId } = loadSiteInfo(request.headers)
  const sessionToken = cookies.get("wikijump_token")
  let locales = parseAcceptLangHeader(request)

  if (!locales.includes(defaults.fallbackLocale)) locales.push(defaults.fallbackLocale)

  const response = await info()

  let translateKeys: TranslateKeys = {
    ...defaults.translateKeys
  }

  const viewData = {
    backend: response,
    frontend: {
      name: serverInfo.frontendName,
      description: serverInfo.frontendDescription,
      repository: serverInfo.frontendRepository,
      version: serverInfo.frontendVersion,
      license: serverInfo.frontendLicense,
      node: process.versions.node,
      pnpm: serverInfo.pnpmVersion
    }
  }

  const translated = await translate(locales, translateKeys)

  viewData.internationalization = translated

  return viewData
}
