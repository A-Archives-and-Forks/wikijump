<script lang="ts">
  import type { PageData } from "./$types"

  interface Props {
    data: PageData
  }

  let { data }: Props = $props()
</script>

<h1>UNTRANSLATED:Loaded forum thread</h1>
<p>UNTRANSLATED:This is the forum thread with ID {data.threadId}</p>
