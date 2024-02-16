# kblog
A static site generated for blogs

# Usage
Create the following directory structure:
```
./
├─ content/
│  ├─ posts/
│  ├─ drafs/
├─ config.toml
```

## Config
The `config.toml` file should be a toml file with four values in it:
```
domain = <dns domain for the blog>
blog_name = <name of the blog>
blog_subtitle = <blog subtitle>
author = <author name>
```

## Drafts
Drafts should contain markdown files that represent draft posts.
The filename should be the slug you want to use for the draft, with hypens for spaces.
For example, if you wanted a draft located at http://myblog/drafts/my-great-draft, then the filename should be my-great-draft.md.

## Posts
Posts should contain markdown files that represent published posts.
The filename should be publish data, followed by an underscore, followed the slug you want to use for the posts, with hypens for spaces, i.e. <date>_<slug>.md.
For example, if you wanted a draft located at http://myblog/posts/my-great-draft published on 2024-02-02, then the filename should be 2024-02-02_my-great-post.md.
The first line of the markdown file will be used as the title of the post.

