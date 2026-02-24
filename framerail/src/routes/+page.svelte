<script lang="ts">
  import Page from "./[slug]/[...extra]/page.svelte"
  let { data } = $props()
</script>

<Page {data} />
