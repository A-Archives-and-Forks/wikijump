### ???

special-error-site-slug = <h1>使用 { -service-name } 作名字的站点并不存在。</h1>
    <p>
      <a href="https://{ $slug }.{ $domain }/">{ $slug }.{ $domain }</a> 并不存在。
      返回 <a href="https://{ $domain }/">{ -service-name }</a>。
    </p>

special-error-site-custom = <h1>使用 { -service-name } 作名字的站点并不存在。</h1>
    <p>
      没有站点使用自订域名 <a href="https://{ $custom_domain }/">{ $custom_domain }</a>。
      返回 <a href="https://{ $main_domain }/">{ -service-name }</a>。
    </p>
