# Qalam

Swampy static site generator.

## Expected Directory Structure

```
index.toml
projects.toml
posts/{cfg.toml,*.md}
themes/{*.css,syntax/{*.tmTheme}}
img/*
```

## TODO

- [x] MD -> HTML
- [x] Read posts/ dir.
- [x] Projects page; TOML page config & project array -> HTML
  - Would we need an HTML templater like `maud`? Yes, and it's glorious.
- [ ] Parse index.toml for homepage
- [ ] CSS class assignment to home/about-page elements
- [ ] Read MD frontmatter for post sorting
- [ ] Syntax highlighting via `syntect`
  - [ ] Swamp light/dark themes OOTB
  - [ ] Configurable themes as part of cfg.toml
- [ ] Post tagging via MD frontmatter & HTML output
