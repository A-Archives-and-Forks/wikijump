<script lang="ts">
  import { page } from "$app/state"
  import { errorPopupState, pageLayoutState } from "$lib/stores.svelte"
  import { Layout } from "$lib/types"

  let showVoteList = $state<boolean>(false)
  let voteMap = $state<Map<number, Record<string, any>>>(new Map())
  let voteRating = $state<number>()

  async function getVoteList() {
    const fdata = new FormData()
    fdata.set("site-id", page.data.site.site_id)
    fdata.set("page-id", page.data.page.page_id)
    const res = await fetch(`/${page.data.page.slug}/vote-get`, {
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
      voteMap.clear()
      res.forEach((vote) => {
        voteMap.set(vote.user_id, vote)
      })
    }
  }

  async function castVote(value?: number) {
    const fdata = new FormData()
    fdata.set("site-id", page.data.site.site_id)
    fdata.set("page-id", page.data.page.page_id)
    fdata.set("value", value ?? 0)
    const res = await fetch(`/${page.data.page.slug}/vote-cast`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      errorPopupState.current = {
        state: true,
        message: res.message,
        data: res.data
      }
    }
  }

  async function cancelVote() {
    const fdata = new FormData()
    fdata.set("site-id", page.data.site.site_id)
    fdata.set("page-id", page.data.page.page_id)
    const res = await fetch(`/${page.data.page.slug}/vote-cancel`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      errorPopupState.current = {
        state: true,
        message: res.message,
        data: res.data
      }
    }
  }

  $effect(() => {
    async function fetchVoteRating() {
      let fdata = new FormData()
      fdata.set("site-id", page.data.site.site_id)
      fdata.set("page-id", page.data.page.page_id)
      let res = await fetch(`/${page.data.page.slug}/score`, {
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
        voteRating = res.score ?? 0
      }
    }
    fetchVoteRating()
  })
</script>

{#if pageLayoutState.current === Layout.WIKIJUMP}
  <h1 class="page-vote-header">
    {page.data.internationalization?.["wiki-page-vote"]}
  </h1>
  <div class="page-rate-widget-area">
    <div class="page-rate-widget-box">
      <span class="rate-points">
        {page.data.internationalization?.["wiki-page-vote.score"]}&nbsp;
        <span class="number prw54353">{voteRating}</span>
      </span>
      <span class="rateup btn btn-default">
        <!-- svelte-ignore a11y_invalid_attribute -->
        <a
          href="javascript:;"
          onclick={(event) => {
            event.stopPropagation()
            castVote(1)
          }}
          type="button">+</a
        >
      </span>
      <span class="ratedown btn btn-default">
        <!-- svelte-ignore a11y_invalid_attribute -->
        <a
          href="javascript:;"
          onclick={(event) => {
            event.stopPropagation()
            castVote(-1)
          }}
          type="button">-</a
        >
      </span>
      <span class="cancel btn btn-default">
        <!-- svelte-ignore a11y_invalid_attribute -->
        <a
          href="javascript:;"
          onclick={(event) => {
            event.stopPropagation()
            cancelVote()
          }}
          type="button">x</a
        >
      </span>
    </div>
  </div>
  <p>
    <!-- svelte-ignore a11y_invalid_attribute -->
    <a
      href="javascript:;"
      onclick={(event) => {
        event.stopPropagation()
        getVoteList().then(() => {
          showVoteList = true
        })
      }}
    >
      {page.data.internationalization?.["wiki-page-vote.list"]}
    </a>
  </p>
  {#if showVoteList}
    <ul class="vote-list">
      {#each [...voteMap].sort((a, b) => b[0] - a[0]) as [, vote] (vote.page_vote_id)}
        <li class="vote-item" data-id={vote.page_vote_id} data-user-id={vote.user_id}>
          UT: User {vote.user_id}: {vote.value}
        </li>
      {/each}
    </ul>
  {/if}
{:else}
  <h2 class="page-vote-header">
    {page.data.internationalization?.["wiki-page-vote"]}
  </h2>
  <div class="vote-panel">
    <div class="action-row vote-action">
      <button
        class="action-button view-vote-list clickable"
        onclick={(event) => {
          event.stopPropagation()
          getVoteList().then(() => {
            showVoteList = true
          })
        }}
        type="button"
      >
        {page.data.internationalization?.["wiki-page-vote.list"]}
      </button>
      <div class="action-button vote-rating">
        <span class="vote-desc">
          {page.data.internationalization?.["wiki-page-vote.score"]}
        </span>
        <span class="vote-rating-number">{voteRating}</span>
      </div>
      <div class="action-button cast-vote">
        <span class="vote-desc">
          {page.data.internationalization?.["wiki-page-vote.set"]}
        </span>
        <button
          class="vote-subbutton clickable"
          onclick={(event) => {
            event.stopPropagation()
            castVote(1)
          }}
          type="button"
        >
          +1
        </button>
        <button
          class="vote-subbutton clickable"
          onclick={(event) => {
            event.stopPropagation()
            castVote(0)
          }}
          type="button"
        >
          0
        </button>
        <button
          class="vote-subbutton clickable"
          onclick={(event) => {
            event.stopPropagation()
            castVote(-1)
          }}
          type="button"
        >
          -1
        </button>
      </div>
      <button
        class="action-button remove-vote clickable"
        onclick={(event) => {
          event.stopPropagation()
          cancelVote()
        }}
        type="button"
      >
        {page.data.internationalization?.["wiki-page-vote.remove"]}
      </button>
    </div>
    {#if showVoteList}
      <ul class="vote-list">
        {#each [...voteMap].sort((a, b) => b[0] - a[0]) as [, vote] (vote.page_vote_id)}
          <li class="vote-item" data-id={vote.page_vote_id} data-user-id={vote.user_id}>
            UNTRANSLATED: User {vote.user_id}: {vote.value}
          </li>
        {/each}
      </ul>
    {/if}
  </div>
{/if}

<style global lang="scss">
  .page-rate-widget-area {
    text-align: center;
  }
</style>
