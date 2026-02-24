<script lang="ts">
  import { page } from "$app/state"
  import { invalidateAll } from "$app/navigation"
  import { errorPopupState, pageLayoutState, pagePaneState } from "$lib/stores.svelte"
  import { Layout, PagePane } from "$lib/types"

  let parents = $state<string>("")

  async function setParents() {
    const form = document.querySelector<HTMLFormElement>("form#page-parent")
    if (!form) return
    const fdata = new FormData(form)
    fdata.set("site-id", page.data.site.site_id)
    fdata.set("page-id", page.data.page.page_id)
    const newParents = (fdata.get("parents")?.toString() ?? "")
      .split(" ")
      .filter((p) => p)
    const oldParents = parents.split(" ").filter((p) => p)
    const removed: string[] = oldParents.filter((p) => !newParents.includes(p))
    const common: string[] = oldParents.filter((p) => newParents.includes(p))
    const added: string[] = newParents.filter((p) => !common.includes(p))

    if (added.length) fdata.set("add-parents", added.join(" "))
    if (removed.length) fdata.set("remove-parents", removed.join(" "))

    const res = await fetch(`/${page.data.page.slug}/parent-set`, {
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
      invalidateAll()
    }
  }

  $effect(() => {
    async function fetchParents() {
      const fdata = new FormData()
      fdata.set("site-id", page.data.site.site_id)
      fdata.set("page-id", page.data.page.page_id)
      const res = await fetch(`/${page.data.page.slug}/parent-get`, {
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
        parents = res.join(" ")
      }
    }

    fetchParents()
  })
</script>

{#if pageLayoutState.current === Layout.WIKIDOT}
  <h1 class="page-parent-header">
    {page.data.internationalization?.["wiki-page-parent"]}
  </h1>
{:else}
  <h2 class="page-parent-header">
    {page.data.internationalization?.["wiki-page-parent"]}
  </h2>
{/if}

<form
  id="page-parent"
  class="page-parent"
  method="POST"
  onsubmit={(event) => {
    event.preventDefault()
    setParents()
  }}
>
  <input
    name="parents"
    class="page-parent-new-parents"
    placeholder={page.data.internationalization?.parents}
    type="text"
    value={parents}
  />
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
    <div class="action-row page-parent-actions">
      <button
        class="action-button page-parent-button button-cancel clickable"
        onclick={(event) => {
          event.stopPropagation()
          pagePaneState.current = PagePane.None
        }}
        type="button"
      >
        {page.data.internationalization?.cancel}
      </button>
      <button
        class="action-button page-parent-button button-save clickable"
        onclick={(event) => event.stopPropagation()}
        type="submit"
      >
        {page.data.internationalization?.save}
      </button>
    </div>
  {/if}
</form>

<style lang="scss">
  .page-parent {
    display: flex;
    flex-direction: column;
    gap: 15px;
    align-items: stretch;
    justify-content: stretch;
    width: 100%;
    padding: 0 0 2em;
  }
</style>
