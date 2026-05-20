use crate::app::MarkdownApp;
use gpui::*;

pub fn open_file(app: &mut MarkdownApp, window: &mut Window, cx: &mut Context<MarkdownApp>) {
    let receiver = cx.prompt_for_paths(PathPromptOptions {
        files: true,
        directories: false,
        multiple: false,
        prompt: None,
    });
    let app_entity = cx.entity();
    let editor = app.editor.clone();

    window
        .spawn(cx, async move |cx| {
            if let Ok(Ok(Some(paths))) = receiver.await {
                let path = paths[0].clone();
                match std::fs::read_to_string(&path) {
                    Ok(text) => {
                        let _ = editor.update_in(cx, |ed, w, cx| {
                            ed.set_value(&text, w, cx);
                        });
                        let _ = app_entity.update_in(cx, |app, _w, cx| {
                            app.markdown_content = text.into();
                            app.current_path = Some(path);
                            cx.notify();
                        });
                    }
                    Err(err) => eprintln!("Error reading file: {}", err),
                }
            }
        })
        .detach();
}

pub fn save_file(app: &MarkdownApp, _window: &mut Window, cx: &mut Context<MarkdownApp>) {
    if let Some(path) = app.current_path.as_ref() {
        let content = app.editor.read(cx).value().to_string();
        let _ = std::fs::write(path, &content);
    }
}

pub fn save_file_as(app: &mut MarkdownApp, window: &mut Window, cx: &mut Context<MarkdownApp>) {
    let content = app.editor.read(cx).value();
    let receiver = cx.prompt_for_new_path(std::path::Path::new(""), None);
    let app_entity = cx.entity();

    window
        .spawn(cx, async move |cx| {
            if let Ok(Ok(Some(path))) = receiver.await {
                let _ = std::fs::write(&path, content.as_str());
                let _ = app_entity.update_in(cx, |app, _w, cx| {
                    app.current_path = Some(path);
                    cx.notify();
                });
            }
        })
        .detach();
}
