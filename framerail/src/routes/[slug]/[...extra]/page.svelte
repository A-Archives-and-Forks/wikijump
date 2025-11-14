<script lang="ts">
  import { page } from "$app/stores"
  import { goto, invalidateAll } from "$app/navigation"
  import { onMount } from "svelte"
  import { useErrorPopup, usePagePaneState } from "$lib/stores"
  import { PagePane } from "$lib/types"
  import {
    EditorPane,
    FilePane,
    HistoryPane,
    LayoutPane,
    MovePane,
    ParentPane,
    VotePane
  } from "."
  let showErrorPopup = useErrorPopup()
  let pagePaneState = usePagePaneState()

  let showSource = false
  let showRevision = false
  let revision: Record<string, any> = {}

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
    } else invalidateAll()
  }

  function navigateEdit() {
    let options: string[] = []
    if ($page.data.options.no_render) options.push("norender")
    if ($page.data.options.no_redirect) options.push("noredirect")
    options = options.map((opt) => `/${opt}`)
    goto(`/${$page.data.page.slug}${options.join("")}/edit`, {
      noScroll: true
    })
  }

  function setShowRevision(state: boolean) {
    showRevision = state
  }

  function setRevision(rev: Record<string, any>) {
    revision = rev
  }

  onMount(() => {
    if ($page.data?.options.history) pagePaneState.set(PagePane.History)
  })
</script>

<h1>UNTRANSLATED:Loaded page</h1>
<p>
  UNTRANSLATED:Response <textarea class="debug">{JSON.stringify($page, null, 2)}</textarea
  >
</p>

{#if showRevision}
  <h2>{revision.title}</h2>
{:else}
  <h2>{$page.data.page_revision.title}</h2>
{/if}

<hr />

<div class="page-content">
  {#if $page.data.options?.no_render}
    {$page.data.internationalization["wiki-page-no-render"]}
    <textarea class="page-source" readonly={true}>{$page.data.wikitext}</textarea>
  {:else if showRevision}
    {@html revision.compiled_html}
  {:else}
    {@html $page.data.compiled_html}
  {/if}
</div>

<div class="page-tags-container">
  {$page.data.internationalization?.tags}
  <hr />
  <ul class="page-tags">
    {#if showRevision}
      {#each revision.tags as tag}
        <li class="tag">{tag}</li>
      {/each}
    {:else}
      {#each $page.data.page_revision.tags as tag}
        <li class="tag">{tag}</li>
      {/each}
    {/if}
  </ul>
</div>

<div class="page-meta-info-container">
  <div class="page-meta-info info-revision">
    {$page.data.internationalization["wiki-page-revision"]}
  </div>
  <div class="page-meta-info info-last-edit">
    {$page.data.internationalization["wiki-page-last-edit"]}
  </div>
</div>

{#if $page.data.options?.edit}
  <EditorPane />
{:else}
  <div class="action-row editor-actions">
    <button
      class="action-button editor-button button-move clickable"
      type="button"
      on:click={() => {
        pagePaneState.set(PagePane.Move)
      }}
    >
      {$page.data.internationalization?.move}
    </button>
    <button
      class="action-button editor-button button-layout clickable"
      type="button"
      on:click={() => {
        pagePaneState.set(PagePane.Layout)
      }}
    >
      {$page.data.internationalization?.layout}
    </button>
    <button
      class="action-button editor-button button-parents clickable"
      type="button"
      on:click={() => {
        pagePaneState.set(PagePane.Parent)
      }}
    >
      {$page.data.internationalization?.parents}
    </button>
    <button
      class="action-button editor-button button-delete clickable"
      type="button"
      on:click={handleDelete}
    >
      {$page.data.internationalization?.delete}
    </button>
    <button
      class="action-button editor-button button-edit clickable"
      type="button"
      on:click={navigateEdit}
    >
      {$page.data.internationalization?.edit}
    </button>
  </div>
  <div class="action-row other-actions">
    <button
      class="action-button button-source clickable"
      type="button"
      on:click={() => (showSource = true)}
    >
      {$page.data.internationalization?.["wiki-page-view-source"]}
    </button>
    <button
      class="action-button button-history clickable"
      type="button"
      on:click={() => {
        pagePaneState.set(PagePane.History)
      }}
    >
      {$page.data.internationalization?.history}
    </button>
    <button
      class="action-button button-vote clickable"
      type="button"
      on:click={() => {
        pagePaneState.set(PagePane.Vote)
      }}
    >
      {$page.data.internationalization?.vote}
    </button>
    <button
      class="action-button button-files clickable"
      type="button"
      on:click={() => {
        pagePaneState.set(PagePane.File)
      }}
    >
      {$page.data.internationalization?.files}
    </button>
  </div>
{/if}

{#if showSource}
  <textarea class="page-source" readonly={true}>{$page.data.wikitext}</textarea>
{/if}

{#if $pagePaneState === PagePane.Move}
  <MovePane />
{:else if $pagePaneState === PagePane.Layout}
  <LayoutPane />
{:else if $pagePaneState === PagePane.Parent}
  <ParentPane />
{:else if $pagePaneState === PagePane.Vote}
  <VotePane />
{:else if $pagePaneState === PagePane.File}
  <FilePane />
{:else if $pagePaneState === PagePane.History}
  <HistoryPane {setRevision} {setShowRevision} />
{/if}

<style global lang="scss">
  .debug {
    width: 100%;
    height: 60vh;
  }

  .page-content,
  .page-tags-container,
  .page-meta-info-container,
  .editor-actions,
  .other-actions {
    padding: 0 0 2em;
  }

  .page-tags {
    display: flex;
    flex-direction: row;
    flex-wrap: wrap;
    gap: 10px;
    align-items: center;
    justify-content: flex-start;
    padding: 0;
    margin: 0;
    list-style: none;
  }

  .page-meta-info-container {
    text-align: right;
  }

  .page-source {
    width: 100%;
    height: 60vh;
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
