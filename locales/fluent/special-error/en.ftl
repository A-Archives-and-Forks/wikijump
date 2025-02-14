### Special Error HTML templates

special-error-site-slug = <h1>No { -service-name } site exists with this address.</h1>
    <p>
      <a href="https://{ $slug }.{ $domain }/">{ $slug }.{ $domain }</a> does not exist.
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

special-error-site-custom = <h1>No { -service-name } site exists with this address.</h1>
    <p>
      No site has the custom domain <a href="https://{ $custom_domain }/">{ $custom_domain }</a>.
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>

special-error-site-fetch = <h1>Unable to fetch site information.</h1>
    <p>
      { -service-name } was unable to retrieve information about <code>{ $domain }</code>.
    </p>

    <p>
      Return to <a href="https://{ $main_domain }/">{ -service-name }</a>.
    </p>
