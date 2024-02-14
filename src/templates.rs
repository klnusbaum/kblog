pub const TOKEN_STYLES: &'static str = "{{style}}";
pub const TOKEN_BODY: &'static str = "{{body}}";
pub const TOKEN_TITLE: &'static str = "{{title}}";
pub const TOKEN_DATE: &'static str = "{{date}}";
pub const TOKEN_CONTENT: &'static str = "{{content}}";
pub const TOKEN_POST_LIST: &'static str = "{{post_list}}";
pub const TOKEN_ANALYTICS_TAG: &'static str = "{{analytics_tag}}";
pub const TOKEN_RSS_FEED: &'static str = "{{feed_file}}";
pub const TOKEN_YEAR: &'static str = "{{year}}";
pub const TOKEN_URL: &'static str = "{{url}}";
pub const TOKEN_OG_TYPE: &'static str = "{{og_type}}";

pub const PAGE_TEMPLATE: &'static str = r#"<!DOCTYPE html>
<html lang="en">
<head>
<meta charset="utf-8">
<meta name="viewport" content="width=device-width, initial-scale=1">

<meta property="og:title" content="{{title}}">
<meta property="og:type" content="{{og_type}}">
<meta property="og:url" content="{{url}}">
<meta property="og:site_name" content="knusbaum.org">

<link href="/{{feed_file}}" type="application/atom+xml" rel="alternate" title="Sitewide Atom feed" />
<title>{{title}}</title>
<link rel="stylesheet" type="text/css" href="/{{style}}"/>
<body>
{{body}}
<hr>
<footer>
<a href="/">Home</a> | <a href="https://github.com/klnusbaum/">GitHub</a> | <a href="/{{feed_file}}">RSS</a><span class="copyright">© {{year}} Kurtis Nusbaum</span>
</footer>

<!-- Google tag (gtag.js) -->
<script async src="https://www.googletagmanager.com/gtag/js?id=G-0CMCYFYD3R"></script>
<script>
  window.dataLayer = window.dataLayer || [];
  function gtag(){dataLayer.push(arguments);}
  gtag('js', new Date());

  gtag('config', '{{analytics_tag}}');
</script>
</body>
</html>
"#;

pub const POST_TEMPLATE: &str = r#"<main>
<h1>{{title}}</h1>
<h4>{{date}}</h4>
{{content}}</main>
"#;

pub const DRAFT_TEMPLATE: &str = r#"<main>
<h1>{{title}}</h1>
{{content}}</main>
"#;

pub const INDEX_TEMPLATE: &str = r#"<header>
<h1>knusbaum.org</h1>
<p>The web log of Kurtis Nusbaum</p>
</header>
<br/>
<main>
<ul>
{{post_list}}</ul>
</main>
"#;
