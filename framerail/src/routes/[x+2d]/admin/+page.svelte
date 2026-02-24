<script lang="ts">
  import { page } from "$app/state"
  import { invalidateAll } from "$app/navigation"
  import { errorPopupState } from "$lib/stores.svelte"
  import { Layout } from "$lib/types"

  let isEdit = $state<boolean>(false)

  async function saveEdit() {
    const form = document.querySelector<HTMLFormElement>("form#editor")
    if (!form) return
    const fsrc = new FormData(form)
    const fdata = new FormData()
    for (const [key, val] of fsrc.entries()) {
      if (val !== page.data.site[key as keyof typeof page.data.site]) fdata.set(key, val)
    }
    fdata.set("site-id", page.data.site.site_id)
    fdata.set("action", "edit")
    const res = await fetch(`/-/admin`, {
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
      isEdit = false
      invalidateAll()
    }
  }
</script>

<h1>UNTRANSLATED:Admin panel route</h1>

<textarea class="debug">{JSON.stringify(page, null, 2)}</textarea>

{#if isEdit}
  <form
    id="editor"
    class="editor"
    method="POST"
    onsubmit={(event) => {
      event.preventDefault()
      saveEdit()
    }}
  >
    <label for="name">
      {page.data.internationalization?.["site-info.name"]}
    </label>
    <input
      name="name"
      class="site-attribute name"
      type="text"
      value={page.data.site.name}
    />

    <label for="slug">
      {page.data.internationalization?.["site-info.slug"]}
    </label>
    <input
      name="slug"
      class="site-attribute slug"
      type="text"
      value={page.data.site.slug}
    />

    <label for="tagline">
      {page.data.internationalization?.["site-info.tagline"]}
    </label>
    <input
      name="tagline"
      class="site-attribute tagline"
      type="text"
      value={page.data.site.tagline}
    />

    <label for="description">
      {page.data.internationalization?.["site-info.description"]}
    </label>
    <input
      name="description"
      class="site-attribute description"
      type="text"
      value={page.data.site.description}
    />

    <label for="default-page">
      {page.data.internationalization?.["site-info.default-page"]}
    </label>
    <input
      name="default-page"
      class="site-attribute default-page"
      type="text"
      value={page.data.site.default_page}
    />

    <label for="locale">
      {page.data.internationalization?.["site-info.locale"]}
    </label>
    <input
      name="locale"
      class="site-attribute locale"
      type="text"
      value={page.data.site.locale}
    />

    <label for="layout">
      {page.data.internationalization?.["site-info.layout"]}
    </label>
    <select name="layout" class="site-attribute layout" value={page.data.site.layout}>
      <option value={null}>
        {page.data.internationalization?.["wiki-page-layout.default"]}
      </option>
      {#each Object.values(Layout) as layoutOption (layoutOption)}
        <option value={layoutOption}>
          {page.data.internationalization?.[`wiki-page-layout.${layoutOption}`]}
        </option>
      {/each}
    </select>

    <div class="action-row editor-actions">
      <button
        class="action-button editor-button button-cancel clickable"
        onclick={(event) => {
          event.stopPropagation()
          isEdit = false
        }}
        type="button"
      >
        {page.data.internationalization?.cancel}
      </button>
      <button class="action-button editor-button button-save clickable" type="submit">
        {page.data.internationalization?.save}
      </button>
    </div>
  </form>
{:else}
  <div class="site-info" data-id={page.data.site.site_id}>
    {#if page.data.site.name}
      <div class="site-attribute name">
        <span class="site-attribute-label">
          {page.data.internationalization?.["site-info.name"]}
        </span>
        <span class="site-attribute-value">{page.data.site.name}</span>
      </div>
    {/if}

    {#if page.data.site.slug}
      <div class="site-attribute slug">
        <span class="site-attribute-label">
          {page.data.internationalization?.["site-info.slug"]}
        </span>
        <span class="site-attribute-value">{page.data.site.slug}</span>
      </div>
    {/if}

    {#if page.data.site.tagline}
      <div class="site-attribute tagline">
        <span class="site-attribute-label">
          {page.data.internationalization?.["site-info.tagline"]}
        </span>
        <span class="site-attribute-value">{page.data.site.tagline}</span>
      </div>
    {/if}

    {#if page.data.site.description}
      <div class="site-attribute description">
        <span class="site-attribute-label">
          {page.data.internationalization?.["site-info.description"]}
        </span>
        <span class="site-attribute-value">{page.data.site.description}</span>
      </div>
    {/if}

    {#if page.data.site.default_page}
      <div class="site-attribute default-page">
        <span class="site-attribute-label">
          {page.data.internationalization?.["site-info.default-page"]}
        </span>
        <span class="site-attribute-value">{page.data.site.default_page}</span>
      </div>
    {/if}

    {#if page.data.site.locale}
      <div class="site-attribute locale">
        <span class="site-attribute-label">
          {page.data.internationalization?.["site-info.locale"]}
        </span>
        <span class="site-attribute-value">{page.data.site.locale}</span>
      </div>
    {/if}

    {#if page.data.site.layout}
      <div class="site-attribute layout">
        <span class="site-attribute-label">
          {page.data.internationalization?.["site-info.layout"]}
        </span>
        <span class="site-attribute-value" data-value={page.data.site.layout}>
          {page.data.internationalization?.[`wiki-page-layout.${page.data.site.layout}`]}
        </span>
      </div>
    {/if}
  </div>

  <div class="action-row editor-actions">
    <button
      class="action-button editor-button button-edit clickable"
      onclick={(event) => {
        event.stopPropagation()
        isEdit = true
      }}
      type="button"
    >
      {page.data.internationalization?.edit}
    </button>
  </div>
{/if}

<style global lang="scss">
  .debug {
    width: 100%;
    height: 60vh;
  }

  .site-info {
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

  .action-row {
    display: flex;
    flex-direction: row;
    gap: 10px;
    align-items: stretch;
    justify-content: flex-end;
    width: 100%;
  }
</style>
