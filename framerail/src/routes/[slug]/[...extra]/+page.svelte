<script lang="ts">
  import Page from "./page.svelte"

  import type { PageProps } from "./$types"

  let props: PageProps = $props()
</script>

<Page {...props} />
