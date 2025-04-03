### Special Error HTML templates

special-error-site-slug = <h1>No { -service-name } site exists with this address.</h1>
    <p>
      <a href="https://{ $slug }.{ $main_domain }/">{ $slug }.{ $main_domain }</a> does not exist.
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

    .title = No such { -service-name } site

special-error-site-custom = <h1>No { -service-name } site exists with this address.</h1>
    <p>
      No site has the custom domain <a href="https://{ $custom_domain }/">{ $custom_domain }</a>.
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

    .title = No such { -service-name } site

special-error-site-fetch = <h1>Unable to fetch site information.</h1>
    <p>
      { -service-name } was unable to retrieve information about <code>{ $domain }</code>.
    </p>

    <p>
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

    .title = Unable to fetch { -service-name } information

special-error-file-root = <h1>Invalid route</h1>
    <p>
      { -service-name } serves user-generated data from { $files_domain }, but this is not a valid URL.
    </p>

    <p>
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

    .title = { -service-name }
