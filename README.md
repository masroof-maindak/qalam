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
│   └── *.css
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
└── index.html
```

## TODO

- [x] MD -> HTML
- [x] Read posts/ dir.
- [x] Projects page; TOML page config & project array -> HTML
  - Would we need an HTML templater like `maud`? Yes, and it's glorious.
- [x] Parse index.toml for homepage
- [ ] Home page HTML
- [ ] CSS class assignment to homepage elements
- [ ] Read MD frontmatter for post sorting on posts/index.html
- [ ] Syntax highlighting via `syntect`
  - [ ] Swamp light/dark themes OOTB
  - [ ] Configurable themes as part of cfg.toml
- [ ] Post tagging via MD frontmatter & HTML output
