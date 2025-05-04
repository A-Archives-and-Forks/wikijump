### Special Error HTML templates

special-error-site-slug = <h1>No { -service-name } site exists with this address</h1>
    <p>
      The site <a href="https://{ $slug }.{ $main_domain }/"><code>{ $slug }.{ $main_domain }</code></a> does not exist.
    </p>

    <p>
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

    .title = No such site - { -service-name }

special-error-site-custom = <h1>No { -service-name } site exists with this address</h1>
    <p>
      No site has the custom domain <a href="https://{ $custom_domain }/"><code>{ $custom_domain }</code></a>.
    </p>

    <p>
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

    .title = No such site - { -service-name }

special-error-page-slug = <h1>This page does not exist</h1>
    <p>
      The page <a href="https://{ $domain }/{ $page_slug }"><code>{ $domain }/{ $page_slug }</code></a> does not exist.
    </p>

    <p>
      Return to the <a href="https://{ $domain }/">site's main page</a>.
    </p>

    .title = No such page - { $domain }

special-error-page-fetch = <h1>Unable to fetch page data</h1>
    <p>
      Server error: Information about page <a href="https://{ $domain }/{ $page_slug }"><code>{ $domain }/{ $page_slug }</code></a> could not be loaded.
    </p>

    <p>
      Return to the <a href="https://{ $domain }/{ $page_slug }">page</a>, or the <a href="https://{ $domain }/">site's main page</a>.
    </p>

    .title = Server Error - { $domain }

special-error-file-root = <h1>Invalid route</h1>
    <p>
      { -service-name } serves user-generated data from <code>{ $files_domain }</code>, but this is not a valid URL.
    </p>

    <p>
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

    .title = { -service-name }
