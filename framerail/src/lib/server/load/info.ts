import "$lib/vite-env.d.ts"

import defaults from "$lib/defaults"
import process from "process"

import { parseAcceptLangHeader } from "$lib/locales"
import { info } from "$lib/server/deepwell"
import { translate } from "$lib/server/deepwell/translate"
import { loadSiteInfo } from "$lib/server/load/site-info"

import type { TranslateKeys } from "$lib/types"

export async function loadInfo(request: Request) {
  // eslint-disable-next-line @typescript-eslint/no-unused-vars
  const { siteId } = loadSiteInfo(request.headers)
  const locales = parseAcceptLangHeader(request)

  if (!locales.includes(defaults.fallbackLocale)) locales.push(defaults.fallbackLocale)

  const response = await info()

  const translateKeys: TranslateKeys = {
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

  const internationalization = await translate(locales, translateKeys)

  return { ...viewData, internationalization }
}
