use crate::app::MarkdownApp;
use gpui::*;
use gpui_component::{
    list::ListItem,
    tree::{Tree, TreeEntry, TreeItem, TreeState},
};
use std::path::{Path, PathBuf};

pub fn is_markdown(path: &Path) -> bool {
    matches!(
        path.extension().and_then(|e| e.to_str()),
        Some("md" | "markdown")
    )
}

pub fn scan_directory(dir: &Path) -> Vec<TreeItem> {
    let mut items = Vec::new();
    if let Ok(entries) = std::fs::read_dir(dir) {
        let mut entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
        entries.sort_by_key(|e| {
            let dir_first = !e.path().is_dir();
            let name = e.file_name().to_string_lossy().to_string();
            (dir_first, name)
        });
        for entry in entries {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with('.') {
                continue;
            }
            if path.is_dir() {
                let children = scan_directory(&path);
                items.push(TreeItem::new(path.display().to_string(), name).children(children));
            } else if is_markdown(&path) {
                items.push(TreeItem::new(path.display().to_string(), name));
            }
        }
    }
    items
}

pub fn open_folder(window: &mut Window, cx: &mut Context<MarkdownApp>) {
    let receiver = cx.prompt_for_paths(PathPromptOptions {
        files: false,
        directories: true,
        multiple: false,
        prompt: None,
    });
    let app_entity = cx.entity();
    window
        .spawn(cx, async move |cx| {
            if let Ok(Ok(Some(paths))) = receiver.await {
                let root = paths[0].clone();
                let items = scan_directory(&root);
                let _ = app_entity.update_in(cx, |app, _w, cx| {
                    app.open_folder_path = Some(root);
                    app.tree_state = Some(cx.new(|cx| TreeState::new(cx).items(items)));
                    app.tree_visible = true;
                    cx.notify();
                });
            }
        })
        .detach();
}

pub fn toggle_tree(app: &mut MarkdownApp) {
    app.tree_visible = !app.tree_visible;
}

pub fn render_tree_sidebar(
    tree_state: &Entity<TreeState>,
    open_folder_path: &Option<PathBuf>,
    on_file_click: impl Fn(PathBuf, &mut App) + 'static + Clone,
    sidebar_bg: Hsla,
    sidebar_fg: Hsla,
) -> AnyElement {
    let root_name = open_folder_path
        .as_ref()
        .and_then(|p| p.file_name())
        .map(|n| n.to_string_lossy().to_string())
        .unwrap_or_default();

    let tree_view = Tree::new(
        tree_state,
        move |ix: usize, entry: &TreeEntry, selected: bool, _window: &mut Window, _cx: &mut App| {
            let item = entry.item();
            let indent = px(16.) * entry.depth() as f32;
            let mut list_item = ListItem::new(ix)
                .selected(selected)
                .pl(indent)
                .child(item.label.clone());

            if !entry.is_folder() {
                let on_file_click = on_file_click.clone();
                let path = item.id.to_string();
                list_item = list_item.on_click(move |_, _, cx| {
                    on_file_click(PathBuf::from(&path), cx);
                });
            }
            list_item
        },
    );

    div()
        .id("tree-sidebar")
        .flex_none()
        .w(px(250.))
        .h_full()
        .flex_col()
        .bg(sidebar_bg)
        .child(
            div()
                .px_3()
                .py_2()
                .text_sm()
                .text_color(sidebar_fg)
                .child(root_name),
        )
        .child(div().flex_1().overflow_hidden().child(tree_view.h_full()))
        .into_any_element()
}
