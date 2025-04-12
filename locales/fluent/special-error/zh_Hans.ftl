### 特殊错误 HTML 模板

special-error-site-slug = <h1>该网址不存在 { -service-name } 站点。</h1>
    <p>
      <a href="https://{ $slug }.{ $main_domain }/"><code>{ $slug }.{ $main_domain }</code></a> 不存在。
    </p>

    <p>
      返回 <a href="https://{ $main_domain }/">{ -service-name }</a>。
    </p>

    .title = 站点不存在 - { -service-name }

special-error-site-custom = <h1>该网址不存在 { -service-name } 站点。</h1>
    <p>
      没有网站使用此自定义域名 <a href="https://{ $custom_domain }/"><code>{ $custom_domain }</code></a>。
    </p>

    <p>
      返回 <a href="https://{ $main_domain }/">{ -service-name }</a>。
    </p>

    .title = 站点不存在 - { -service-name }

special-error-file-root = <h1>无效路径</h1>
    <p>
      { -service-name }  于 <code>{ $files_domain }</code> 提供用户生成的数据，但这不是有效的URL。
    </p>

    <p>
      返回 <a href="https://{ $main_domain }/">{ -service-name }</a>。
    </p>

    .title = { -service-name }
