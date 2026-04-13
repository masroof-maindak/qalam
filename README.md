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
  - [ ] Use for post sorting via date
  - [ ] Prepend to page's HTML
- [ ] CSS for post entry
- [ ] Syntax highlighting via `syntect`
  - [ ] Swamp light/dark themes OOTB
  - [ ] Configurable themes as part of index.toml
- [ ] flags
  - [ ] `<path>` -- chdir to given path and build there; '.' by default
  - [ ] `build` -- default argument; self-explanatory
  - [ ] `serve` -- watches changes and re-builds, while serving over HTTP
- [ ] TOC for blog posts
- [ ] Post tagging via MD frontmatter & output HTML tag pages