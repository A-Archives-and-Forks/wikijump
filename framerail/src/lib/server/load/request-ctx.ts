// Helper functions and types for request context, a set of common metadata for each request to Deepwell.

interface RequestContextOptional {
  sessionToken?: string
  siteId?: number
  page?: string | number
}

export type RequestContext = RequestContextOptional | void

export function storeRequestContext(
  locals: App.Locals,
  sessionToken?: string,
  siteId?: number,
  page?: string | number
) {
  locals.requestContext = {
    sessionToken,
    siteId,
    page
  }
}

export function getRequestContext(locals: App.Locals): RequestContext {
  return locals.requestContext
}
