<script lang="ts">
  import { page } from "$app/stores"
  import { invalidateAll } from "$app/navigation"
  import { useErrorPopup, usePagePaneState } from "$lib/stores"
  import { Layout, PagePane } from "$lib/types"
  let showErrorPopup = useErrorPopup()
  let pagePaneState = usePagePaneState()

  async function handleLayout() {
    let form = document.getElementById("page-layout")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    let res = await fetch(`/${$page.data.page.slug}/layout`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      showErrorPopup.set({
        state: true,
        message: res.message,
        data: res.data
      })
    } else {
      pagePaneState.set(PagePane.None)
      invalidateAll()
    }
  }
</script>

<form
  id="page-layout"
  class="page-layout"
  method="POST"
  on:submit|preventDefault={handleLayout}
>
  <select name="layout" class="page-layout-select" value={$page.data.page.layout}>
    <option value={null}
      >{$page.data.internationalization?.["wiki-page-layout.default"]}</option
    >
    {#each Object.values(Layout) as layoutOption}
      <option value={layoutOption}
        >{$page.data.internationalization?.[`wiki-page-layout.${layoutOption}`]}</option
      >
    {/each}
  </select>
  <div class="action-row page-layout-actions">
    <button
      class="action-button page-layout-button button-cancel clickable"
      type="button"
      on:click|stopPropagation={() => {
        pagePaneState.set(PagePane.None)
      }}
    >
      {$page.data.internationalization?.cancel}
    </button>
    <button
      class="action-button page-layout-button button-save clickable"
      type="submit"
      on:click|stopPropagation
    >
      {$page.data.internationalization?.save}
    </button>
  </div>
</form>

<style lang="scss">
  .page-layout {
    display: flex;
    flex-direction: column;
    gap: 15px;
    align-items: stretch;
    justify-content: stretch;
    width: 100%;
    padding: 0 0 2em;
  }
</style>
