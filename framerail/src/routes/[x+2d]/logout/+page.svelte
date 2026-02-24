<script lang="ts">
  import { page } from "$app/state"
  import { invalidateAll } from "$app/navigation"

  let isLoggedIn = $state<boolean>(page.data.isLoggedIn)

  async function tryLogout() {
    let res = await fetch(`/-/logout`, {
      method: "DELETE"
    }).then((res) => res.ok)

    if (res) {
      isLoggedIn = false
      invalidateAll()
    }
  }
</script>

{#if isLoggedIn}
  <div class="action-row auth-actions">
    <button
      class="action-button auth-button button-logout clickable"
      onclick={tryLogout}
      type="button"
    >
      {page.data.internationalization?.logout}
    </button>
  </div>
{:else}
  {page.data.internationalization?.["logout.toast"]}
{/if}
