<script lang="ts">
  import { page } from "$app/stores"
  import { goto, invalidateAll } from "$app/navigation"
  import { useErrorPopup, usePageLayoutState, usePagePaneState } from "$lib/stores"
  import { Layout, PagePane } from "$lib/types"
  let showErrorPopup = useErrorPopup()
  let pagePaneState = usePagePaneState()
  let pageLayout = usePageLayoutState()
  let moveInputNewSlugElem: HTMLInputElement
  enum DeleteOptions {
    Move = "move",
    Delete = "delete"
  }
  let delOption = DeleteOptions.Move

  async function handleDelete() {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("last-revision-id", $page.data.page_revision.revision_id)
    let res = await fetch(`/${$page.data.page.slug}`, {
      method: "DELETE",
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

  async function handleMove() {
    let form = document.getElementById("page-delete")
    let fdata = new FormData(form)
    let newSlug = fdata.get("new-slug")
    if (!newSlug) {
      moveInputNewSlugElem.classList.add("error")
      return
    } else {
      moveInputNewSlugElem.classList.remove("error")
    }
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("last-revision-id", $page.data.page_revision.revision_id)
    let res = await fetch(`/${$page.data.page.slug}/move`, {
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
      goto(`/${newSlug}`, {
        noScroll: true
      })
      pagePaneState.set(PagePane.None)
    }
  }
</script>

{#if $pageLayout === Layout.WIKIDOT}
  <h1 class="page-delete-header">
    {$page.data.internationalization?.["wiki-page-delete"]}
  </h1>
{:else}
  <h2 class="page-delete-header">
    {$page.data.internationalization?.["wiki-page-delete"]}
  </h2>
{/if}

<form
  id="page-delete"
  class="page-delete"
  method="POST"
  on:submit|preventDefault={() => {
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
  /><label class="page-delete-option option-move" for="page-delete-option-move"
    >{$page.data.internationalization?.["wiki-page-move"]}</label
  >
  <input
    id="page-delete-option-delete"
    name="page-delete-option"
    type="radio"
    value={DeleteOptions.Delete}
    bind:group={delOption}
  /><label class="page-delete-option option-delete" for="page-delete-option-delete"
    >{$page.data.internationalization?.["wiki-page-delete"]}</label
  >
  {#if delOption === DeleteOptions.Move}
    <input
      bind:this={moveInputNewSlugElem}
      name="new-slug"
      class="page-move-new-slug"
      placeholder={$page.data.internationalization?.["wiki-page-move.new-slug"]}
      type="text"
      value={`deleted:${$page.data.page.slug}`}
    />
    <textarea
      name="comments"
      class="page-move-comments"
      placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
    ></textarea>
  {/if}
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
        value={$page.data.internationalization?.confirm}
        on:click|stopPropagation
      />
    </div>
  {:else}
    <div class="action-row page-delete-actions">
      <button
        class="action-button page-delete-button button-cancel clickable"
        type="button"
        on:click|stopPropagation={() => {
          pagePaneState.set(PagePane.None)
        }}
      >
        {$page.data.internationalization?.cancel}
      </button>
      <button
        class="action-button page-delete-button button-confirm clickable"
        type="submit"
        on:click|stopPropagation
      >
        {$page.data.internationalization?.confirm}
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
