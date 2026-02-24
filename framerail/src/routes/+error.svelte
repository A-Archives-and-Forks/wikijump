<script lang="ts">
  import { page } from "$app/state"
  import { goto, invalidateAll } from "$app/navigation"
  import { Layout } from "$lib/types"
  import { resolve } from "$app/paths"
  import { pageLayoutState, errorPopupState } from "$lib/stores.svelte"

  let showRestoreAction = $state<boolean>(false)
  let deletedPages = $state<Record<string, any>[]>([])

  function cancelCreate() {
    goto(resolve(`/${page.params.slug}`, {}), {
      noScroll: true
    })
  }

  async function saveCreate() {
    const form = document.querySelector<HTMLFormElement>("form#editor")
    if (!form) return
    const fdata = new FormData(form)
    fdata.set("site-id", page.error?.site.site_id)
    fdata.set("slug", page.params.slug)
    const res = await fetch(`/${page.params.slug}/edit`, {
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
      goto(resolve(`/${page.params.slug}`, {}), {
        noScroll: true
      })
    }
  }

  async function getDeleted() {
    const fdata = new FormData()
    fdata.set("site-id", page.error?.site.site_id)
    const res = await fetch(`/${page.params.slug}/deleted-get`, {
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
      deletedPages = res
      showRestoreAction = true
    }
  }

  async function handleRestore() {
    const form = document.querySelector<HTMLFormElement>("form#page-restore")
    if (!form) return
    const fdata = new FormData(form)
    fdata.set("site-id", page.error?.site.site_id)
    let res = await fetch(`/${page.params.slug}/restore`, {
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
      showRestoreAction = false
      invalidateAll()
    }
  }
</script>

<h1>UNTRANSLATED:Svelte Error</h1>

<p><textarea class="debug">{JSON.stringify(page, null, 2)}</textarea></p>

<!--
Use svelte-switch-case package with {#switch data.view}
as soon as we can figure out prettier support for it.
-->
{#if page.error?.view === "missing"}
  UNTRANSLATED:Page not found

  {#if page.error?.options?.edit}
    {#if pageLayoutState.current === Layout.WIKIDOT}
      <h1 class="page-create-header">
        {page.error?.internationalization?.["wiki-page-create"]}
      </h1>
    {:else}
      <h2 class="page-create-header">
        {page.error?.internationalization?.["wiki-page-create"]}
      </h2>
    {/if}

    <form
      id="editor"
      class="editor"
      method="POST"
      onsubmit={(event) => {
        event.preventDefault()
        saveCreate()
      }}
    >
      <input
        name="title"
        class="editor-title"
        placeholder={page.error.internationalization?.title}
        type="text"
      />
      <input
        name="alt-title"
        class="editor-alt-title"
        placeholder={page.error.internationalization?.["alt-title"]}
        type="text"
      />
      <textarea name="wikitext" class="editor-wikitext"></textarea>
      <input
        name="tags"
        class="editor-tags"
        placeholder={page.error.internationalization?.tags}
        type="text"
      />
      <select name="layout" class="editor-layout">
        <option value={null}>
          {page.error.internationalization?.["wiki-page-layout.default"]}
        </option>
        {#each Object.values(Layout) as layoutOption (layoutOption.toString())}
          <option value={layoutOption}>
            {page.error.internationalization?.[`wiki-page-layout.${layoutOption}`]}
          </option>
        {/each}
      </select>
      <textarea
        name="comments"
        class="editor-comments"
        placeholder={page.error.internationalization?.["wiki-page-revision-comments"]}
      ></textarea>
      {#if pageLayoutState.current === Layout.WIKIDOT}
        <div class="buttons">
          <input
            class="btn btn-danger"
            onclick={(event) => {
              event.preventDefault()
              cancelCreate()
            }}
            type="button"
            value={page.error.internationalization?.cancel}
          />
          <input
            class="btn btn-primary"
            onclick={(event) => event.stopPropagation()}
            type="submit"
            value={page.error.internationalization?.save}
          />
        </div>
      {:else}
        <div class="action-row editor-actions">
          <button
            class="action-button editor-button button-cancel clickable"
            onclick={(event) => {
              event.preventDefault()
              cancelCreate()
            }}
            type="button"
          >
            {page.error.internationalization?.cancel}
          </button>
          <button
            class="action-button editor-button button-save clickable"
            onclick={(event) => event.stopPropagation()}
            type="submit"
          >
            {page.error.internationalization?.save}
          </button>
        </div>
      {/if}
    </form>
  {:else}
    <div id="page-content">
      {@html page.error.compiled_body_html}
    </div>

    {#if pageLayoutState.current === Layout.WIKIDOT}
      <div id="page-options-container">
        <div id="page-options-bottom" class="page-options-bottom">
          <!-- svelte-ignore a11y_invalid_attribute -->
          <a
            id="restore-button"
            class="btn btn-default"
            href="javascript:;"
            onclick={getDeleted}
            type="button"
          >
            {page.error.internationalization?.restore}
          </a>
        </div>
      </div>
    {:else}
      <div class="action-row editor-actions">
        <button
          class="action-button editor-button button-restore clickable"
          onclick={getDeleted}
          type="button"
        >
          {page.error.internationalization?.restore}
        </button>
      </div>
    {/if}

    {#if showRestoreAction}
      {#if pageLayoutState.current === Layout.WIKIDOT}
        <div id="action-area">
          <h1 class="page-restore-header">
            {page.error?.internationalization?.["wiki-page-restore"]}
          </h1>

          <form
            id="page-restore"
            class="page-restore"
            method="POST"
            onsubmit={(event) => {
              event.preventDefault()
              handleRestore()
            }}
          >
            <fieldset>
              <legend>
                {page.error.internationalization?.["wiki-page-restore.select"]}
              </legend>
              {#each deletedPages as deletedPage (deletedPage.page_id)}
                <input
                  id={`restore-page-id-${deletedPage.page_id}`}
                  name="page-id"
                  class="page-restore-id"
                  type="radio"
                  value={deletedPage.page_id}
                />
                <label for={`restore-page-id-${deletedPage.page_id}`}>
                  <span class="page-restore-title">{deletedPage.title}</span>
                  {#if deletedPage.alt_title}
                    &nbsp;-&nbsp;
                    <span class="page-restore-alt-title">
                      {deletedPage.alt_title}
                    </span>
                  {/if} (
                  <span class="page-restore-rating">
                    {(deletedPage.rating > 0 ? "+" : "") + deletedPage.rating}
                  </span>
                  ) - {page.error?.internationalization?.["wiki-page-deleted"].replace(
                    "{$datetime}",
                    new Date(deletedPage.page_deleted_at).toLocaleString()
                  )}
                </label>
                <br />
              {/each}
            </fieldset>

            <textarea
              name="comments"
              class="page-restore-comments"
              placeholder={page.error?.internationalization?.[
                "wiki-page-revision-comments"
              ]}
            ></textarea>

            <div class="buttons">
              <input
                class="btn btn-primary"
                onclick={(event) => {
                  event.stopPropagation()
                  showRestoreAction = false
                }}
                type="button"
                value={page.error?.internationalization?.cancel}
              />
              <input
                class="btn btn-primary"
                onclick={(event) => event.stopPropagation()}
                type="submit"
                value={page.error?.internationalization?.restore}
              />
            </div>
          </form>
        </div>
      {:else}
        <h2 class="page-restore-header">
          {page.error?.internationalization?.["wiki-page-restore"]}
        </h2>

        <form
          id="page-restore"
          class="page-restore"
          method="POST"
          onsubmit={(event) => {
            event.preventDefault()
            handleRestore()
          }}
        >
          <fieldset>
            <legend>
              {page.error.internationalization?.["wiki-page-restore.select"]}
            </legend>
            {#each deletedPages as deletedPage (deletedPage.page_id)}
              <input
                id={`restore-page-id-${deletedPage.page_id}`}
                name="page-id"
                class="page-restore-id"
                type="radio"
                value={deletedPage.page_id}
              />
              <label for={`restore-page-id-${deletedPage.page_id}`}>
                <span class="page-restore-title">{deletedPage.title}</span
                >{#if deletedPage.alt_title}&nbsp;-&nbsp;<span
                    class="page-restore-alt-title">{deletedPage.alt_title}</span
                  >{/if} (<span class="page-restore-rating"
                  >{(deletedPage.rating > 0 ? "+" : "") + deletedPage.rating}</span
                >) - {page.error?.internationalization?.["wiki-page-deleted"].replace(
                  "{$datetime}",
                  new Date(deletedPage.page_deleted_at).toLocaleString()
                )}
              </label>
              <br />
            {/each}
          </fieldset>

          <textarea
            name="comments"
            class="page-restore-comments"
            placeholder={page.error?.internationalization?.[
              "wiki-page-revision-comments"
            ]}
          ></textarea>

          <div class="action-row page-restore-actions">
            <button
              class="action-button page-restore-button button-cancel clickable"
              onclick={(event) => {
                event.stopPropagation()
                showRestoreAction = false
              }}
              type="button"
            >
              {page.error?.internationalization?.cancel}
            </button>
            <button
              class="action-button page-restore-button button-restore clickable"
              onclick={(event) => event.stopPropagation()}
              type="submit"
            >
              {page.error?.internationalization?.restore}
            </button>
          </div>
        </form>
      {/if}
    {/if}
  {/if}
{:else if page.error?.view === "permissions"}
  UNTRANSLATED:Lacks permissions for page
  {@html page.error?.compiled_body_html}
{:else}
  UNTRANSLATED:Fatal error: Unable to display view
{/if}

<style global lang="scss">
  .debug {
    width: 100%;
    height: 60vh;
  }

  .editor,
  .page-restore {
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

  .editor-actions,
  .page-restore-actions {
    display: flex;
    flex-direction: row;
    gap: 10px;
    align-items: stretch;
    justify-content: flex-end;
    width: 100%;
  }
</style>
