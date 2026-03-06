<script lang="ts">
  import { page } from "$app/state"
  import { errorPopupState } from "$lib/stores.svelte"
  import { goto } from "$app/navigation"
  import { resolve } from "$app/paths"

  let isLoggedIn = $state<boolean>(page.data.isLoggedIn)

  let password = $state<string>("")
  let confirmPassword = $state<string>("")

  const isMismatch = $derived(
    confirmPassword !== password && confirmPassword.length > 0 && password.length > 0
  )

  let isRegistered = $state<boolean>(false)

  async function tryRegister() {
    const form = document.querySelector<HTMLFormElement>("form#register")
    if (!form) return
    const fdata = new FormData(form)

    const res: {
      user_id: number
      slug: string
    } = await fetch(`/-/register`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())

    if (res.user_id) {
      isRegistered = true
      goto(resolve(`/-/login`, {}))
    } else {
      errorPopupState.current = {
        state: true,
        message: res.message,
        data: res.data
      }
    }
  }

  function validateMatch(event: FocusEvent, isMismatch: boolean): void {
    const target = event.target
    if (target instanceof HTMLInputElement) {
      if (isMismatch) {
        target.setCustomValidity(
          page.data.internationalization?.["error-form.password-mismatch"] ?? ""
        )
      } else {
        target.setCustomValidity("")
      }
    }
  }
</script>

{#if isLoggedIn || isRegistered}
  {page.data.internationalization?.["register.toast"]}
{:else}
  <form
    id="register"
    class="register-form"
    method="POST"
    onsubmit={(event) => {
      event.preventDefault()
      tryRegister()
    }}
  >
    <label for="username">
      {page.data.internationalization?.["username"]}
    </label>
    <div class="input-container">
      <input
        id="username"
        name="username"
        class="username"
        placeholder={page.data.internationalization?.["username.placeholder"]}
        required
        type="text"
      />
      <p class="description">{page.data.internationalization?.["username.info"]}</p>
    </div>

    <label for="email">
      {page.data.internationalization?.["email"]}
    </label>
    <div class="input-container">
      <input
        id="email"
        name="email"
        class="email"
        placeholder={page.data.internationalization?.["email.placeholder"]}
        required
        type="text"
      />
      <p class="description">{page.data.internationalization?.["email.info"]}</p>
    </div>

    <label for="password">
      {page.data.internationalization?.["password"]}
    </label>
    <div class="input-container">
      <input
        name="password"
        class="auth-password"
        placeholder={page.data.internationalization?.["password.placeholder"]}
        required
        type="password"
        bind:value={password}
      />
    </div>

    <label for="confirm-password">
      {page.data.internationalization?.["confirm-password"]}
    </label>
    <div class="input-container">
      <input
        name="confirm-password"
        class="confirm-password"
        onblur={(event) => {
          validateMatch(event, isMismatch)
        }}
        oninput={(event) => {
          const input = event.target as HTMLInputElement
          input.setCustomValidity("")
        }}
        placeholder={page.data.internationalization?.["password.placeholder"]}
        required
        type="password"
        bind:value={confirmPassword}
      />
    </div>

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
        class="action-button auth-button button-create clickable"
        onclick={(event) => {
          event.stopPropagation()
        }}
        type="submit"
      >
        {page.data.internationalization?.["create-account"]}
      </button>
    </div>
  </form>
{/if}

<style lang="scss">
  .register-form {
    display: grid;
    grid-template-columns: auto auto;

    gap: 1rem;
    justify-content: center;

    .input-container {
      .description {
        margin: 0;
        font-size: 0.8rem;
        color: #666;
      }
    }

    .action-row {
      grid-column: span 2;
      justify-self: center;
    }
  }
</style>
