### Special Error HTML templates

special-error-site-slug = <h1>No { -service-name } site exists with this address.</h1>
    <p>
      <a href="https://{ $slug }.{ $main_domain }/"><code>{ $slug }.{ $main_domain }</code></a> does not exist.
    </p>

    <p>
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

    .title = No such site - { -service-name }

special-error-site-custom = <h1>No { -service-name } site exists with this address.</h1>
    <p>
      No site has the custom domain <a href="https://{ $custom_domain }/"><code>{ $custom_domain }</code></a>.
    </p>

    <p>
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

    .title = No such site - { -service-name }

special-error-site-fetch = <h1>Unable to fetch site information.</h1>
    <p>
      { -service-name } was unable to retrieve information about <code>{ $domain }</code>.
    </p>

    <p>
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

    .title = Unable to fetch information - { -service-name }

special-error-file-root = <h1>Invalid route</h1>
    <p>
      { -service-name } serves user-generated data from <code>{ $files_domain }</code>, but this is not a valid URL.
    </p>

    <p>
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

    .title = { -service-name }
