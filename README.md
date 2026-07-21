# webify-cli

A Rust CLI that converts images to **WebP** format and generates a complete **favicon** set (ico, svg, png and web manifest) from a single source image.

## Features

- **`wc`** — Converts a single image or every image in a directory (`.jpg`, `.jpeg`, `.png`) to `.webp` using real lossy encoding (libwebp).
- **`favicon`** — From a single source image, generates:
  - `favicon.ico` (multi-resolution: 48×48, 32×32, 16×16)
  - `favicon.svg` (96×96, base64-embedded PNG inside an SVG)
  - `favicon-96x96.png`
  - `apple-touch-icon.png` (180×180)
  - `web-app-manifest-192x192.png`
  - `web-app-manifest-512x512.png`
  - `site.webmanifest`
  - An HTML snippet ready to paste into the site's `<head>`
- Centralized error handling via a single `AppError` type (using `thiserror`), consistent across every command.
- Layered architecture (`core` / `commands` / `utils`) that makes adding new commands straightforward.

## Requirements

- Rust **1.85+** (the project uses `edition = "2024"`)
- `cargo` (bundled with Rust)

Install Rust if you don't have it yet:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## Installation

Clone the repository and build it:

```bash
git clone https://github.com/ramcodez3/webify-cli-v2.git
cd webify-cli-v2
cargo build --release
```

The binary will be at `target/release/webify`.

### Install it as a global command (optional)

```bash
cargo install --path .
```

This copies the binary to `~/.cargo/bin/webify`, which is normally already in your `PATH` if you installed Rust with `rustup`. After that, you can run `webify` from any directory.

## Project structure

```
webify-cli-v2/
├── README.md
├── Cargo.toml
├── LICENSE
└── src/
    ├── main.rs                 # CLI parsing (clap) and dispatch to commands::*
    ├── commands/
    │   ├── mod.rs
    │   ├── webp.rs              # `wc` command handler
    │   └── favicon.rs           # `favicon` command handler
    ├── core/
    │   ├── mod.rs
    │   ├── webp.rs               # Pure webp conversion logic
    │   ├── favicon.rs             # Pure favicon generation logic
    │   └── favicon_type.rs        # Enums/specs for each favicon format
    └── utils/
        ├── mod.rs
        ├── error.rs               # Centralized AppError
        └── path.rs                 # Path resolution (e.g. ".")
```

## Commands

### `wc` — Convert to WebP

```bash
webify wc <path> [-k | --keep]
```

| Argument / flag | Description |
|---|---|
| `<path>` | Path to a file or a directory. Use `.` for the current directory. |
| `-k`, `--keep` | Keeps the original images instead of deleting them after conversion. |

Examples:

```bash
# Convert a single file (deletes the original)
webify wc photo.jpg

# Convert every image in a directory, keeping the originals
webify wc ./images --keep

# Convert the current directory
webify wc .
```

### `favicon` — Generate favicons

```bash
webify favicon <path> [-n | --name-app <name>] [-d | --destination <path>]
```

| Argument / flag | Description |
|---|---|
| `<path>` | Path to the source image the favicons are generated from. |
| `-n`, `--name-app <name>` | App/site name, used in `site.webmanifest` and the HTML snippet. Defaults to `MyWebSite`. |
| `-d`, `--destination <path>` | Folder where the `favicon/` subfolder with all generated files will be created. Defaults to the source image's own directory. |

Examples:

```bash
# Generate favicons into ./favicon (next to the source image)
webify favicon logo.png

# Specifying the app name
webify favicon logo.png --name-app "My Project"

# Specifying where to save the favicon/ folder
webify favicon logo.png -n "My Project" -d ./public
```

Once finished, the command prints the list of generated files along with an HTML snippet ready to copy into your site's `<head>`.

## License

This project is licensed under the [Apache License 2.0](LICENSE).
