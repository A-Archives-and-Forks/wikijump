<script lang="ts">
  import { page } from "$app/stores"
  import { onMount } from "svelte"
  import { useErrorPopup, usePageLayoutState } from "$lib/stores"
  import { Layout } from "$lib/types"
  let showErrorPopup = useErrorPopup()
  let pageLayout = usePageLayoutState()
  let showVoteList = false
  let voteMap: Map<number, Record<string, any>> = new Map()
  let voteRating: number

  async function getVoteList() {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    let res = await fetch(`/${$page.data.page.slug}/vote-get`, {
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
      voteMap = new Map()
      res.forEach((vote) => {
        voteMap.set(vote.user_id, vote)
      })
    }
  }

  async function castVote(value?: number) {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    fdata.set("value", value ?? 0)
    let res = await fetch(`/${$page.data.page.slug}/vote-cast`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      showErrorPopup.set({
        state: true,
        message: res.message,
        data: res.data
      })
    }
  }

  async function cancelVote() {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    let res = await fetch(`/${$page.data.page.slug}/vote-cancel`, {
      method: "POST",
      body: fdata
    }).then((res) => res.json())
    if (res?.message) {
      showErrorPopup.set({
        state: true,
        message: res.message,
        data: res.data
      })
    }
  }

  onMount(async () => {
    let fdata = new FormData()
    fdata.set("site-id", $page.data.site.site_id)
    fdata.set("page-id", $page.data.page.page_id)
    let res = await fetch(`/${$page.data.page.slug}/score`, {
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
      voteRating = res.score ?? 0
    }
  })
</script>

{#if $pageLayout === Layout.WIKIDOT}
  <h1 class="page-vote-header">
    {$page.data.internationalization?.["wiki-page-vote"]}
  </h1>
  <div class="page-rate-widget-area">
    <div class="page-rate-widget-box">
      <span class="rate-points"
        >{$page.data.internationalization?.["wiki-page-vote-score"]}&nbsp;<span
          class="number prw54353">{voteRating}</span
        ></span
      ><span class="rateup btn btn-default"
        ><!-- svelte-ignore a11y-invalid-attribute --><a
          href="javascript:;"
          type="button"
          on:click|stopPropagation={() => castVote(1)}>+</a
        ></span
      ><span class="ratedown btn btn-default"
        ><!-- svelte-ignore a11y-invalid-attribute --><a
          href="javascript:;"
          type="button"
          on:click|stopPropagation={() => castVote(-1)}>-</a
        ></span
      ><span class="cancel btn btn-default"
        ><!-- svelte-ignore a11y-invalid-attribute --><a
          href="javascript:;"
          type="button"
          on:click|stopPropagation={cancelVote}>x</a
        ></span
      >
    </div>
  </div>
  <p>
    <!-- svelte-ignore a11y-invalid-attribute -->
    <a
      href="javascript:;"
      on:click|stopPropagation={() => {
        getVoteList().then(() => {
          showVoteList = true
        })
      }}
    >
      {$page.data.internationalization?.["wiki-page-vote-list"]}
    </a>
  </p>
  {#if showVoteList}
    <ul class="vote-list">
      {#each [...voteMap].sort((a, b) => b[0] - a[0]) as [_, vote] (vote.page_vote_id)}
        <li class="vote-item" data-id={vote.page_vote_id} data-user-id={vote.user_id}>
          UT: User {vote.user_id}: {vote.value}
        </li>
      {/each}
    </ul>
  {/if}
{:else}
  <h2 class="page-vote-header">
    {$page.data.internationalization?.["wiki-page-vote"]}
  </h2>
  <div class="vote-panel">
    <div class="action-row vote-action">
      <button
        class="action-button view-vote-list clickable"
        type="button"
        on:click|stopPropagation={() => {
          getVoteList().then(() => {
            showVoteList = true
          })
        }}
      >
        {$page.data.internationalization?.["wiki-page-vote-list"]}
      </button>
      <div class="action-button vote-rating">
        <span class="vote-desc"
          >{$page.data.internationalization?.["wiki-page-vote-score"]}</span
        >
        <span class="vote-rating-number">{voteRating}</span>
      </div>
      <div class="action-button cast-vote">
        <span class="vote-desc"
          >{$page.data.internationalization?.["wiki-page-vote-set"]}</span
        >
        <button
          class="vote-subbutton clickable"
          type="button"
          on:click|stopPropagation={() => castVote(1)}
        >
          +1
        </button>
        <button
          class="vote-subbutton clickable"
          type="button"
          on:click|stopPropagation={() => castVote(0)}
        >
          0
        </button>
        <button
          class="vote-subbutton clickable"
          type="button"
          on:click|stopPropagation={() => castVote(-1)}
        >
          -1
        </button>
      </div>
      <button
        class="action-button remove-vote clickable"
        type="button"
        on:click|stopPropagation={cancelVote}
      >
        {$page.data.internationalization?.["wiki-page-vote-remove"]}
      </button>
    </div>
    {#if showVoteList}
      <ul class="vote-list">
        {#each [...voteMap].sort((a, b) => b[0] - a[0]) as [_, vote] (vote.page_vote_id)}
          <li class="vote-item" data-id={vote.page_vote_id} data-user-id={vote.user_id}>
            UT: User {vote.user_id}: {vote.value}
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
