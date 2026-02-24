<script lang="ts">
  import { page } from "$app/state"
  import { goto } from "$app/navigation"
  import { errorPopupState, pageLayoutState } from "$lib/stores.svelte"
  import { Layout } from "$lib/types"
  import { resolve } from "$app/paths"

  function cancelEdit() {
    const options: string[] = Object.entries({
      norender: page.data.options.no_render,
      noredirect: page.data.options.no_redirect
    })
      .filter(([, enabled]) => enabled)
      .map(([key]) => `/${key}`)

    goto(resolve(`/${page.data.page.slug}${options.join("")}`, {}), {
      noScroll: true
    })
  }

  async function saveEdit() {
    const form = document.querySelector<HTMLFormElement>("form#editor")
    if (!form) return
    const fdata = new FormData(form)
    fdata.set("site-id", page.data.site.site_id)
    fdata.set("page-id", page.data.page.page_id)
    fdata.set("last-revision-id", page.data.page_revision.revision_id.toString())
    const res = await fetch(`/${page.data.page.slug}/edit`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      errorPopupState.current = {
        state: true,
        message: res.message,
        data: res
      }
    } else {
      goto(resolve(`/${page.data.page.slug}`, {}), {
        noScroll: true
      })
    }
  }
</script>

{#if pageLayoutState.current === Layout.WIKIDOT}
  <h1 class="page-edit-header">
    {page.data.internationalization?.["wiki-page-edit"]}
  </h1>
{:else}
  <h2 class="page-edit-header">
    {page.data.internationalization?.["wiki-page-edit"]}
  </h2>
{/if}

<form
  id="editor"
  class="editor"
  method="POST"
  onsubmit={(event) => {
    event.preventDefault()
    saveEdit()
  }}
>
  <input
    name="title"
    class="editor-title"
    placeholder={page.data.internationalization?.title}
    type="text"
    value={page.data.page_revision.title}
  />
  <input
    name="alt-title"
    class="editor-alt-title"
    placeholder={page.data.internationalization?.["alt-title"]}
    type="text"
    value={page.data.page_revision.alt_title}
  />
  <textarea name="wikitext" class="editor-wikitext">{page.data.wikitext}</textarea>
  <input
    name="tags"
    class="editor-tags"
    placeholder={page.data.internationalization?.tags}
    type="text"
    value={page.data.page_revision.tags?.join(" ")}
  />
  <textarea
    name="comments"
    class="editor-comments"
    placeholder={page.data.internationalization?.["wiki-page-revision-comments"]}
  ></textarea>
  {#if pageLayoutState.current === Layout.WIKIDOT}
    <div class="buttons alignleft">
      <input
        name="cancel"
        class="btn btn-danger"
        onclick={(event) => {
          event.stopPropagation()
          cancelEdit()
        }}
        type="button"
        value={page.data.internationalization?.cancel}
      />
      <input
        name="save"
        class="btn btn-primary"
        onclick={(event) => event.stopPropagation()}
        type="submit"
        value={page.data.internationalization?.save}
      />
    </div>
  {:else}
    <div class="action-row editor-actions">
      <button
        class="action-button editor-button button-cancel clickable"
        onclick={(event) => {
          event.stopPropagation()
          cancelEdit()
        }}
        type="button"
      >
        {page.data.internationalization?.cancel}
      </button>
      <button
        class="action-button editor-button button-save clickable"
        onclick={(event) => event.stopPropagation()}
        type="submit"
      >
        {page.data.internationalization?.save}
      </button>
    </div>
  {/if}
</form>

<style lang="scss">
  .editor-actions {
    padding: 0 0 2em;
  }

  .editor {
    display: flex;
    flex-direction: column;
    gap: 15px;
    align-items: stretch;
    justify-content: stretch;
    width: 100%;
  }

  .editor-wikitext {
    height: 60vh;
  }
</style>
