import defaults from "$lib/defaults"
import { client } from "$lib/server/deepwell"
import type { Nullable, Optional } from "$lib/types"
import { Layout } from "$lib/types"

export async function pageDelete(
  siteId: number,
  pageId: Optional<number>,
  userId: number,
  userIpAddr: string,
  slug: string,
  lastRevisionId: number,
  revisionComments: Optional<string>
): Promise<object> {
  return client.request("page_delete", {
    site_id: siteId,
    page: pageId ?? slug,
    user_id: userId,
    ip_address: userIpAddr,
    last_revision_id: lastRevisionId,
    revision_comments: revisionComments
  })
}

export async function pageEdit(
  siteId: number,
  pageId: Optional<number>,
  userId: number,
  userIpAddr: string,
  slug: string,
  lastRevisionId: Optional<number>,
  revisionComments: Optional<string>,
  wikitext: Optional<string>,
  title: Optional<string>,
  altTitle: Optional<string>,
  tags: string[],
  layout: Optional<Nullable<Layout>>
): Promise<object> {
  return client.request(pageId ? "page_edit" : "page_create", {
    site_id: siteId,
    page: pageId ?? slug,
    slug,
    user_id: userId,
    ip_address: userIpAddr,
    last_revision_id: lastRevisionId,
    revision_comments: revisionComments,
    wikitext,
    title,
    alt_title: altTitle,
    tags,
    layout:
      layout !== undefined
        ? Layout[layout?.toUpperCase() as keyof typeof Layout] ?? null
        : undefined
  })
}

export async function pageHistory(
  siteId: number,
  pageId: Optional<number>,
  revisionNumber: Optional<number>,
  limit: Optional<number>
): Promise<object> {
  return client.request("page_revision_range", {
    site_id: siteId,
    page_id: pageId,
    revision_number: revisionNumber ?? defaults.page.history.revisionNumber,
    revision_direction: "before",
    limit: limit ?? defaults.page.history.limit
  })
}

export async function pageMove(
  siteId: number,
  pageId: Optional<number>,
  userId: number,
  userIpAddr: string,
  slug: string,
  lastRevisionId: number,
  newSlug: string,
  revisionComments: Optional<string>
): Promise<object> {
  return client.request("page_move", {
    site_id: siteId,
    page: pageId ?? slug,
    new_slug: newSlug,
    user_id: userId,
    ip_address: userIpAddr,
    last_revision_id: lastRevisionId,
    revision_comments: revisionComments
  })
}

export async function pageRevision(
  siteId: number,
  pageId: Optional<number>,
  revisionNumber: Optional<number>,
  compiledHtml?: boolean,
  wikitext?: boolean
): Promise<object> {
  return client.request("page_revision_get", {
    site_id: siteId,
    page_id: pageId,
    revision_number: revisionNumber ?? defaults.page.history.revisionNumber,
    details: {
      compiled_html: compiledHtml ?? false,
      wikitext: wikitext ?? false
    }
  })
}

export async function pageRollback(
  siteId: number,
  pageId: Optional<number>,
  userId: number,
  userIpAddr: string,
  slug: string,
  lastRevisionId: number,
  revisionNumber: Optional<number>,
  revisionComments: Optional<string>
): Promise<object> {
  return client.request("page_rollback", {
    site_id: siteId,
    page: pageId ?? slug,
    user_id: userId,
    ip_address: userIpAddr,
    last_revision_id: lastRevisionId,
    revision_number: revisionNumber ?? defaults.page.history.revisionNumber,
    revision_comments: revisionComments
  })
}

export async function pageVoteList(
  siteId: number,
  pageId: Optional<number>
): Promise<object> {
  return client.request("vote_list", {
    type: "Page",
    id: pageId,
    deleted: false,
    disabled: false,
    start_id: 0,
    limit: 100
  })
}

export async function pageVoteCast(
  siteId: number,
  pageId: Optional<number>,
  userId: number,
  value: number
): Promise<object> {
  return client.request("vote_set", {
    page_id: pageId,
    user_id: userId,
    value
  })
}

export async function pageVoteRemove(
  siteId: number,
  pageId: Optional<number>,
  userId: number
): Promise<object> {
  return client.request("vote_remove", {
    page_id: pageId,
    user_id: userId
  })
}

export async function pageRerender(siteId: number, pageId: number): Promise<object> {
  return client.request("page_rerender", {
    site_id: siteId,
    page_id: pageId
  })
}

export async function pageLayout(
  siteId: number,
  pageId: number,
  userId: number,
  userIpAddr: string,
  layout: Optional<Nullable<Layout>>
): Promise<object> {
  return client.request("page_set_layout", {
    site_id: siteId,
    page_id: pageId,
    user_id: userId,
    ip_address: userIpAddr,
    layout: Layout[layout?.toUpperCase() as keyof typeof Layout] ?? null
  })
}

export async function pageParentUpdate(
  siteId: number,
  pageId: number,
  userId: number,
  add: Optional<string[]>,
  remove: Optional<string[]>
): Promise<object> {
  return client.request("parent_update", {
    site_id: siteId,
    child: pageId,
    user_id: userId,
    add,
    remove
  })
}

export async function pageParentGet(
  siteId: number,
  pageId: Optional<number>,
  slug: string
): Promise<object> {
  return client.request("parent_get_all", {
    site_id: siteId,
    page: pageId ?? slug
  })
}

export async function pageDeletedGet(siteId: number, slug: string): Promise<object> {
  return client.request("page_get_deleted", {
    site_id: siteId,
    slug
  })
}

export async function pageRestore(
  siteId: number,
  pageId: number,
  userId: number,
  userIpAddr: string,
  revisionComments: Optional<string>
): Promise<object> {
  return client.request("page_restore", {
    site_id: siteId,
    page_id: pageId,
    user_id: userId,
    ip_address: userIpAddr,
    revision_comments: revisionComments
  })
}

export async function pageScore(
  siteId: number,
  pageId: Optional<number>,
  slug: string
): Promise<object> {
  return client.request("page_get_score", {
    site_id: siteId,
    page: pageId ?? slug
  })
}
