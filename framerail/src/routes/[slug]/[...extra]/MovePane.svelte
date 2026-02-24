<script lang="ts">
  import { page } from "$app/state"
  import { goto } from "$app/navigation"
  import { errorPopupState, pageLayoutState, pagePaneState } from "$lib/stores.svelte"
  import { Layout, PagePane } from "$lib/types"
  import { resolve } from "$app/paths"

  let moveInputNewSlugElem = $state<HTMLInputElement>()

  async function handleMove() {
    const form = document.querySelector<HTMLFormElement>("form#page-move")
    if (!form) return
    const fdata = new FormData(form)
    const newSlug = fdata.get("new-slug")
    if (!moveInputNewSlugElem) return
    if (!newSlug) {
      moveInputNewSlugElem.classList.add("error")
      return
    } else {
      moveInputNewSlugElem.classList.remove("error")
    }
    fdata.set("site-id", page.data.site.site_id)
    fdata.set("page-id", page.data.page.page_id)
    fdata.set("last-revision-id", page.data.page_revision.revision_id.toString())
    const res = await fetch(`/${page.data.page.slug}/move`, {
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
      goto(resolve(`/${newSlug}`, {}), {
        noScroll: true
      })
      pagePaneState.current = PagePane.None
    }
  }
</script>

{#if pageLayoutState.current === Layout.WIKIDOT}
  <h1 class="page-move-header">
    {page.data.internationalization?.["wiki-page-move"]}
  </h1>
{:else}
  <h2 class="page-move-header">
    {page.data.internationalization?.["wiki-page-move"]}
  </h2>
{/if}

<form
  id="page-move"
  class="page-move"
  method="POST"
  onsubmit={(event) => {
    event.preventDefault()
    handleMove()
  }}
>
  <input
    bind:this={moveInputNewSlugElem}
    name="new-slug"
    class="page-move-new-slug"
    placeholder={page.data.internationalization?.["wiki-page-move.new-slug"]}
    type="text"
  />
  <textarea
    name="comments"
    class="page-move-comments"
    placeholder={page.data.internationalization?.["wiki-page-revision-comments"]}
  ></textarea>
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
        value={page.data.internationalization?.move}
      />
    </div>
  {:else}
    <div class="action-row page-move-actions">
      <button
        class="action-button page-move-button button-cancel clickable"
        onclick={(event) => {
          event.stopPropagation()
          pagePaneState.current = PagePane.None
        }}
        type="button"
      >
        {page.data.internationalization?.cancel}
      </button>
      <button
        class="action-button page-move-button button-move clickable"
        onclick={(event) => event.stopPropagation()}
        type="submit"
      >
        {page.data.internationalization?.move}
      </button>
    </div>
  {/if}
</form>

<style lang="scss">
  .page-move {
    display: flex;
    flex-direction: column;
    gap: 15px;
    align-items: stretch;
    justify-content: stretch;
    width: 100%;
    padding: 0 0 2em;
  }
</style>
