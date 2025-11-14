<script lang="ts">
  import { page } from "$app/stores"
  import { goto } from "$app/navigation"
  import { useErrorPopup, usePagePaneState } from "$lib/stores"
  import { PagePane } from "$lib/types"
  let showErrorPopup = useErrorPopup()
  let pagePaneState = usePagePaneState()
  let moveInputNewSlugElem: HTMLInputElement

  async function handleMove() {
    let form = document.getElementById("page-move")
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

<form
  id="page-move"
  class="page-move"
  method="POST"
  on:submit|preventDefault={handleMove}
>
  <input
    bind:this={moveInputNewSlugElem}
    name="new-slug"
    class="page-move-new-slug"
    placeholder={$page.data.internationalization?.["wiki-page-move-new-slug"]}
    type="text"
  />
  <textarea
    name="comments"
    class="page-move-comments"
    placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
  />
  <div class="action-row page-move-actions">
    <button
      class="action-button page-move-button button-cancel clickable"
      type="button"
      on:click|stopPropagation={() => {
        pagePaneState.set(PagePane.None)
      }}
    >
      {$page.data.internationalization?.cancel}
    </button>
    <button
      class="action-button page-move-button button-move clickable"
      type="submit"
      on:click|stopPropagation
    >
      {$page.data.internationalization?.move}
    </button>
  </div>
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
