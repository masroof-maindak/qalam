# Qalam

Swampy static site generator.

## Setup

```bash
git clone https://github.com/masroof-maindak/qalam.git
cd qalam
cargo install --path .
```

## Usage

```bash
# After completing the configuration, and ensuring you have the expected
# directory structure...

qalam <dir> # '.' by default
```

## Development

Note that it is essential that the project is run from the program root while in
debug mode (e.g w/ `cargo run <dir>`). This is required because of the splendid
`rust-embed` crate, that allows us to 'bake in' the required theme files into
the output binary.

## Configuration

### `./index.toml`

```toml
page_title = "My Site"
name = "John Doe"
username = "@john-doe"
bio = "Phrase describing you."
email = "john.doe@gmail.com"
github = "https://github.com/your-profile"
desc = "Brief paragraph about you."
footer = "© 2025 John Doe"
theme_name = "swamp-light"
```

### `./posts.toml`

```toml
page_title = "My Site/Posts"
title = "Posts"
desc = "Articles & blog entries"
```

### `./projects.toml`

```toml
page_title = "My Site/Projects"
title = "Projects"
desc = "Stuff I've built"

[[projects]]
name = "qalam"
desc = "Swampy static site generator."
tags = ["Rust", "Web"]
url = "https://github.com/masroof-maindak/qalam"

# As many [[projects]] as you want
```

## Theming

You can change the website's colour theme by setting the following variables
within `./themes/override.css`, in the root of your website's directory.

```css
:root {
  /* Swamp Light */
  --bg: #f1e3d1;
  --text: #64513e;
  --muted: #a0907d;
  --less-muted: #8c7b68;
  --blockquote-fg: #786653;
  --accent: #bf7979;
  --accent2: #9e5581;
  --surface: #ddcebc;
}
```

## Expected Directory Structure

### Input

```txt
.
├── img/
├── posts/
│   └── *.md
├── syntax-themes/
│   └── *.tmTheme
├── themes/
│   └── override.css
├── index.toml
├── posts.toml
└── projects.toml
```

### Output

```txt
build/
├── img/
├── posts/
│   ├── index.html
│   └── *.html
├── projects/
│   └── index.html
├── themes/
│   └── styles.css
└── index.html
```

## TODO

- [x] MD -> HTML
  - Using `pulldown_cmark`, but `comrak` (that I discovered much later) seems to
    be much easier... And also seamlessly integrates w/ `syntect`
- [x] Read posts/ dir.
- [x] Projects page; TOML page config & project array -> HTML
  - Would we need an HTML templater like `maud`? Yes, and it's glorious.
- [x] Parse index.toml for homepage
- [x] Home page HTML
- [x] CSS Class Assignment
  - [x] Home
  - [x] Projects
  - [x] Posts
  - [x] Post entry
- [x] CSS
  - [x] Home
  - [x] Projects
  - [x] Posts
  - [x] Post entry
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
- [x] Copyright notice in footer
- [x] `<dir>` flag -- chdir to given path and build there
- [x] Syntax highlighting via `syntect`
  - [x] CSS - padding
  - [x] Swamp Light `.tmTheme`
    - It's not very good, but I had it lying around from my `bat` config
  - [x] Configurable themes as part of index.toml
- [x] 'Bake in' 'official' CSS & syntax themes, to remove reliance on project
      root existing & comprising 'themes/' + 'syntax-themes/'
  - See: <https://github.com/RMHEDGE/rust-embed>
- [ ] GH Pages deploy action
- [ ] Swamp dark `.tmTheme`
- [ ] Light/dark-mode toggle
- [ ] PR Syntect for line numbers
- [ ] TOC for blog posts
- [ ] Post tagging via MD frontmatter & output HTML tag pages

## LLM Usage Disclosure

- `styles.css`
- Converting my base16 Neovim theme to a `.tmTheme`

## Acknowledgements

- [Isunjn's](https://github.com/isunjn) excellent theme for Zola,
  [serene](https://github.com/isunjn/serene), that I adored, but left solely by
  virtue of the frequency with which it, and Zola itself too, introduced
  breaking changes.
