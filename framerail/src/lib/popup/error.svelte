<script lang="ts">
  export let exitPrompt: () => void
  import { onMount, onDestroy } from "svelte"
  import { page } from "$app/stores"
  import { useErrorPopup, usePageLayoutState } from "$lib/stores"
  import { Layout } from "$lib/types"
  let showErrorPopup = useErrorPopup()
  let pageLayout = usePageLayoutState()
  function containerExitPrompt(e: Event) {
    e.preventDefault()
    let classList = (e.target as HTMLElement).classList
    if (
      classList.contains("modal-container") ||
      classList.contains("odialog-container") ||
      classList.contains("odialog-shader") ||
      classList.contains("button-close-message")
    ) {
      exitPrompt()
    }
  }
  const escKeydown = (e: KeyboardEvent) => {
    if (e.code.toLowerCase() === "escape") exitPrompt()
  }
  onMount(() => {
    window.addEventListener("keydown", escKeydown)
  })
  onDestroy(() => {
    window.removeEventListener("keydown", escKeydown)
  })
</script>

{#if $pageLayout === Layout.WIKIDOT}
  <div
    id="odialog-shader"
    class="odialog-shader"
    role="presentation"
    on:click={containerExitPrompt}
    on:keydown={escKeydown}
  ></div>
  <div
    id="odialog-container"
    style:--basalt-compat="block"
    class="odialog-container"
    role="presentation"
    on:click={containerExitPrompt}
    on:keydown={escKeydown}
  >
    <div id="owindow-1" class="owindow error">
      <div class="title modal-header">
        {$page.data?.internationalization?.error ??
          $page.error?.internationalization?.error}
      </div>
      <div class="content modal-body">
        <h1 id="modal-title">
          {$showErrorPopup.message}
        </h1>
        {#if $showErrorPopup.data}
          <p id="model-message-extra" class="modal-message-extra">
            {$showErrorPopup.data.call_trace}
          </p>
        {/if}
      </div>
      <div class="button-bar modal-footer">
        <!-- svelte-ignore a11y-invalid-attribute -->
        <a
          class="btn btn-default button button-close-message"
          href="javascript:;"
          role="button"
          tabindex="0"
          on:click={containerExitPrompt}
          on:keydown={escKeydown}
          >{$page.data?.internationalization?.close ??
            $page.error?.internationalization?.close}</a
        >
      </div>
    </div>
  </div>
{:else}
  <div
    class="modal-container"
    aria-describedby="modal-message"
    aria-labelledby="modal-title"
    role="presentation"
    on:click={containerExitPrompt}
    on:keydown={escKeydown}
  >
    <div class="modal error-modal">
      <h2 id="modal-title">
        {$page.data?.internationalization?.error ??
          $page.error?.internationalization?.error}
      </h2>
      <div id="modal-message" class="modal-message">
        {$showErrorPopup.message}
      </div>
      {#if $showErrorPopup.data}
        <div id="model-message-extra" class="modal-message-extra">
          {$showErrorPopup.data.call_trace}
        </div>
      {/if}
    </div>
  </div>
{/if}

<style global lang="scss">
  #odialog-container.odialog-container {
    position: fixed;
    display: flex;
    align-items: center;
    justify-content: center;
  }
  .modal-container {
    position: fixed;
    top: 0;
    right: 0;
    bottom: 0;
    left: 0;
    z-index: 100;
    width: 100%;
    height: 100%;
    text-align: center;
    background-color: #57575788;
  }
  .modal {
    display: inline-block;
    width: 30%;
    padding: 10px;
    margin: 40vh auto;
    color: var(--text);
    text-align: center;
    background-color: var(--background);
    border: 1px solid var(--border);
    border-radius: 10px;
  }
  .modal h2 {
    margin-top: 0;
  }
  .modal-message-extra {
    white-space: pre-wrap;
  }
</style>
