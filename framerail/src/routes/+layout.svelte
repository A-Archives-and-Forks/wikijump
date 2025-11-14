<script lang="ts">
  import { page } from "$app/stores"
  import SigmaEsque from "$lib/sigma-esque/sigma-esque.svelte"
  import Wikidot from "$lib/sigma-esque/wikidot.svelte"
  import wjBanner from "$assets/logo-outline.min.svg?raw"
  import { useErrorPopup, usePageLayoutState } from "$lib/stores"
  import { Layout } from "$lib/types"
  import ErrorPopup from "$lib/popup/error.svelte"
  let showErrorPopup = useErrorPopup()
  function closeErrorPopup() {
    showErrorPopup.set({
      state: false,
      message: null,
      data: null
    })
  }

  let pageLayout = usePageLayoutState()
  function setLayout() {
    if ($page.route.id.startsWith("/[x+2d]/")) {
      // this is a special page, use Wikijump layout
      $pageLayout = Layout.WIKIJUMP
    } else {
      $pageLayout =
        $page.data?.page?.layout ??
        $page.data?.site?.layout ??
        $page.error?.site?.layout ??
        Layout.WIKIJUMP
    }
  }
  setLayout()
  page.subscribe(() => setLayout())
</script>

{#if $showErrorPopup.state}
  <ErrorPopup exitPrompt={closeErrorPopup} />
{/if}

{#if $pageLayout === Layout.WIKIDOT}
  <style global>
    /* Use Sigma 10 as default Wikidot theme for now */
    @import url("https://d3g0gp89917ko0.cloudfront.net/v--7690939296dc/common--theme/base/css/style.css");
    @import url("https://d3g0gp89917ko0.cloudfront.net/v--7690939296dc/common--modules/css/pagerate/PageRateWidgetModule.css");
    @import url("https://cdn.scpwiki.com/theme/en/sigma/css/sigma.min.css");
  </style>
  <Wikidot>
    <svelte:fragment slot="header">
      <h1>
        <a class="active" href="/"
          ><span>{$page.data?.site?.name ?? $page.error?.site?.name}</span></a
        >
      </h1>
      <h2>
        <span>{$page.data?.site?.tagline ?? $page.error?.site?.tagline}</span>
      </h2>
    </svelte:fragment>

    <svelte:fragment slot="top-bar">UNTRANSLATED: Top bar</svelte:fragment>

    <svelte:fragment slot="side-bar">UNTRANSLATED: Side bar</svelte:fragment>

    <svelte:fragment slot="content">
      <slot />
    </svelte:fragment>

    <svelte:fragment slot="footer">
      <div class="options">
        <a href="/"
          >{$page.data?.internationalization?.docs ??
            $page.error?.internationalization?.docs}</a
        >
        |
        <a href="/"
          >{$page.data?.internationalization?.terms ??
            $page.error?.internationalization?.terms}</a
        >
        |
        <a href="/"
          >{$page.data?.internationalization?.privacy ??
            $page.error?.internationalization?.privacy}</a
        >
        |
        <a href="/"
          >{$page.data?.internationalization?.security ??
            $page.error?.internationalization?.security}</a
        >
      </div>
      <div class="footer-powered-by">
        {$page.data?.internationalization?.["footer-powered-by"] ??
          $page.error?.internationalization?.["footer-powered-by"]}
      </div>
    </svelte:fragment>
  </Wikidot>
{:else}
  <SigmaEsque>
    <svelte:fragment slot="header">
      <div class="header-wjbanner">
        {@html wjBanner}
      </div>
    </svelte:fragment>

    <svelte:fragment slot="top-bar">UNTRANSLATED: Top bar</svelte:fragment>

    <svelte:fragment slot="content">
      <slot />
    </svelte:fragment>

    <svelte:fragment slot="footer">
      <div class="footer-inner">
        <ul class="footer-items">
          <li class="footer-item">
            <a href="/"
              >{$page.data?.internationalization?.terms ??
                $page.error?.internationalization?.terms}</a
            >
          </li>
          <li class="footer-item">
            <a href="/"
              >{$page.data?.internationalization?.privacy ??
                $page.error?.internationalization?.privacy}</a
            >
          </li>
          <li class="footer-item">
            <a href="/"
              >{$page.data?.internationalization?.docs ??
                $page.error?.internationalization?.docs}</a
            >
          </li>
          <li class="footer-item">
            <a href="/"
              >{$page.data?.internationalization?.security ??
                $page.error?.internationalization?.security}</a
            >
          </li>
        </ul>
        <div class="footer-powered-by">
          {$page.data?.internationalization?.["footer-powered-by"] ??
            $page.error?.internationalization?.["footer-powered-by"]}
        </div>
      </div>
    </svelte:fragment>
  </SigmaEsque>
{/if}

<!-- Ignoring the "unused" svg as we know we imported and embedded a raw svg -->
<!-- svelte-ignore css-unused-selector -->
<style global lang="scss">
  $tablet-max-width: 767px;

  .header-wjbanner {
    height: 80%;
    color: #fff;

    svg {
      width: auto;
      height: 100%;
    }
  }

  .footer-inner {
    display: flex;
    flex-direction: row;
    gap: 10px;
    align-items: center;
    justify-content: stretch;
    width: 100%;
  }

  .footer-items {
    display: flex;
    flex: 1;
    flex-direction: row;
    gap: 10px;
    align-items: center;
    justify-content: flex-start;
    padding: 0;
    list-style: none;

    .footer-item a {
      color: #fff;
      text-decoration: none;
    }
  }

  @media (max-width: $tablet-max-width) {
    .header-wjbanner {
      text-align: center;

      svg {
        height: initial;
        max-height: 6.5em;
      }
    }
  }
</style>
