<script lang="ts">
  import { page } from "$app/state"
  import { errorPopupState, pageLayoutState, pagePaneState } from "$lib/stores.svelte"
  import { Layout, PagePane } from "$lib/types"

  async function handleLayout() {
    const form = document.querySelector<HTMLFormElement>("form#page-layout")
    if (!form) return
    const fdata = new FormData(form)
    fdata.set("site-id", page.data.site.site_id)
    fdata.set("page-id", page.data.page.page_id)
    const res = await fetch(`/${page.data.page.slug}/layout`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      errorPopupState.current = {
        state: true,
        message: res.message,
        data: res.data
      }
    } else {
      pagePaneState.current = PagePane.None
      pageLayoutState.current = fdata.get("layout") ?? page.data.site.layout
      window.location.reload()
    }
  }
</script>

{#if pageLayoutState.current === Layout.WIKIDOT}
  <h1 class="page-layout-header">
    {page.data.internationalization?.["wiki-page-layout"]}
  </h1>
{:else}
  <h2 class="page-layout-header">
    {page.data.internationalization?.["wiki-page-layout"]}
  </h2>
{/if}

<form
  id="page-layout"
  class="page-layout"
  method="POST"
  onsubmit={(event) => {
    event.preventDefault()
    handleLayout()
  }}
>
  <select name="layout" class="page-layout-select" value={page.data.page.layout}>
    <option value={null}>
      {page.data.internationalization?.["wiki-page-layout.default"]}
    </option>
    {#each Object.values(Layout) as layoutOption (layoutOption)}
      <option value={layoutOption}>
        {page.data.internationalization?.[`wiki-page-layout.${layoutOption}`]}
      </option>
    {/each}
  </select>
  {#if pageLayoutState.current === Layout.WIKIDOT}
    <div class="buttons">
      <input
        class="btn btn-danger"
        onclick={(event) => {
          event.stopPropagation()
          pagePaneState.current = PagePane.None
        }}
        type="button"
        value={page.data.internationalization?.cancel}
      />
      <input
        class="btn btn-primary"
        onclick={(event) => event.stopPropagation()}
        type="submit"
        value={page.data.internationalization?.save}
      />
    </div>
  {:else}
    <div class="action-row page-layout-actions">
      <button
        class="action-button page-layout-button button-cancel clickable"
        onclick={(event) => {
          event.stopPropagation()
          pagePaneState.current = PagePane.None
        }}
        type="button"
      >
        {page.data.internationalization?.cancel}
      </button>
      <button
        class="action-button page-layout-button button-save clickable"
        onclick={(event) => event.stopPropagation()}
        type="submit"
      >
        {page.data.internationalization?.save}
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
