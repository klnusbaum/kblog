# KBlog
A highly-opinionated static site generated for blogs written in Rust.
Used to generate [knusbaum.org](https://knusbaum.org).

# Usage
Create the following directory structure:
```
./
├─ content/
│  ├─ posts/
│  ├─ drafts/
│  ├─ imgs/
├─ config.toml
```
and then simply run the `kblog` command. A full website will be generated in a directory called `gen`.

Note the `-C` option allows you to specify the directory in which the `kblog` command should preform all work.

## Config
The `config.toml` file should be a toml file with following content:
```
[metadata]
domain = <dns domain for the blog>
blog_name = <name of the blog>
blog_subtitle = <blog subtitle>
author = <author name>
links = [
  {name = "GitHub", url = "https://github.com/<mygithub>"},
  {name = "X", url = "https://twitter.com/<mytwitter>"},
]

[themes.light]
background_color = <light_background_color>
text_color = <light_text_color>
link_color = <light_link_color>
footer_color = <light_footer_color>

[themes.dark]
background_color = <dark_background_color>
text_color = <dark_text_color>
link_color = <dark_link_color>
footer_color = <dark_footer_color>
```

## Drafts
Drafts should contain markdown files that represent draft posts and reside in the `drafts` directory.
The filename should be the slug you want to use for the draft, with hypens for spaces.
For example, if you wanted a draft located at http://myblog/drafts/my-great-draft, then the filename should be `my-great-draft.md`.

## Posts
Posts should contain markdown files that represent published posts and reside in the `posts` directory.
The filename should be publish date, followed by an underscore, followed the slug you want to use for the posts, with hypens for spaces, i.e. `<date>_<slug>.md`.
For example, if you wanted a post located at http://myblog/posts/my-great-draft published on 2024-02-02, then the filename should be `2024-02-02_my-great-post.md`.
The first line of the markdown file will be used as the title of the post.
The first paragraph of the post will be used as a summary.

## Images
To include images in drafs/posts, simply put the desired image into the `imgs` directory.
You can then include the image in your post by adding an `img` tag in your markdown, e.g. `<img src="/imgs/my_image_name.png"/>`.
