use gpui::{App, BackgroundExecutor, ReadGlobal};
use fs::{Fs, PathEventKind};
use std::{path::PathBuf, sync::Arc, time::Duration};
use futures::{StreamExt, channel::mpsc};

pub fn watch_config_file(
    executor: &BackgroundExecutor,
    fs: Arc<dyn Fs>,
    path: PathBuf,
) -> (mpsc::UnboundedReceiver<String>, gpui::Task<()>) {
    let (tx, rx) = mpsc::unbounded();
    let task = executor.spawn(async move {
        let path = fs.canonicalize(&path).await.unwrap_or_else(|_| path);
        let (events, _) = fs.watch(&path, Duration::from_millis(100)).await;
        futures::pin_mut!(events);

        let contents = fs.load(&path).await.unwrap_or_default();
        if tx.unbounded_send(contents).is_err() {
            return;
        }

        loop {
            if events.next().await.is_none() {
                break;
            }

            if let Ok(contents) = fs.load(&path).await
                && tx.unbounded_send(contents).is_err()
            {
                break;
            }
        }
    });
    (rx, task)
}
