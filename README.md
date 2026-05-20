# evk-doc

A **Markdown editor** built with [GPUI](https://github.com/zed-industries/gpui) — the Rust GUI framework from Zed.

## Features

- **Split-pane editing** — Edit Markdown on the left, see live preview on the right
- **Syntax highlighting** — Code blocks with language-aware highlighting
- **Resizable panels** — Drag the divider to resize; panel ratio constrained between 15%–85% with a 200px minimum
- **File operations** — Open, save, and save-as `.md` files via system dialogs
- **File tree sidebar** — Open a folder to browse and open Markdown files in a tree view (`Ctrl+B` to toggle)
- **Dark / Light theme** — Toggle between themes with a shortcut
- **Line numbers** — Editor shows line numbers for reference
- **Live preview** — Preview updates in real time as you type
- **Keyboard shortcuts**

| Shortcut | Action |
|----------|--------|
| `Ctrl+O` | Open file |
| `Ctrl+S` | Save file |
| `Ctrl+Shift+S` | Save as… |
| `Ctrl+Shift+O` | Open folder (populate tree) |
| `Ctrl+B` | Toggle file tree sidebar |
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
