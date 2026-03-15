<script lang="ts">
  import type { PageData } from "./$types"

  let { data }: { data: PageData } = $props()
</script>

<h1>UNTRANSLATED: Loaded user profile</h1>

<textarea class="debug">{JSON.stringify(data, null, 2)}</textarea>

<div class="user-info" data-id={data.user?.user_id}>
  {#if data.user?.name}
    <h2 class="user-attribute name">
      {data.user.name}
    </h2>
  {/if}

  {#if data.user?.real_name}
    <div class="user-attribute real-name">
      <span class="user-attribute-label"
        >{data.internationalization?.["user-profile-info.real-name"]}</span
      >
      <span class="user-attribute-value">{data.user.real_name}</span>
    </div>
  {/if}

  {#if data.user?.gender}
    <div class="user-attribute gender">
      <span class="user-attribute-label"
        >{data.internationalization?.["user-profile-info.gender"]}</span
      >
      <span class="user-attribute-value">{data.user.gender}</span>
    </div>
  {/if}

  {#if data.user?.avatar}
    <div class="user-attribute avatar">
      <span class="user-attribute-label"
        >{data.internationalization?.["user-profile-info.avatar"]}</span
      >
      <img
        class="user-attribute-value"
        alt={data.internationalization?.avatar}
        src={data.user.avatar}
      />
    </div>
  {/if}

  {#if data.user?.birthday}
    <div class="user-attribute birthday">
      <span class="user-attribute-label"
        >{data.internationalization?.["user-profile-info.birthday"]}</span
      >
      <span class="user-attribute-value">{data.user.birthday}</span>
    </div>
  {/if}

  {#if data.user?.location}
    <div class="user-attribute location">
      <span class="user-attribute-label"
        >{data.internationalization?.["user-profile-info.location"]}</span
      >
      <span class="user-attribute-value">{data.user.location}</span>
    </div>
  {/if}

  {#if data.user?.user_page}
    <div class="user-attribute user-page">
      <span class="user-attribute-label"
        >{data.internationalization?.["user-profile-info.user-page"]}</span
      >
      <span class="user-attribute-value">{data.user.user_page}</span>
    </div>
  {/if}

  {#if data.user?.biography}
    <div class="user-attribute biography">
      <span class="user-attribute-label"
        >{data.internationalization?.["user-profile-info.biography"]}</span
      >
      <span class="user-attribute-value">{data.user.biography}</span>
    </div>
  {/if}

  {#if data.user?.locales?.length}
    <div class="user-attribute locales">
      <span class="user-attribute-label"
        >{data.internationalization?.["user-profile-info.locales"]}</span
      >
      <span class="user-attribute-value">{data.user.locales?.join(", ")}</span>
    </div>
  {/if}
</div>

<style global lang="scss">
  .debug {
    width: 100%;
    height: 60vh;
  }
</style>
