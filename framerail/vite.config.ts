import { sveltekit } from "@sveltejs/kit/vite"
import { execSync } from "child_process"
import type { UserConfig } from "vite"
import pkg from "./package.json"

let pnpmVersion = null
try {
  pnpmVersion = execSync("pnpm -v").toString("utf-8").trim()
} catch (_) {}

const config: UserConfig = {
  server: {
    host: "::",
    port: 3000,
    strictPort: true,

    // This setting was added to avoid a security issue:
    // https://github.com/vitejs/vite/security/advisories/GHSA-vg6x-rcgg-rjx6
    //
    // Normally this should be a list but setting it to "true" disables the check.
    // After discussion, this is acceptable because:
    // 1. Vite is only used in development, not in deployed instances.
    // 2. In the stack, wws receives requests and reverse proxies them
    //    to framerail. This performs a domain lookup to get site information,
    //    so hostile domains cannot utilize this exception since they are not
    //    in the site_domain table. Essentially, wws acts as "allowedHosts" for us.
    allowedHosts: true
  },
  plugins: [sveltekit()],
  define: {
    // also update $lib/vite-env.d.ts if these defines are changed
    serverInfo: {
      pnpmVersion,
      frontendName: pkg.name ?? null,
      frontendVersion: pkg.version ?? null,
      frontendDescription: pkg.description ?? null,
      frontendRepository: pkg.repository ?? null,
      frontendLicense: pkg.license ?? null
    }
  }
}

export default config
