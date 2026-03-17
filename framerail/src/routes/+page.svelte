<script lang="ts">
  import Page from "./[slug]/[...extra]/page.svelte"
  let props = $props()
</script>

<Page {...props} />
