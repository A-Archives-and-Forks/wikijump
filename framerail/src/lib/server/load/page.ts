import defaults from "$lib/defaults"
import { parseAcceptLangHeader } from "$lib/locales"
import { translate } from "$lib/server/deepwell/translate"
import { pageView } from "$lib/server/deepwell/views"
import { loadSiteInfo } from "$lib/server/load/site-info"
import type { Optional, TranslateKeys } from "$lib/types"
import { error, redirect } from "@sveltejs/kit"

// TODO form single deepwell request that does all the relevant prep stuff here

export async function loadPage(
  slug: Optional<string>,
  extra: Optional<string>,
  request,
  cookies
) {
  // Set up parameters
  const { siteId } = loadSiteInfo(request.headers)
  const route = slug || extra ? { slug, extra } : null
  const sessionToken = cookies.get("wikijump_token")
  let locales = parseAcceptLangHeader(request)

  // Request data from backend
  // Includes fallback locale in case there is no Accept-Language header
  const response = await pageView(
    siteId,
    [...locales, defaults.fallbackLocale],
    route,
    sessionToken
  )

  if (response.data?.user_session?.user?.locales) {
    locales = [
      ...response.data.user_session.user.locales,
      ...locales.filter(
        (locale) => !response.data.user_session.user.locales.includes(locale)
      )
    ]
  }

  if (response.data?.site?.locale && !locales.includes(response.data.site.locale)) {
    locales.push(response.data.site.locale)
  }

  if (!locales.includes(defaults.fallbackLocale)) locales.push(defaults.fallbackLocale)

  // Process response, performing redirects etc
  const viewData = response.data
  viewData.view = response.type

  let checkRedirect = true
  let errorStatus = null

  switch (response.type) {
    case "found":
      break
    case "missing":
      viewData.page = null
      viewData.page_revision = null
      errorStatus = 404
      break
    case "permissions":
      errorStatus = 403
      break
    default:
      // Unexpected response type!
      // There is an inconsistency between here / DEEPWELL
      errorStatus = 500
  }

  let translateKeys: TranslateKeys = {
    ...defaults.translateKeys,

    // Page actions
    "save": {},
    "cancel": {},

    // Page edit
    "title": {},
    "alt-title": {},
    "tags": {},
    "wiki-page-revision-comments": {},
    "wiki-page-layout": {},
    "wiki-page-layout.default": {},
    "wiki-page-layout.wikidot": {},
    "wiki-page-layout.wikijump": {}
  }

  translateKeys["footer-license-unless"] = {
    license: viewData.license_name,
    "license_url": viewData.license_url
  }

  if (errorStatus === null) {
    // Calculate difference of days since latest page edit
    let updatedAt = Date.parse(viewData.page.updated_at ?? viewData.page.created_at)
    let daysDiff = Math.floor((Date.now() - updatedAt) / 1000 / 86400)

    translateKeys = {
      ...translateKeys,

      // Page actions
      "edit": {},
      "delete": {},
      "history": {},
      "move": {},
      "view": {},
      "vote": {},
      "layout": {},
      "parents": {},
      "options": {},
      "confirm": {},

      // Page history
      "wiki-page-revision": {
        revision: viewData.page_revision.revision_number
      },
      "wiki-page-last-edit": {
        date: new Date(updatedAt).toLocaleString(locales),
        days: daysDiff
      },
      "wiki-page-revision-history": {},
      "wiki-page-revision-number": {},
      "wiki-page-revision-created-at": {},
      "wiki-page-revision-user": {},
      "wiki-page-revision-rollback": {},
      "wiki-page-revision-type": {},
      "wiki-page-revision-type.create": {},
      "wiki-page-revision-type.regular": {},
      "wiki-page-revision-type.move": {},
      "wiki-page-revision-type.delete": {},
      "wiki-page-revision-type.rollback": {},
      "wiki-page-revision-type.undelete": {},
      "wiki-page-revision-type.undo": {},

      // Page vote
      "wiki-page-vote": {},
      "wiki-page-vote.list": {},
      "wiki-page-vote.set": {},
      "wiki-page-vote.remove": {},
      "wiki-page-vote.score": {},

      // Page files
      "files": {},
      "upload": {},
      "restore": {},
      "wiki-page-file": {},
      "wiki-page-file-no-files": {},
      "wiki-page-file-upload.select": {},
      "wiki-page-file-upload.name": {},
      "wiki-page-file.name": {},
      "wiki-page-file.created-at": {},
      "wiki-page-file.updated-at": {},
      "wiki-page-file.mime": {},
      "wiki-page-file.size": {},
      "wiki-page-file.page": {},
      "wiki-page-file-move-destination-page": {},
      "wiki-page-file-revision-type": {},
      "wiki-page-file-revision-type.create": {},
      "wiki-page-file-revision-type.regular": {},
      "wiki-page-file-revision-type.move": {},
      "wiki-page-file-revision-type.delete": {},
      "wiki-page-file-revision-type.rollback": {},
      "wiki-page-file-revision-type.undelete": {},
      "wiki-page-file-revision-type.undo": {},
      "wiki-page-file-restore.new-page": {},
      "wiki-page-file-restore.new-name": {},

      // Misc
      "wiki-page-edit": {},
      "wiki-page-parent": {},
      "wiki-page-delete": {},
      "wiki-page-move": {},
      "wiki-page-move.new-slug": {},
      "wiki-page-no-render": {},
      "wiki-page-source": {},
      "wiki-page-view-source": {}
    }
  } else {
    translateKeys = {
      ...translateKeys,

      // Page actions
      "restore": {},
      "wiki-page-restore": {},
      "wiki-page-restore.select": {},
      "wiki-page-create": {},
      "wiki-page-deleted": {
        // To be determined lazily
        datetime: "{$datetime}"
      }
    }
  }

  const translated = await translate(locales, translateKeys)

  viewData.internationalization = translated

  if (errorStatus !== null) {
    error(errorStatus, viewData)
  }

  // TODO remove checkRedirect when errorStatus is fixed
  if (checkRedirect) {
    runRedirect(viewData, slug, extra)
  }

  // Return to page for rendering
  return viewData
}

function runRedirect(
  viewData,
  originalSlug: Optional<string>,
  extra: Optional<string>
): void {
  if (!viewData.redirectSite && !viewData.redirectPage) {
    // Nothing to do
    return
  }

  const slug: Optional<string> = viewData.redirectPage || originalSlug
  const route: string = buildRoute(slug, extra)
  redirect(308, `/${route}`)
}

function buildRoute(slug: Optional<string>, extra: Optional<string>): string {
  // Combines a nullable slug and extra to form a route for redirection.
  //
  // Test cases:
  // null, null => ''
  // 'start', null => 'start'
  // 'start', '' => 'start'
  // 'start', 'comments/show' => 'start/comments/show'
  // null, 'xyz' => (impossible)

  if (slug === null) {
    return ""
  } else if (!extra) {
    return slug
  } else {
    return `${slug}/${extra}`
  }
}
