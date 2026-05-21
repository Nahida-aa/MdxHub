use collections::{
    // btree_map, hash_map, BTreeMap,
    HashMap,
};
use futures::{
    channel::{mpsc, oneshot},
    future::LocalBoxFuture,
    FutureExt, StreamExt,
};
use gpui::{
    App, AppContext, AsyncApp, BorrowAppContext, Entity, Global, SharedString, Task, UpdateGlobal,
};
use settings_content::{
    // merge_from::MergeFrom, ExtensionsSettingsContent, ProfileBase, ProjectSettingsContent,
    // RootUserSettings,
    SettingsContent,
    // UserSettingsContent,
};
use std::{
    any::{type_name, Any, TypeId},
    fmt::Debug,
    ops::Range,
    path::{Path, PathBuf},
    rc::Rc,
    str,
    sync::Arc,
};
use util::{
    // ResultExt as _,
    rel_path::RelPath,
    // schemars::{AllowTrailingCommas, DefaultDenyUnknownFields, replace_subschema},
};
// use settings_content::{ActionName, ParseStatus};

use crate::{
    // settings_content::{
    //     merge_from::MergeFrom, ExtensionsSettingsContent, ProfileBase, ProjectSettingsContent,
    //     RootUserSettings, SettingsContent, UserSettingsContent,
    // },
    // ActiveSettingsProfileName, FontFamilyName, IconThemeName, LanguageSettingsContent,
    // LanguageToSettingsMap, LspSettings, LspSettingsMap,
    SemanticTokenRules,
    // ThemeName,
    // UserSettingsContentExt, VsCodeSettings,
    WorktreeId,
};

/// A value that can be defined as a user setting.
///
/// Settings can be loaded from a combination of multiple JSON files.
pub trait Settings: 'static + Send + Sync + Sized {
    /// The name of the keys in the [`FileContent`](Self::FileContent) that should
    /// always be written to a settings file, even if their value matches the default
    /// value.
    ///
    /// This is useful for tagged [`FileContent`](Self::FileContent)s where the tag
    /// is a "version" field that should always be persisted, even if the current
    /// user settings match the current version of the settings.
    const PRESERVED_KEYS: Option<&'static [&'static str]> = None;

    /// Read the value from default.json.
    ///
    /// This function *should* panic if default values are missing,
    /// and you should add a default to default.json for documentation.
    fn from_settings(content: &SettingsContent) -> Self;

    #[track_caller]
    fn register(cx: &mut App)
    // 注册到 SettingsStore
    //  Settings trait 默认实现
    where
        Self: Sized,
    {
        SettingsStore::update_global(cx, |store, _| {
            store.register_setting::<Self>();
        });
    }

    #[track_caller]
    fn get<'a>(path: Option<SettingsLocation>, cx: &'a App) -> &'a Self
    where
        Self: Sized,
    {
        cx.global::<SettingsStore>().get(path)
    }

    #[track_caller]
    fn get_global(cx: &App) -> &Self
    where
        Self: Sized,
    {
        cx.global::<SettingsStore>().get(None) // 读取全局值
    }

    #[track_caller]
    fn try_get(cx: &App) -> Option<&Self>
    where
        Self: Sized,
    {
        if cx.has_global::<SettingsStore>() {
            cx.global::<SettingsStore>().try_get(None)
        } else {
            None
        }
    }

    #[track_caller]
    fn try_read_global<R>(cx: &AsyncApp, f: impl FnOnce(&Self) -> R) -> Option<R>
    where
        Self: Sized,
    {
        cx.try_read_global(|s: &SettingsStore, _| f(s.get(None)))
    }

    #[track_caller]
    fn override_global(settings: Self, cx: &mut App)
    where
        Self: Sized,
    {
        cx.global_mut::<SettingsStore>().override_global(settings)
    }
}

// 以 TitleBarSettings 为例，RegisteredSetting 包含了：
// - 创建 SettingValue<TitleBarSettings>
// - 从 JSON 解析出值
// - id: TitleBarSettings 的 TypeId
pub struct RegisteredSetting {
    pub settings_value: fn() -> Box<dyn AnySettingValue>,
    pub from_settings: fn(&SettingsContent) -> Box<dyn Any>,
    pub id: fn() -> TypeId,
}

#[doc(hidden)]
pub trait AnySettingValue: 'static + Send + Sync {
    fn setting_type_name(&self) -> &'static str;

    fn from_settings(&self, s: &SettingsContent) -> Box<dyn Any>;

    fn value_for_path(&self, path: Option<SettingsLocation>) -> &dyn Any;
    fn all_local_values(&self) -> Vec<(WorktreeId, Arc<RelPath>, &dyn Any)>;
    fn set_global_value(&mut self, value: Box<dyn Any>);
    fn set_local_value(&mut self, root_id: WorktreeId, path: Arc<RelPath>, value: Box<dyn Any>);
    fn clear_local_values(&mut self, root_id: WorktreeId);
}

pub struct SettingsStore {
    setting_values: HashMap<TypeId, Box<dyn AnySettingValue>>,
    default_settings: Rc<SettingsContent>,
    // user_settings: Option<UserSettingsContent>,
    global_settings: Option<Box<SettingsContent>>,

    extension_settings: Option<Box<SettingsContent>>,
    server_settings: Option<Box<SettingsContent>>,

    language_semantic_token_rules: HashMap<SharedString, SemanticTokenRules>,

    merged_settings: Rc<SettingsContent>,

    last_user_settings_content: Option<String>,
    last_global_settings_content: Option<String>,
    local_settings: BTreeMap<(WorktreeId, Arc<RelPath>), SettingsContent>,
    pub editorconfig_store: Entity<EditorconfigStore>,

    _settings_files_watcher: Option<Task<()>>,
    _setting_file_updates: Task<()>,
    setting_file_updates_tx:
        mpsc::UnboundedSender<Box<dyn FnOnce(AsyncApp) -> LocalBoxFuture<'static, Result<()>>>>,
    file_errors: BTreeMap<SettingsFile, SettingsParseResult>,
}
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum SettingsFile {
    Default,
    Global,
    User,
    Server,
    /// Represents project settings in ssh projects as well as local projects
    Project((WorktreeId, Arc<RelPath>)),
}
impl SettingsStore {
    fn register_setting_internal(&mut self, registered_setting: &RegisteredSetting) {
        // 按 TypeId 查 HashMap，避免重复注册
        let entry = self.setting_values.entry((registered_setting.id)());
        // 如果 HashMap 里已有这个 TypeId，直接返回（不重复注册）
        if matches!(entry, hash_map::Entry::Occupied(_)) {
            return;
        }
        // 插入新的 SettingValue
        let setting_value = entry.or_insert((registered_setting.settings_value)());
        let value = (registered_setting.from_settings)(&self.merged_settings);
        setting_value.set_global_value(value);
    }
    /// Add a new type of setting to the store.
    pub fn register_setting<T: Settings>(&mut self) {
        self.register_setting_internal(&RegisteredSetting {
            settings_value: || {
                Box::new(SettingValue::<T> {
                    global_value: None,
                    local_values: Vec::new(),
                })
            },
            from_settings: |content| Box::new(T::from_settings(content)),
            id: || TypeId::of::<T>(),
        });
    }
}
