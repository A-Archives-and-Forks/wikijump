<script lang="ts">
  import { page } from "$app/stores"
  import { invalidateAll } from "$app/navigation"
  import { onMount } from "svelte"
  import { useErrorPopup, usePagePaneState } from "$lib/stores"
  import { PagePane } from "$lib/types"
  let showErrorPopup = useErrorPopup()
  let pagePaneState = usePagePaneState()
  let parents = ""

  async function setParents() {
    let form = document.getElementById("page-parent")
    let fdata = new FormData(form)
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    let newParents = (fdata.get("parents")?.toString() ?? "").split(" ").filter((p) => p)
    let oldParents = parents.split(" ").filter((p) => p)
    let added: string[] = []
    let removed: string[] = []
    let common: string[] = []
    for (let i = 0; i < oldParents.length; i++) {
      if (!newParents.includes(oldParents[i])) removed.push(oldParents[i])
      else common.push(oldParents[i])
    }
    for (let i = 0; i < newParents.length; i++) {
      if (!common.includes(newParents[i])) added.push(newParents[i])
    }
    if (added.length) fdata.set("add-parents", added.join(" "))
    if (removed.length) fdata.set("remove-parents", removed.join(" "))

    let res = await fetch(`/${$page.data.page.slug}/parent-set`, {
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
      pagePaneState.set(PagePane.None)
      invalidateAll()
    }
  }

  onMount(async () => {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    let res = await fetch(`/${$page.data.page.slug}/parent-get`, {
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
      parents = res.join(" ")
    }
  })
</script>

<form
  id="page-parent"
  class="page-parent"
  method="POST"
  on:submit|preventDefault={setParents}
>
  <input
    name="parents"
    class="page-parent-new-parents"
    placeholder={$page.data.internationalization?.parents}
    type="text"
    value={parents}
  />
  <div class="action-row page-parent-actions">
    <button
      class="action-button page-parent-button button-cancel clickable"
      type="button"
      on:click|stopPropagation={() => {
        pagePaneState.set(PagePane.None)
      }}
    >
      {$page.data.internationalization?.cancel}
    </button>
    <button
      class="action-button page-parent-button button-save clickable"
      type="submit"
      on:click|stopPropagation
    >
      {$page.data.internationalization?.save}
    </button>
  </div>
</form>

<style lang="scss">
  .page-parent {
    display: flex;
    flex-direction: column;
    gap: 15px;
    align-items: stretch;
    justify-content: stretch;
    width: 100%;
    padding: 0 0 2em;
  }
</style>
