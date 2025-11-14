<script lang="ts">
  import { page } from "$app/stores"
  import { invalidateAll } from "$app/navigation"
  import { onMount } from "svelte"
  import { useErrorPopup, usePageLayoutState } from "$lib/stores"
  import { Layout } from "$lib/types"
  let showErrorPopup = useErrorPopup()
  let pageLayout = usePageLayoutState()
  let showFileUploadAction = false
  let showFileEditAction = false
  let showFileMoveAction = false
  let showFileRestoreAction = false
  let showFileHistory = false
  let fileMap: Map<number, Record<string, any>> = new Map()
  let filesUpload: FileList
  let filesEditElem: HTMLInputElement
  let fileEditId: number | null = null
  let fileRevisionMap: Map<number, Record<string, any>> = new Map()

  async function getFileList(deleted?: boolean) {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    if (deleted !== undefined) fdata.set("deleted", deleted)
    let res = await fetch(`/${$page.data.page.slug}/file-list`, {
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
      fileMap = new Map()
      res.forEach((file) => {
        fileMap.set(file.file_id, file)
      })
    }
  }

  async function uploadFile() {
    let form = document.getElementById("file-upload")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)

    let res = await fetch(`/${$page.data.page.slug}/file-upload`, {
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
      filesUpload = null
      showFileUploadAction = false
      await getFileList()
    }
  }

  async function deleteFile(fileId: number, lastRevisionId: number) {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("file-id", fileId)
    fdata.set("last-revision-id", lastRevisionId)

    let res = await fetch(`/${$page.data.page.slug}/file-delete`, {
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
      showFileHistory = false
      await getFileList()
    }
  }

  async function editFile() {
    let form = document.getElementById("file-edit")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("file-id", fileEditId)
    fdata.set("last-revision-id", fileMap.get(fileEditId)?.revision_id)

    if (!filesEditElem.files?.length) fdata.delete("file")

    let res = await fetch(`/${$page.data.page.slug}/file-edit`, {
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
      showFileEditAction = false
      showFileHistory = false
      await getFileList()
    }
  }

  async function moveFile() {
    let form = document.getElementById("file-move")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("file-id", fileEditId)
    fdata.set("last-revision-id", fileMap.get(fileEditId)?.revision_id)

    let res = await fetch(`/${$page.data.page.slug}/file-move`, {
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
      showFileMoveAction = false
      showFileHistory = false
      await getFileList()
    }
  }

  async function restoreFile() {
    let form = document.getElementById("file-restore")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("file-id", fileEditId)
    let res = await fetch(`/${$page.params.slug}/file-restore`, {
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
      showFileRestoreAction = false
      getFileList()
      invalidateAll()
    }
  }

  async function handleFileHistory(fileId: number) {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("file-id", fileId)
    let res = await fetch(`/${$page.data.page.slug}/file-history`, {
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
      fileRevisionMap = new Map()
      res.forEach((rev) => {
        fileRevisionMap.set(rev.revision_number, rev)
      })
      showFileHistory = true
    }
  }

  async function rollbackFileRevision(revisionNumber: number, comments?: string) {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("file-id", fileEditId)
    fdata.set("revision-number", revisionNumber)
    fdata.set("last-revision-id", fileMap.get(fileEditId)?.revision_id)
    if (comments !== undefined) fdata.set("comments", comments)
    let res = await fetch(`/${$page.data.page.slug}/file-rollback`, {
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
      getFileList()
      showFileHistory = false
      fileRevisionMap = new Map()
      handleFileHistory(fileEditId)
      invalidateAll()
    }
  }

  onMount(async () => {
    getFileList(false)
  })
</script>

{#if $pageLayout === Layout.WIKIDOT}
  <h1 class="page-file-header">
    {$page.data.internationalization?.["wiki-page-file"]}
  </h1>
{:else}
  <h2 class="page-file-header">
    {$page.data.internationalization?.["wiki-page-file"]}
  </h2>
{/if}

<div class="file-panel">
  {#if $pageLayout === Layout.WIKIDOT}
    <div class="buttons">
      <input
        class="btn btn-primary"
        type="button"
        value={$page.data.internationalization?.upload}
        on:click|stopPropagation={() => {
          showFileUploadAction = true
        }}
      />
      <input
        class="btn btn-default"
        type="submit"
        value={$page.data.internationalization?.restore}
        on:click={() => {
          getFileList(true)
        }}
      />
    </div>
  {:else}
    <div class="action-row file-action">
      <button
        class="action-button upload-file clickable"
        type="button"
        on:click|stopPropagation={() => {
          showFileUploadAction = true
        }}
      >
        {$page.data.internationalization?.upload}
      </button>
      <button
        class="action-button deleted-file clickable"
        type="button"
        on:click={() => {
          getFileList(true)
        }}
      >
        {$page.data.internationalization?.restore}
      </button>
    </div>
  {/if}

  {#if fileMap.size > 0}
    <div class="file-list">
      <div class="file-list-header">
        <div class="file-attribute name">
          {$page.data.internationalization?.["wiki-page-file.name"]}
        </div>
        <div class="file-attribute created-at">
          {$page.data.internationalization?.["wiki-page-file.created-at"]}
        </div>
        <div class="file-attribute updated-at">
          {$page.data.internationalization?.["wiki-page-file.updated-at"]}
        </div>
        {#if $pageLayout !== Layout.WIKIDOT}
          <div class="file-attribute mime">
            {$page.data.internationalization?.["wiki-page-file.mime"]}
          </div>
        {/if}
        <div class="file-attribute size">
          {$page.data.internationalization?.["wiki-page-file.size"]}
        </div>
        <div class="file-attribute action" />
      </div>
      {#each [...fileMap].sort((a, b) => b[0] - a[0]) as [_, file] (file.file_id)}
        <div class="file-row" data-id={file.file_id}>
          <div class="file-attribute name">
            <a
              href="//{$page.data.site_file_domain}/-/file/{$page.data.page
                .slug}/{file.name}">{file.name}</a
            >
          </div>
          <div class="file-attribute created-at">
            {new Date(file.file_created_at).toLocaleString()}
          </div>
          <div class="file-attribute updated-at">
            {file.file_updated_at ? new Date(file.file_updated_at).toLocaleString() : ""}
          </div>
          {#if $pageLayout !== Layout.WIKIDOT}
            <div class="file-attribute mime">
              {file.mime}
            </div>
          {/if}
          <div class="file-attribute size">
            {file.size}
          </div>
          <div class="file-attribute action">
            {#if $pageLayout === Layout.WIKIDOT}
              {#if file.revision_type === "delete"}
                <!-- svelte-ignore a11y-invalid-attribute -->
                <a
                  class="btn btn-primary btn-sm btn-small"
                  href="javascript:;"
                  on:click|stopPropagation={() => {
                    fileEditId = file.file_id
                    showFileRestoreAction = true
                  }}
                >
                  {$page.data.internationalization?.restore}
                </a>
              {:else}
                <!-- svelte-ignore a11y-invalid-attribute -->
                <a
                  class="btn btn-primary btn-sm btn-small"
                  href="javascript:;"
                  on:click|stopPropagation={() => {
                    showFileHistory = false
                    handleFileHistory(file.file_id)
                  }}
                >
                  {$page.data.internationalization?.history}
                </a>
                <!-- svelte-ignore a11y-invalid-attribute -->
                <a
                  class="btn btn-primary btn-sm btn-small"
                  href="javascript:;"
                  on:click|stopPropagation={() => {
                    fileEditId = file.file_id
                    showFileMoveAction = true
                  }}
                >
                  {$page.data.internationalization?.move}
                </a>
                <!-- svelte-ignore a11y-invalid-attribute -->
                <a
                  class="btn btn-primary btn-sm btn-small"
                  href="javascript:;"
                  on:click|stopPropagation={() => {
                    fileEditId = file.file_id
                    showFileEditAction = true
                  }}
                >
                  {$page.data.internationalization?.edit}
                </a>
                <!-- svelte-ignore a11y-invalid-attribute -->
                <a
                  class="btn btn-primary btn-sm btn-small"
                  href="javascript:;"
                  on:click|stopPropagation={() => {
                    deleteFile(file.file_id, file.revision_id)
                  }}
                >
                  {$page.data.internationalization?.delete}
                </a>
              {/if}
            {:else if file.revision_type === "delete"}
              <button
                class="action-button restore-file clickable"
                type="button"
                on:click|stopPropagation={() => {
                  fileEditId = file.file_id
                  showFileRestoreAction = true
                }}
              >
                {$page.data.internationalization?.restore}
              </button>
            {:else}
              <button
                class="action-button file-history clickable"
                type="button"
                on:click|stopPropagation={() => {
                  showFileHistory = false
                  handleFileHistory(file.file_id)
                }}
              >
                {$page.data.internationalization?.history}
              </button>
              <button
                class="action-button move-file clickable"
                type="button"
                on:click|stopPropagation={() => {
                  fileEditId = file.file_id
                  showFileMoveAction = true
                }}
              >
                {$page.data.internationalization?.move}
              </button>
              <button
                class="action-button edit-file clickable"
                type="button"
                on:click|stopPropagation={() => {
                  fileEditId = file.file_id
                  showFileEditAction = true
                }}
              >
                {$page.data.internationalization?.edit}
              </button>
              <button
                class="action-button delete-file clickable"
                type="button"
                on:click|stopPropagation={() => {
                  deleteFile(file.file_id, file.revision_id)
                }}
              >
                {$page.data.internationalization?.delete}
              </button>
            {/if}
          </div>
        </div>
      {/each}
    </div>
  {:else}
    <div class="file-list">
      <div class="file-list-message">
        {$page.data.internationalization?.["wiki-page-file-no-files"]}
      </div>
    </div>
  {/if}

  {#if showFileUploadAction}
    <form
      id="file-upload"
      class="file-upload"
      method="POST"
      on:submit|preventDefault={uploadFile}
    >
      <div class="file-form-field">
        <label for="file"
          >{$page.data.internationalization?.["wiki-page-file-select"]}</label
        >
        <input
          name="file"
          class="file-attribute file"
          type="file"
          bind:files={filesUpload}
        />
      </div>
      <div class="file-form-field">
        <label for="name"
          >{$page.data.internationalization?.["wiki-page-file-name"]}</label
        >
        <input
          name="name"
          class="file-attribute name"
          placeholder={filesUpload?.[0]?.name}
          type="text"
        />
      </div>
      <textarea
        name="comments"
        class="file-form-field file-comments"
        placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
      />
      {#if $pageLayout === Layout.WIKIDOT}
        <div class="buttons">
          <input
            class="btn btn-default"
            type="button"
            value={$page.data.internationalization?.cancel}
            on:click|stopPropagation={() => {
              filesUpload = null
              showFileUploadAction = false
            }}
          />
          <input
            class="btn btn-primary"
            type="submit"
            value={$page.data.internationalization?.upload}
            on:click|stopPropagation
          />
        </div>
      {:else}
        <div class="action-row file-upload-actions">
          <button
            class="action-button file-upload-button button-cancel clickable"
            type="button"
            on:click|stopPropagation={() => {
              filesUpload = null
              showFileUploadAction = false
            }}
          >
            {$page.data.internationalization?.cancel}
          </button>
          <button
            class="action-button file-upload-button button-upload clickable"
            type="submit"
            on:click|stopPropagation
          >
            {$page.data.internationalization?.upload}
          </button>
        </div>
      {/if}
    </form>
  {/if}

  {#if showFileEditAction}
    <form
      id="file-edit"
      class="file-edit"
      method="POST"
      on:submit|preventDefault={editFile}
    >
      <div class="file-form-field">
        <label for="file"
          >{$page.data.internationalization?.["wiki-page-file-select"]}</label
        >
        <input
          bind:this={filesEditElem}
          name="file"
          class="file-attribute file"
          type="file"
        />
      </div>
      <div class="file-form-field">
        <label for="name"
          >{$page.data.internationalization?.["wiki-page-file-name"]}</label
        >
        <input
          name="name"
          class="file-attribute name"
          placeholder={fileMap.get(fileEditId)?.name}
          type="text"
        />
      </div>
      <textarea
        name="comments"
        class="file-form-field file-comments"
        placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
      />
      {#if $pageLayout === Layout.WIKIDOT}
        <div class="buttons">
          <input
            class="btn btn-default"
            type="button"
            value={$page.data.internationalization?.cancel}
            on:click|stopPropagation={() => {
              showFileEditAction = false
            }}
          />
          <input
            class="btn btn-primary"
            type="submit"
            value={$page.data.internationalization?.save}
            on:click|stopPropagation
          />
        </div>
      {:else}
        <div class="action-row file-edit-actions">
          <button
            class="action-button file-edit-button button-cancel clickable"
            type="button"
            on:click|stopPropagation={() => {
              showFileEditAction = false
            }}
          >
            {$page.data.internationalization?.cancel}
          </button>
          <button
            class="action-button file-edit-button button-save clickable"
            type="submit"
            on:click|stopPropagation
          >
            {$page.data.internationalization?.save}
          </button>
        </div>
      {/if}
    </form>
  {/if}

  {#if showFileMoveAction}
    <form
      id="file-move"
      class="file-move"
      method="POST"
      on:submit|preventDefault={moveFile}
    >
      <input
        name="destination-page"
        class="file-move-destination-page"
        placeholder={$page.data.internationalization?.[
          "wiki-page-file-move-destination-page"
        ]}
        type="text"
      />
      <textarea
        name="comments"
        class="file-move-comments"
        placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
      />
      {#if $pageLayout === Layout.WIKIDOT}
        <div class="buttons">
          <input
            class="btn btn-default"
            type="button"
            value={$page.data.internationalization?.cancel}
            on:click|stopPropagation={() => {
              showFileMoveAction = false
            }}
          />
          <input
            class="btn btn-primary"
            type="submit"
            value={$page.data.internationalization?.move}
            on:click|stopPropagation
          />
        </div>
      {:else}
        <div class="action-row file-move-actions">
          <button
            class="action-button file-move-button button-cancel clickable"
            type="button"
            on:click|stopPropagation={() => {
              showFileMoveAction = false
            }}
          >
            {$page.data.internationalization?.cancel}
          </button>
          <button
            class="action-button file-move-button button-move clickable"
            type="submit"
            on:click|stopPropagation
          >
            {$page.data.internationalization?.move}
          </button>
        </div>
      {/if}
    </form>
  {/if}

  {#if showFileRestoreAction}
    <form
      id="file-restore"
      class="file-restore"
      method="POST"
      on:submit|preventDefault={restoreFile}
    >
      <input
        name="new-page"
        class="file-restore-new-page"
        placeholder={$page.data.internationalization?.["wiki-page-file-restore.new-page"]}
        type="text"
      />
      <input
        name="new-name"
        class="file-restore-new-name"
        placeholder={$page.data.internationalization?.["wiki-page-file-restore.new-name"]}
        type="text"
      />
      <textarea
        name="comments"
        class="file-restore-comments"
        placeholder={$page.data.internationalization?.["wiki-page-revision-comments"]}
      />
      {#if $pageLayout === Layout.WIKIDOT}
        <div class="buttons">
          <input
            class="btn btn-default"
            type="button"
            value={$page.data.internationalization?.cancel}
            on:click|stopPropagation={() => {
              showFileRestoreAction = false
            }}
          />
          <input
            class="btn btn-primary"
            type="submit"
            value={$page.data.internationalization?.restore}
            on:click|stopPropagation
          />
        </div>
      {:else}
        <div class="action-row file-restore-actions">
          <button
            class="action-button file-restore-button button-cancel clickable"
            type="button"
            on:click|stopPropagation={() => {
              showFileRestoreAction = false
            }}
          >
            {$page.data.internationalization?.cancel}
          </button>
          <button
            class="action-button file-restore-button button-restore clickable"
            type="submit"
            on:click|stopPropagation
          >
            {$page.data.internationalization?.restore}
          </button>
        </div>
      {/if}
    </form>
  {/if}

  {#if showFileHistory}
    <div class="revision-list">
      <div class="revision-header">
        <div class="revision-attribute action" />
        <div class="revision-attribute revision-number">
          {$page.data.internationalization?.["wiki-page-revision-number"]}
        </div>
        <div class="revision-attribute revision-type">
          {$page.data.internationalization?.["wiki-page-file-revision-type"]}
        </div>
        <div class="revision-attribute created-at">
          {$page.data.internationalization?.["wiki-page-file.created-at"]}
        </div>
        <div class="revision-attribute user">
          {$page.data.internationalization?.["wiki-page-revision-user"]}
        </div>
        <div class="revision-attribute page">
          {$page.data.internationalization?.["wiki-page-file.page"]}
        </div>
        <div class="revision-attribute name">
          {$page.data.internationalization?.["wiki-page-file.name"]}
        </div>
        <div class="revision-attribute mime">
          {$page.data.internationalization?.["wiki-page-file.mime"]}
        </div>
        <div class="revision-attribute size">
          {$page.data.internationalization?.["wiki-page-file.size"]}
        </div>
        <div class="revision-attribute comments">
          {$page.data.internationalization?.["wiki-page-revision-comments"]}
        </div>
      </div>
      <!-- Here we sort the list in descending order. -->
      {#each [...fileRevisionMap].sort((a, b) => b[0] - a[0]) as [_, revisionItem] (revisionItem.revision_number)}
        <div class="revision-row" data-id={revisionItem.revision_id}>
          <div class="revision-attribute action">
            {#if ["create", "regular"].includes(revisionItem.revision_type)}
              {#if $pageLayout === Layout.WIKIDOT}
                <!-- svelte-ignore a11y-invalid-attribute -->
                <a
                  class="btn btn-primary btn-sm btn-small"
                  href="javascript:;"
                  on:click|stopPropagation={() => {
                    fileEditId = revisionItem.file_id
                    rollbackFileRevision(revisionItem.revision_number)
                  }}
                >
                  {$page.data.internationalization?.["wiki-page-revision-rollback"]}
                </a>
              {:else}
                <button
                  class="action-button revision-rollback clickable"
                  type="button"
                  on:click|stopPropagation={() => {
                    fileEditId = revisionItem.file_id
                    rollbackFileRevision(revisionItem.revision_number)
                  }}
                >
                  {$page.data.internationalization?.["wiki-page-revision-rollback"]}
                </button>
              {/if}
            {/if}
          </div>
          <div class="revision-attribute revision-number">
            {revisionItem.revision_number}
          </div>
          <div class="revision-attribute revision-type">
            {$page.data.internationalization?.[
              `wiki-page-file-revision-type.${revisionItem.revision_type}`
            ]}
          </div>
          <div class="revision-attribute created-at">
            {new Date(revisionItem.created_at).toLocaleString()}
          </div>
          <div class="revision-attribute user">
            {revisionItem.user_id}
          </div>
          <div class="revision-attribute page">
            {revisionItem.page_id}
          </div>
          <div class="revision-attribute name">
            {revisionItem.name}
          </div>
          <div class="revision-attribute mime">
            {revisionItem.mime}
          </div>
          <div class="revision-attribute size">
            {revisionItem.size}
          </div>
          <div class="revision-attribute comments">
            {revisionItem.comments}
          </div>
        </div>
      {/each}
    </div>
  {/if}
</div>

<style lang="scss">
  .file-upload,
  .file-edit,
  .file-move,
  .file-restore {
    display: flex;
    flex-direction: column;
    gap: 15px;
    align-items: stretch;
    justify-content: stretch;
    width: 100%;
  }

  .file-list {
    display: table;
    width: 100%;
    padding: 0 0 2em;

    .file-list-header,
    .file-row {
      display: table-row;

      .file-attribute {
        display: table-cell;
      }
    }
  }

  .revision-list {
    display: table;
    width: 100%;

    .revision-header,
    .revision-row {
      display: table-row;

      .revision-attribute {
        display: table-cell;
      }
    }
  }
</style>
