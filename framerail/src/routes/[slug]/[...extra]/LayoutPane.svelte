<script lang="ts">
  import { page } from "$app/stores"
  import { useErrorPopup, usePageLayoutState, usePagePaneState } from "$lib/stores"
  import { Layout, PagePane } from "$lib/types"
  let showErrorPopup = useErrorPopup()
  let pagePaneState = usePagePaneState()
  let pageLayout = usePageLayoutState()

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
      pageLayout.set(fdata.get("layout") ?? $page.data.site.layout)
      window.location.reload()
    }
  }
</script>

{#if $pageLayout === Layout.WIKIDOT}
  <h1 class="page-layout-header">
    {$page.data.internationalization?.["wiki-page-layout"]}
  </h1>
{:else}
  <h2 class="page-layout-header">
    {$page.data.internationalization?.["wiki-page-layout"]}
  </h2>
{/if}

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
  {#if $pageLayout === Layout.WIKIDOT}
    <div class="buttons">
      <input
        class="btn btn-danger"
        type="button"
        value={$page.data.internationalization?.cancel}
        on:click|stopPropagation={() => {
          pagePaneState.set(PagePane.None)
        }}
      />
      <input
        class="btn btn-primary"
        type="submit"
        value={$page.data.internationalization?.save}
        on:click|stopPropagation
      />
    </div>
  {:else}
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
  {/if}
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
