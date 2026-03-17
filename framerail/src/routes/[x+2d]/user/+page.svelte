<script lang="ts">
  import Page from "./[slug]/page.svelte"

  import { errorPopupState } from "$lib/stores.svelte"
  import { invalidateAll } from "$app/navigation"
  import { fileProxy, superForm } from "sveltekit-superforms"
  import { untrack } from "svelte"

  import type { PageProps } from "./$types"

  let { data }: PageProps = $props()

  let isEdit = $state<boolean>(false)

  const { form, enhance } = superForm(
    untrack(() => data.userEditForm),
    {
      onResult: async ({ result }) => {
        if (result.type === "success" && result.data) {
          isEdit = false
          await invalidateAll()
        }
        if (result.type === "failure" && result.data) {
          errorPopupState.current = {
            state: true,
            message: result.data?.message,
            data: result.data?.data
          }
        }
      }
    }
  )
  const avatar = fileProxy(form, "avatar")

  $form.name = untrack(() => data.user?.name ?? "")
  $form.realName = untrack(() => data.user?.real_name ?? "")
  $form.email = untrack(() => data.user?.email ?? "")
  $form.gender = untrack(() => data.user?.gender ?? "")
  $form.birthday = untrack(() => data.user?.birthday ?? "")
  $form.location = untrack(() => data.user?.location ?? "")
  $form.userPage = untrack(() => data.user?.user_page ?? "")
  $form.biography = untrack(() => data.user?.biography ?? "")
  $form.locales = untrack(() => data.user?.locales?.join(" ") ?? "")
</script>

{#if isEdit}
  <h1>UNTRANSLATED: Loaded user profile</h1>

  <textarea class="debug">{JSON.stringify(data, null, 2)}</textarea>

  <form
    id="editor"
    class="editor"
    action="?/userEdit"
    enctype="multipart/form-data"
    method="POST"
    use:enhance
  >
    <label for="name">{data.internationalization?.["user-profile-info.name"]}</label>
    <input name="name" class="user-attribute name" type="text" bind:value={$form.name} />
    <label for="real-name"
      >{data.internationalization?.["user-profile-info.real-name"]}</label
    >
    <input
      name="realName"
      class="user-attribute real-name"
      type="text"
      bind:value={$form.realName}
    />
    <label for="email">{data.internationalization?.["user-profile-info.email"]}</label>
    <input
      name="email"
      class="user-attribute email"
      type="text"
      bind:value={$form.email}
    />
    <label for="avatar">{data.internationalization?.["user-profile-info.avatar"]}</label>
    <input
      name="avatar"
      class="user-attribute avatar"
      accept="image/png,image/jpeg,image/bmp"
      type="file"
      bind:files={$avatar}
    />
    <label for="gender">{data.internationalization?.["user-profile-info.gender"]}</label>
    <input
      name="gender"
      class="user-attribute gender"
      type="text"
      bind:value={$form.gender}
    />
    <label for="birthday"
      >{data.internationalization?.["user-profile-info.birthday"]}</label
    >
    <input
      name="birthday"
      class="user-attribute birthday"
      type="date"
      bind:value={$form.birthday}
    />
    <label for="location"
      >{data.internationalization?.["user-profile-info.location"]}</label
    >
    <input
      name="location"
      class="user-attribute location"
      type="text"
      bind:value={$form.location}
    />
    <label for="user-page"
      >{data.internationalization?.["user-profile-info.user-page"]}</label
    >
    <input
      name="userPage"
      class="user-attribute user-page"
      type="text"
      bind:value={$form.userPage}
    />
    <label for="biography"
      >{data.internationalization?.["user-profile-info.biography"]}</label
    >
    <input
      name="biography"
      class="user-attribute biography"
      type="text"
      bind:value={$form.biography}
    />
    <label for="locales">{data.internationalization?.["user-profile-info.locales"]}</label
    >
    <input
      name="locales"
      class="user-attribute locales"
      type="text"
      bind:value={$form.locales}
    />
    <div class="action-row editor-actions">
      <button
        class="action-button editor-button button-cancel clickable"
        onclick={() => (isEdit = false)}
        type="button"
      >
        {data.internationalization?.cancel}
      </button>
      <button class="action-button editor-button button-save clickable" type="submit">
        {data.internationalization?.save}
      </button>
    </div>
  </form>
{:else}
  <Page {data} />

  <div class="action-row editor-actions">
    <button
      class="action-button editor-button button-edit clickable"
      onclick={() => (isEdit = true)}
      type="button"
    >
      {data.internationalization?.edit}
    </button>
  </div>
{/if}
