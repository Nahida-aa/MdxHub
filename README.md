# evk-doc

A **Markdown editor** built with [GPUI](https://github.com/zed-industries/gpui) — the Rust GUI framework from Zed.

## Features

- **Split-pane editing** — Edit Markdown on the left, see live preview on the right
- **Syntax highlighting** — Code blocks with language-aware highlighting
- **Resizable panels** — Drag the divider to adjust panel widths
- **File operations** — Open and save `.md` files
- **Dark / Light theme** — Toggle between themes with a shortcut
- **Keyboard shortcuts**

| Shortcut | Action |
|----------|--------|
| `Ctrl+O` | Open file |
| `Ctrl+S` | Save file |
| `Ctrl+Shift+S` | Save as… |
| `Ctrl+T` | Toggle theme |

## Getting Started

```bash
# Run in development mode
cargo run

# Build for release
cargo build --release
```

### Prerequisites

- Rust toolchain (edition 2021)

## Project Structure

```
evk-doc/
├── Cargo.toml          # Workspace manifest
├── crates/
│   └── evk-doc/        # Main application crate
│       ├── Cargo.toml
│       └── src/
│           ├── main.rs  # Entry point
│           ├── app.rs   # MarkdownApp — UI & logic
│           └── theme.rs # Theme initialization
```

## Built With

- [GPUI](https://github.com/zed-industries/gpui) — Rust GUI framework
- [gpui-component](https://crates.io/crates/gpui-component) — UI components (Input, TextView, Theme)
