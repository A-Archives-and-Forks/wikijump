import { authGetSession } from "$lib/server/auth/getSession"
import * as page from "$lib/server/deepwell/page"
import * as pageFile from "$lib/server/deepwell/pageFile"

// Handling of server events from client

export async function POST(event) {
  let data = await event.request.formData()
  let slug = event.params.slug

  let sessionToken = event.cookies.get("wikijump_token")
  let ipAddr = event.getClientAddress()
  let userAgent = event.cookies.get("User-Agent")

  let session = await authGetSession(sessionToken)

  let extra = event.params.extra
    ?.toLowerCase()
    .split("/")
    .filter((flag) => flag.length)

  let pageIdVal = data.get("page-id")?.toString()
  let pageId = pageIdVal ? parseInt(pageIdVal) : null
  let siteIdVal = data.get("site-id")?.toString()
  let siteId = siteIdVal ? parseInt(siteIdVal) : null

  let res: object = {}

  try {
    if (extra.includes("edit")) {
      /** Edit or create page. */
      let comments = data.get("comments")?.toString() ?? ""
      let wikitext = data.get("wikitext")?.toString()
      let title = data.get("title")?.toString()
      let altTitle = data.get("alt-title")?.toString()
      let tagsStr = data.get("tags")?.toString().trim()
      let tags: string[] = []
      if (tagsStr?.length) tags = tagsStr.split(" ").filter((tag) => tag.length)
      let layout = data.get("layout")?.toString().trim()
      let lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      let lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null

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
      let revisionNumberStr = data.get("revision-number")?.toString()
      let revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null
      let limitStr = data.get("limit")?.toString()
      let limit = limitStr ? parseInt(limitStr) : null

      res = await page.pageHistory(siteId, pageId, revisionNumber, limit)
    } else if (extra.includes("move")) {
      /** Move page to new slug. */
      let comments = data.get("comments")?.toString() ?? ""
      let newSlug = data.get("new-slug")?.toString()
      let lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      let lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null

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
      let revisionNumberStr = data.get("revision-number")?.toString()
      let compiledHtml = data.get("compiled-html")?.toString() === "true"
      let wikitext = data.get("wikitext")?.toString() === "true"
      let revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null

      res = await page.pageRevision(
        siteId,
        pageId,
        revisionNumber,
        compiledHtml,
        wikitext
      )
    } else if (extra.includes("rollback")) {
      let revisionNumberStr = data.get("revision-number")?.toString()
      let revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null
      let comments = data.get("comments")?.toString() ?? ""
      let lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      let lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null

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
      let valueStr = data.get("value")?.toString()
      let value = valueStr ? parseInt(valueStr) : null

      res = await page.pageVoteCast(siteId, pageId, session?.user_id, value)
    } else if (extra.includes("vote-cancel")) {
      res = await page.pageVoteRemove(siteId, pageId, session?.user_id)
    } else if (extra.includes("layout")) {
      let layout = data.get("layout")?.toString().trim() ?? null

      res = await page.pageLayout(siteId, pageId, session?.user_id, ipAddr, layout)
    } else if (extra.includes("parent-set")) {
      let addParentStr = data.get("add-parents")?.toString().trim() ?? ""
      let addParents = addParentStr.split(" ").filter((p) => p)
      let removeParentStr = data.get("remove-parents")?.toString().trim() ?? ""
      let removeParents = removeParentStr.split(" ").filter((p) => p)

      if (addParents.length + removeParents.length)
        res = await page.pageParentUpdate(
          siteId,
          pageId,
          session?.user_id,
          addParents.length ? addParents : undefined,
          removeParents.length ? removeParents : undefined
        )
    } else if (extra.includes("parent-get")) {
      res = await page.pageParentGet(siteId, pageId, slug)
    } else if (extra.includes("deleted-get")) {
      res = await page.pageDeletedGet(siteId, slug)
    } else if (extra.includes("restore")) {
      let comments = data.get("comments")?.toString() ?? ""

      res = await page.pageRestore(siteId, pageId, session?.user_id, ipAddr, comments)
    } else if (extra.includes("score")) {
      res = await page.pageScore(siteId, pageId, slug)
    } else if (extra.includes("file-list")) {
      let deleted = data.get("deleted")?.toString() ?? false

      res = await pageFile.pageFileList(
        siteId,
        pageId,
        !["false", "null", "", false].includes(deleted)
      )
    } else if (extra.includes("file-upload")) {
      let file = data.get("file")?.valueOf()
      let name = data.get("name")?.toString().trim()
      if (name === "") name = undefined // use default file name
      let comments = data.get("comments")?.toString() ?? ""

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
      let fileIdStr = data.get("file-id")?.toString().trim()
      let fileId = fileIdStr ? parseInt(fileIdStr) : null
      let lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      let lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null
      let comments = data.get("comments")?.toString() ?? ""

      res = await pageFile.pageFileDelete(
        siteId,
        pageId,
        session?.user_id,
        fileId,
        lastRevId,
        comments
      )
    } else if (extra.includes("file-edit")) {
      let fileIdStr = data.get("file-id")?.toString().trim()
      let fileId = fileIdStr ? parseInt(fileIdStr) : null
      let lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      let lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null
      let file = data.get("file")?.valueOf()
      let name = data.get("name")?.toString().trim()
      if (name === "") name = undefined
      let comments = data.get("comments")?.toString() ?? ""

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
      let fileIdStr = data.get("file-id")?.toString().trim()
      let fileId = fileIdStr ? parseInt(fileIdStr) : null
      let lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      let lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null
      let destinationPage = data.get("destination-page")?.toString()
      let name = data.get("name")?.toString().trim()
      if (name === "") name = undefined
      let comments = data.get("comments")?.toString() ?? ""

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
      let fileIdStr = data.get("file-id")?.toString().trim()
      let fileId = fileIdStr ? parseInt(fileIdStr) : null
      let newPage = data.get("new-page")?.toString().trim()
      let newName = data.get("new-name")?.toString().trim()
      if (newPage === "") newPage = undefined
      if (newName === "") newName = undefined
      let comments = data.get("comments")?.toString() ?? ""

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
      let fileIdStr = data.get("file-id")?.toString().trim()
      let fileId = fileIdStr ? parseInt(fileIdStr) : null
      let revisionNumberStr = data.get("revision-number")?.toString()
      let revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null
      let limitStr = data.get("limit")?.toString()
      let limit = limitStr ? parseInt(limitStr) : null

      res = await pageFile.pageFileHistory(siteId, pageId, fileId, revisionNumber, limit)
    } else if (extra.includes("file-rollback")) {
      let fileIdStr = data.get("file-id")?.toString().trim()
      let fileId = fileIdStr ? parseInt(fileIdStr) : null
      let revisionNumberStr = data.get("revision-number")?.toString()
      let revisionNumber = revisionNumberStr ? parseInt(revisionNumberStr) : null
      let comments = data.get("comments")?.toString() ?? ""
      let lastRevIdStr = data.get("last-revision-id")?.toString().trim()
      let lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null

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
  let data = await event.request.formData()
  let slug = event.params.slug

  let sessionToken = event.cookies.get("wikijump_token")
  let ipAddr = event.getClientAddress()
  let userAgent = event.cookies.get("User-Agent")

  let session = await authGetSession(sessionToken)

  let pageIdVal = data.get("page-id")?.toString()
  let pageId = pageIdVal ? parseInt(pageIdVal) : null
  let siteIdVal = data.get("site-id")?.toString()
  let siteId = siteIdVal ? parseInt(siteIdVal) : null
  let comments = data.get("comments")?.toString() ?? ""
  let lastRevIdStr = data.get("last-revision-id")?.toString().trim()
  let lastRevId = lastRevIdStr ? parseInt(lastRevIdStr) : null

  try {
    let res = await page.pageDelete(
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
