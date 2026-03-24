<script lang="ts">
  import type { PageData } from "./$types"

  interface Props {
    data: PageData
  }

  let { data }: Props = $props()
</script>

<h1>UNTRANSLATED:Loaded forum category</h1>
<p>UNTRANSLATED:This is the forum category with ID {data.categoryId}</p>
