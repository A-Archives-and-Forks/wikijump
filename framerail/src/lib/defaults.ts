const defaults = {
  fallbackLocale: "en",
  translateKeys: {
    // Error
    "error": {},
    "close": {},

    // Footer
    "footer-powered-by": {},
    "terms-conditions": {},
    "privacy": {},
    "docs": {},
    "security": {},
    "footer-license-unless": {}
  },
  translateStripKeys: ["footer-license-unless"],
  page: {
    history: {
      revisionNumber: -1,
      limit: 20
    }
  }
}

export default defaults
