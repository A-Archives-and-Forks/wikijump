<script lang="ts">
  import { page } from "$app/state"
  import { invalidateAll } from "$app/navigation"
  import { errorPopupState } from "$lib/stores.svelte"

  let isLoggedIn = $state<boolean>(page.data.isLoggedIn)

  async function tryLogin() {
    const form = document.querySelector<HTMLFormElement>("form#login")
    if (!form) return
    const fdata = new FormData(form)
    const res = await fetch(`/-/login`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())

    if (res.session_token) {
      isLoggedIn = true
      invalidateAll()
    } else {
      errorPopupState.current = {
        state: true,
        message: res.message,
        data: res.data
      }
    }
  }
</script>

{#if isLoggedIn}
  {page.data.internationalization?.["login.toast"]}
{:else}
  <form
    id="login"
    class="login-form"
    method="POST"
    onsubmit={(event) => {
      event.preventDefault()
      tryLogin()
    }}
  >
    <input
      name="name-or-email"
      class="auth-name-or-email"
      placeholder={page.data.internationalization?.specifier}
      type="text"
    />
    <input
      name="password"
      class="auth-password"
      placeholder={page.data.internationalization?.password}
      type="password"
    />
    <div class="action-row auth-actions">
      <button
        class="action-button auth-button button-cancel clickable"
        onclick={(event) => {
          event.stopPropagation()
        }}
        type="button"
      >
        {page.data.internationalization?.cancel}
      </button>
      <button
        class="action-button auth-button button-login clickable"
        onclick={(event) => {
          event.stopPropagation()
        }}
        type="submit"
      >
        {page.data.internationalization?.login}
      </button>
    </div>
  </form>
{/if}

<style lang="scss">
  .login-form {
    display: flex;
    flex-direction: column;
    gap: 1em;
    align-items: center;
    justify-content: center;
  }
</style>
