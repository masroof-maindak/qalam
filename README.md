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
- [ ] Projects page; TOML page config & project array -> HTML
  - Would we need an HTML templater like `maud`?
- [ ] Read MD frontmatter for post sorting
- [ ] Parse index.toml for homepage
- [ ] Syntax highlighting via `syntect`
  - [ ] Swamp light/dark themes OOTB
  - [ ] Configurable themes as part of cfg.toml
- [ ] Post tagging via MD frontmatter & HTML output
