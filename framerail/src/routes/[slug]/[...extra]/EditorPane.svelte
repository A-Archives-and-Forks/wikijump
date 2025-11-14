<script lang="ts">
  import { page } from "$app/stores"
  import { goto } from "$app/navigation"
  import { useErrorPopup } from "$lib/stores"
  let showErrorPopup = useErrorPopup()

  function cancelEdit() {
    let options: string[] = []
    if ($page.data.options.no_render) options.push("norender")
    if ($page.data.options.no_redirect) options.push("noredirect")
    options = options.map((opt) => `/${opt}`)
    goto(`/${$page.data.page.slug}${options.join("")}`, {
      noScroll: true
    })
  }

  async function saveEdit() {
    let form = document.getElementById("editor")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("last-revision-id", $page.data.page_revision.revision_id)
    let res = await fetch(`/${$page.data.page.slug}/edit`, {
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
      goto(`/${$page.data.page.slug}`, {
        noScroll: true
      })
    }
  }
</script>

<form id="editor" class="editor" method="POST" on:submit|preventDefault={saveEdit}>
  <input
    name="title"
    class="editor-title"
    placeholder={$page.data.internationalization?.title}
    type="text"
    value={$page.data.page_revision.title}
  />
  <input
    name="alt-title"
    class="editor-alt-title"
    placeholder={$page.data.internationalization?.["alt-title"]}
    type="text"
    value={$page.data.page_revision.alt_title}
  />
  <textarea name="wikitext" class="editor-wikitext">{$page.data.wikitext}</textarea>
  <input
    name="tags"
    class="editor-tags"
    placeholder={$page.data.internationalization?.tags}
    type="text"
    value={$page.data.page_revision.tags.join(" ")}
  />
  <textarea
    name="comments"
    class="editor-comments"
    placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
  />
  <div class="action-row editor-actions">
    <button
      class="action-button editor-button button-cancel clickable"
      type="button"
      on:click|stopPropagation={cancelEdit}
    >
      {$page.data.internationalization?.cancel}
    </button>
    <button
      class="action-button editor-button button-save clickable"
      type="submit"
      on:click|stopPropagation
    >
      {$page.data.internationalization?.save}
    </button>
  </div>
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
