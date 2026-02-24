<script lang="ts">
  import { page } from "$app/state"
  import { goto, invalidateAll } from "$app/navigation"
  import { errorPopupState, pageLayoutState, pagePaneState } from "$lib/stores.svelte"
  import { Layout, PagePane } from "$lib/types"
  import { resolve } from "$app/paths"

  let moveInputNewSlugElem = $state<HTMLInputElement>()

  const DeleteOptions = {
    Move: "move",
    Delete: "delete"
  } as const
  type DeleteOptions = (typeof DeleteOptions)[keyof typeof DeleteOptions]
  let delOption = $state<DeleteOptions>(DeleteOptions.Move)

  async function handleDelete() {
    const fdata = new FormData()
    fdata.set("site-id", page.data.site.site_id)
    fdata.set("page-id", page.data.page.page_id)
    fdata.set("last-revision-id", page.data.page_revision.revision_id.toString())
    const res = await fetch(`/${page.data.page.slug}`, {
      method: "DELETE",
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

  async function handleMove() {
    const form = document.querySelector<HTMLFormElement>("form#page-delete")
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
  <h1 class="page-delete-header">
    {page.data.internationalization?.["wiki-page-delete"]}
  </h1>
{:else}
  <h2 class="page-delete-header">
    {page.data.internationalization?.["wiki-page-delete"]}
  </h2>
{/if}

<form
  id="page-delete"
  class="page-delete"
  method="POST"
  onsubmit={(event) => {
    event.preventDefault()
    if (delOption === DeleteOptions.Move) handleMove()
    else handleDelete()
  }}
>
  <input
    id="page-delete-option-move"
    name="page-delete-option"
    type="radio"
    value={DeleteOptions.Move}
    bind:group={delOption}
  />
  <label class="page-delete-option option-move" for="page-delete-option-move">
    {page.data.internationalization?.["wiki-page-move"]}
  </label>
  <input
    id="page-delete-option-delete"
    name="page-delete-option"
    type="radio"
    value={DeleteOptions.Delete}
    bind:group={delOption}
  />
  <label class="page-delete-option option-delete" for="page-delete-option-delete">
    {page.data.internationalization?.["wiki-page-delete"]}
  </label>
  {#if delOption === DeleteOptions.Move}
    <input
      bind:this={moveInputNewSlugElem}
      name="new-slug"
      class="page-move-new-slug"
      placeholder={page.data.internationalization?.["wiki-page-move.new-slug"]}
      type="text"
      value={`deleted:${page.data.page.slug}`}
    />
    <textarea
      name="comments"
      class="page-move-comments"
      placeholder={page.data.internationalization?.["wiki-page-revision-comments"]}
    ></textarea>
  {/if}
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
        value={page.data.internationalization?.confirm}
      />
    </div>
  {:else}
    <div class="action-row page-delete-actions">
      <button
        class="action-button page-delete-button button-cancel clickable"
        onclick={(event) => {
          event.stopPropagation()
          pagePaneState.current = PagePane.None
        }}
        type="button"
      >
        {page.data.internationalization?.cancel}
      </button>
      <button
        class="action-button page-delete-button button-confirm clickable"
        onclick={(event) => event.stopPropagation()}
        type="submit"
      >
        {page.data.internationalization?.confirm}
      </button>
    </div>
  {/if}
</form>

<style lang="scss">
  .page-delete {
    display: flex;
    flex-direction: column;
    gap: 15px;
    align-items: stretch;
    justify-content: stretch;
    width: 100%;
    padding: 0 0 2em;
  }
</style>
