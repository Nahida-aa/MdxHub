use gpui::{AssetSource, Result, SharedString};
use std::borrow::Cow;

pub(crate) struct AppAssetSource;

impl AssetSource for AppAssetSource {
    fn load(&self, path: &str) -> Result<Option<Cow<'static, [u8]>>> {
        match path {
            "icons/window-close.svg" => {
                Ok(Some(Cow::Borrowed(include_bytes!(
                    "../assets/icons/window-close.svg"
                ))))
            }
            "icons/window-maximize.svg" => {
                Ok(Some(Cow::Borrowed(include_bytes!(
                    "../assets/icons/window-maximize.svg"
                ))))
            }
            "icons/window-minimize.svg" => {
                Ok(Some(Cow::Borrowed(include_bytes!(
                    "../assets/icons/window-minimize.svg"
                ))))
            }
            "icons/window-restore.svg" => {
                Ok(Some(Cow::Borrowed(include_bytes!(
                    "../assets/icons/window-restore.svg"
                ))))
            }
            _ => Ok(None),
        }
    }

    fn list(&self, _path: &str) -> Result<Vec<SharedString>> {
        Ok(vec![])
    }
}
