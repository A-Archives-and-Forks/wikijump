import { authGetSession } from "$lib/server/auth/getSession"
import * as page from "$lib/server/deepwell/page"
import * as pageFile from "$lib/server/deepwell/pageFile"

// Handling of server events from client

export async function POST(event) {
  const data = await event.request.formData()
  const slug = event.params.slug

  const sessionToken = event.cookies.get("wikijump_token")
  const ipAddr = event.getClientAddress()
  const userAgent = event.cookies.get("User-Agent")

  const session = await authGetSession(sessionToken)

  const extra = event.params.extra
    ?.toLowerCase()
    .split("/")
    .filter((flag) => flag.length)

  const pageIdVal = data.get("page-id")?.toString()
  const pageId = pageIdVal ? parseInt(pageIdVal) : null
  const siteIdVal = data.get("site-id")?.toString()
  const siteId = siteIdVal ? parseInt(siteIdVal) : null

  let res: object = {}

  try {
    if (extra.includes("edit")) {
      /** Edit or create page. */
      const comments = data.get("comments")?.toString() ?? ""
      const wikitext = data.get("wikitext")?.toString()
      const title = data.get("title")?.toString()
      const altTitle = data.get("alt-title")?.toString()
      const tagsStr = data.get("tags")?.toString().trim()
      let tags: string[] = []
      if (tagsStr?.length) tags = tagsStr.split(" ").filter((tag) => tag.length)
      const layout = data.get("layout")?.toString().trim()
      const lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      const lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null

      res = await page.pageEdit(
        siteId,
        pageId,
        session?.user_id,
        ipAddr,
        slug,
        lastRevId,
        comments,
        wikitext,
        title,
        altTitle,
        tags,
        layout
      )
    } else if (extra.includes("history")) {
      /** Retrieve page revision list. */
      const revisionNumberStr = data.get("revision-number")?.toString()
      const revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null
      const limitStr = data.get("limit")?.toString()
      const limit = limitStr ? parseInt(limitStr) : null

      res = await page.pageHistory(siteId, pageId, revisionNumber, limit)
    } else if (extra.includes("move")) {
      /** Move page to new slug. */
      const comments = data.get("comments")?.toString() ?? ""
      const newSlug = data.get("new-slug")?.toString()
      const lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      const lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null

      res = await page.pageMove(
        siteId,
        pageId,
        session?.user_id,
        ipAddr,
        slug,
        lastRevId,
        newSlug,
        comments
      )
    } else if (extra.includes("revision")) {
      const revisionNumberStr = data.get("revision-number")?.toString()
      const compiledHtml = data.get("compiled-html")?.toString() === "true"
      const wikitext = data.get("wikitext")?.toString() === "true"
      const revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null

      res = await page.pageRevision(
        siteId,
        pageId,
        revisionNumber,
        compiledHtml,
        wikitext
      )
    } else if (extra.includes("rollback")) {
      const revisionNumberStr = data.get("revision-number")?.toString()
      const revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null
      const comments = data.get("comments")?.toString() ?? ""
      const lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      const lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null

      res = await page.pageRollback(
        siteId,
        pageId,
        session?.user_id,
        ipAddr,
        slug,
        lastRevId,
        revisionNumber,
        comments
      )
    } else if (extra.includes("vote-get")) {
      res = await page.pageVoteList(siteId, pageId)
    } else if (extra.includes("vote-cast")) {
      const valueStr = data.get("value")?.toString()
      const value = valueStr ? parseInt(valueStr) : null

      res = await page.pageVoteCast(siteId, pageId, session?.user_id, value)
    } else if (extra.includes("vote-cancel")) {
      res = await page.pageVoteRemove(siteId, pageId, session?.user_id)
    } else if (extra.includes("layout")) {
      const layout = data.get("layout")?.toString().trim() ?? null

      res = await page.pageLayout(siteId, pageId, session?.user_id, ipAddr, layout)
    } else if (extra.includes("parent-set")) {
      const addParentStr = data.get("add-parents")?.toString().trim() ?? ""
      const addParents = addParentStr.split(" ").filter((p) => p)
      const removeParentStr = data.get("remove-parents")?.toString().trim() ?? ""
      const removeParents = removeParentStr.split(" ").filter((p) => p)

      if (addParents.length + removeParents.length) {
        res = await page.pageParentUpdate(
          siteId,
          pageId,
          session?.user_id,
          addParents.length ? addParents : undefined,
          removeParents.length ? removeParents : undefined
        )
      }
    } else if (extra.includes("parent-get")) {
      res = await page.pageParentGet(siteId, pageId, slug)
    } else if (extra.includes("deleted-get")) {
      res = await page.pageDeletedGet(siteId, slug)
    } else if (extra.includes("restore")) {
      const comments = data.get("comments")?.toString() ?? ""

      res = await page.pageRestore(siteId, pageId, session?.user_id, ipAddr, comments)
    } else if (extra.includes("score")) {
      res = await page.pageScore(siteId, pageId, slug)
    } else if (extra.includes("file-list")) {
      const deleted = data.get("deleted")?.toString() ?? false

      res = await pageFile.pageFileList(
        siteId,
        pageId,
        !["false", "null", "", false].includes(deleted)
      )
    } else if (extra.includes("file-upload")) {
      const file = data.get("file")?.valueOf()
      let name = data.get("name")?.toString().trim()
      if (name === "") name = undefined // use default file name
      const comments = data.get("comments")?.toString() ?? ""

      res = await pageFile.pageFileCreate(
        siteId,
        pageId,
        session?.user_id,
        name,
        file,
        null,
        comments
      )
    } else if (extra.includes("file-delete")) {
      const fileIdStr = data.get("file-id")?.toString().trim()
      const fileId = fileIdStr ? parseInt(fileIdStr) : null
      const lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      const lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null
      const comments = data.get("comments")?.toString() ?? ""

      res = await pageFile.pageFileDelete(
        siteId,
        pageId,
        session?.user_id,
        fileId,
        lastRevId,
        comments
      )
    } else if (extra.includes("file-edit")) {
      const fileIdStr = data.get("file-id")?.toString().trim()
      const fileId = fileIdStr ? parseInt(fileIdStr) : null
      const lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      const lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null
      const file = data.get("file")?.valueOf()
      let name = data.get("name")?.toString().trim()
      if (name === "") name = undefined
      const comments = data.get("comments")?.toString() ?? ""

      res = await pageFile.pageFileEdit(
        siteId,
        pageId,
        session?.user_id,
        fileId,
        name,
        file,
        lastRevId,
        comments
      )
    } else if (extra.includes("file-move")) {
      const fileIdStr = data.get("file-id")?.toString().trim()
      const fileId = fileIdStr ? parseInt(fileIdStr) : null
      const lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      const lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null
      const destinationPage = data.get("destination-page")?.toString()
      let name = data.get("name")?.toString().trim()
      if (name === "") name = undefined
      const comments = data.get("comments")?.toString() ?? ""

      res = await pageFile.pageFileMove(
        siteId,
        pageId,
        destinationPage,
        session?.user_id,
        fileId,
        lastRevId,
        name,
        comments
      )
    } else if (extra.includes("file-restore")) {
      const fileIdStr = data.get("file-id")?.toString().trim()
      const fileId = fileIdStr ? parseInt(fileIdStr) : null
      let newPage = data.get("new-page")?.toString().trim()
      let newName = data.get("new-name")?.toString().trim()
      if (newPage === "") newPage = undefined
      if (newName === "") newName = undefined
      const comments = data.get("comments")?.toString() ?? ""

      res = await pageFile.pageFileRestore(
        siteId,
        pageId,
        session?.user_id,
        fileId,
        newPage,
        newName,
        comments
      )
    } else if (extra.includes("file-history")) {
      /** Retrieve file revision list. */
      const fileIdStr = data.get("file-id")?.toString().trim()
      const fileId = fileIdStr ? parseInt(fileIdStr) : null
      const revisionNumberStr = data.get("revision-number")?.toString()
      const revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null
      const limitStr = data.get("limit")?.toString()
      const limit = limitStr ? parseInt(limitStr) : null

      res = await pageFile.pageFileHistory(siteId, pageId, fileId, revisionNumber, limit)
    } else if (extra.includes("file-rollback")) {
      const fileIdStr = data.get("file-id")?.toString().trim()
      const fileId = fileIdStr ? parseInt(fileIdStr) : null
      const revisionNumberStr = data.get("revision-number")?.toString()
      const revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null
      const comments = data.get("comments")?.toString() ?? ""
      const lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      const lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null

      res = await pageFile.pageFileRollback(
        siteId,
        pageId,
        session?.user_id,
        fileId,
        lastRevId,
        revisionNumber,
        comments
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

/** Delete page. */
export async function DELETE(event) {
  const data = await event.request.formData()
  const slug = event.params.slug

  const sessionToken = event.cookies.get("wikijump_token")
  const ipAddr = event.getClientAddress()
  const userAgent = event.cookies.get("User-Agent")

  const session = await authGetSession(sessionToken)

  const pageIdVal = data.get("page-id")?.toString()
  const pageId = pageIdVal ? parseInt(pageIdVal) : null
  const siteIdVal = data.get("site-id")?.toString()
  const siteId = siteIdVal ? parseInt(siteIdVal) : null
  const comments = data.get("comments")?.toString() ?? ""
  const lastRevIdStr = data.get("last-revision-id")?.toString().trim()
  const lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null

  try {
    const res = await page.pageDelete(
      siteId,
      pageId,
      session?.user_id,
      ipAddr,
      slug,
      lastRevId,
      comments
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
