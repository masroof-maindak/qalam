# Qalam

Swampy static site generator.

## Expected Directory Structure

### Input

```
.
├── img/
├── posts/
│   └── *.md
├── themes/
│   ├── override.css
│   └── style.css
├── index.toml
├── posts.toml
└── projects.toml
```

### Output

```
build/
├── posts/
│   ├── index.html
│   └── *.html
├── projects/
│   └── index.html
├── themes/
│   └── style.css
└── index.html
```

## TODO

- [x] MD -> HTML
- [x] Read posts/ dir.
- [x] Projects page; TOML page config & project array -> HTML
  - Would we need an HTML templater like `maud`? Yes, and it's glorious.
- [x] Parse index.toml for homepage
- [x] Home page HTML
- [x] CSS class assignment to homepage elements
- [ ] CSS for home page
- [ ] CSS class assignment to projects/post pages
- [ ] CSS for posts/projects page
- [x] Read MD frontmatter
  - Maintain state? Or better yet -- just re-scan everything honestly (for now)
    - No, let's keep all note-relevant state inside a dedicated struct
      comprising a map
    - This has the added benefit of allowing us to update only the relevant file
      when `serve`-ing later on
    - State management be damned; there are way too many points of failure
  - [x] Prepend to page's HTML
  - [x] Use to present posts, \*, on posts/index.html
    - [x] sorted by date
- [ ] CSS for post entry
- [ ] Syntax highlighting via `syntect`
  - [ ] Swamp light/dark themes OOTB
  - [ ] Configurable themes as part of index.toml
- [x] `<dir>` flag -- chdir to given path and build there
- [ ] TOC for blog posts
- [ ] Post tagging via MD frontmatter & output HTML tag pages
