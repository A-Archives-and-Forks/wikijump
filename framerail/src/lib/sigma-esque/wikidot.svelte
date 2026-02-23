<div id="skrollr-body">
  <!-- svelte-ignore a11y-missing-content -->
  <a name="page-top"></a>
  <div id="container-wrap-wrap">
    <div id="container-wrap">
      <div id="container">
        <div id="header">
          <slot name="header"></slot>
          <div id="top-bar">
            <slot name="top-bar"></slot>
          </div>
        </div>
        <div id="content-wrap">
          <div id="side-bar">
            <slot name="side-bar"></slot>
          </div>
          <div id="main-content">
            <div id="action-area-top"></div>
            <slot name="content"></slot>
          </div>
        </div>
        <div id="footer">
          <slot name="footer"></slot>
        </div>
        <div id="license-area" class="license-area">
          <slot name="license"></slot>
        </div>
        <div id="extrac-div-1"><span></span></div>
        <div id="extrac-div-2"><span></span></div>
        <div id="extrac-div-3"><span></span></div>
      </div>
      <div id="extra-div-1"><span></span></div>
      <div id="extra-div-2"><span></span></div>
      <div id="extra-div-3"><span></span></div>
      <div id="extra-div-4"><span></span></div>
      <div id="extra-div-5"><span></span></div>
      <div id="extra-div-6"><span></span></div>
    </div>
  </div>
</div>

<!-- Ignoring the styling as being a theme it will inevitably style other elements in the entire layout -->
<!-- svelte-ignore css-unused-selector -->
<style global lang="scss">
  $tablet-max-width: 767px;

  :root {
    /* Fallback colors */
    --text: #111;
    --background: #fff;
    --border: #0006;
    --mild-text: #f0f0f0;
    --accent: #0066cc;
    --error: #b42d0a;
    --col-accent-1: #19a9ff;
    --col-accent-2: #0068b5;
  }

  .clickable {
    cursor: pointer;
    user-select: none;

    &:disabled {
      cursor: auto;
    }
  }

  .hidden {
    display: none;
  }

  @media (prefers-color-scheme: light) {
    :root {
      /* Fallback colors */
      --text: #111;
      --background: #fff;
      --border: #0006;
      --mild-text: #f0f0f0;
      --accent: #0066cc;
    }
  }

  @media (prefers-color-scheme: dark) {
    :root {
      /* Fallback colors */
      --text: #b6c2cf;
      --background: #222;
      --border: #b6c2cf;
      --mild-text: #bbb;
      --accent: #44aaff;
    }
  }

  @media (max-width: $tablet-max-width) {
    .header {
      height: initial;
    }
  }
</style>
