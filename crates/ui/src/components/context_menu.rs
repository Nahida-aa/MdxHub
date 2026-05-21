// use collections::HashMap;
// use gpui::{
//     Action, AnyElement, App, Bounds, Context, DefiniteLength, Entity, FocusHandle, OwnedMenu,
//     OwnedMenuItem, Pixels, SharedString, Subscription, Window, actions,
// };
// // use settings::Settings;

// use gpui_component::IconName;
// use schemars::JsonSchema;
// use serde::Deserialize;
// use smallvec::SmallVec;
// use std::{
//     cell::{Cell, RefCell},
//     rc::Rc,
// };
// // use ui::{ContextMenu, PopoverMenu, PopoverMenuHandle, Tooltip, prelude::*};

// // use crate::title_bar_settings::TitleBarSettings;

// pub enum ContextMenuItem {
//     Separator,
//     Header(SharedString),
//     /// title, link_label, link_url
//     HeaderWithLink(SharedString, SharedString, SharedString), // This could be folded into header
//     Label(SharedString),
//     Entry(ContextMenuEntry),
//     CustomEntry {
//         entry_render: Box<dyn Fn(&mut Window, &mut App) -> AnyElement>,
//         handler: Rc<dyn Fn(Option<&FocusHandle>, &mut Window, &mut App)>,
//         selectable: bool,
//         documentation_aside: Option<DocumentationAside>,
//     },
//     Submenu {
//         label: SharedString,
//         icon: Option<IconName>,
//         icon_color: Option<Color>,
//         builder: Rc<dyn Fn(ContextMenu, &mut Window, &mut Context<ContextMenu>) -> ContextMenu>,
//     },
// }
// pub struct ContextMenuEntry {
//     toggle: Option<(IconPosition, bool)>,
//     label: SharedString,
//     icon: Option<IconName>,
//     custom_icon_path: Option<SharedString>,
//     custom_icon_svg: Option<SharedString>,
//     icon_position: IconPosition,
//     icon_size: IconSize,
//     icon_color: Option<Color>,
//     handler: Rc<dyn Fn(Option<&FocusHandle>, &mut Window, &mut App)>,
//     secondary_handler: Option<Rc<dyn Fn(Option<&FocusHandle>, &mut Window, &mut App)>>,
//     action: Option<Box<dyn Action>>,
//     disabled: bool,
//     documentation_aside: Option<DocumentationAside>,
//     end_slot_icon: Option<IconName>,
//     end_slot_title: Option<SharedString>,
//     end_slot_handler: Option<Rc<dyn Fn(Option<&FocusHandle>, &mut Window, &mut App)>>,
//     show_end_slot_on_hover: bool,
// }

// pub struct ContextMenu {
//     builder: Option<Rc<dyn Fn(Self, &mut Window, &mut Context<Self>) -> Self>>,
//     items: Vec<ContextMenuItem>,
//     focus_handle: FocusHandle,
//     action_context: Option<FocusHandle>,
//     selected_index: Option<usize>,
//     delayed: bool,
//     clicked: bool,
//     end_slot_action: Option<Box<dyn Action>>,
//     key_context: SharedString,
//     _on_blur_subscription: Subscription,
//     keep_open_on_confirm: bool,
//     fixed_width: Option<DefiniteLength>,
//     main_menu: Option<Entity<ContextMenu>>,
//     main_menu_observed_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
//     // Docs aide-related fields
//     documentation_aside: Option<(usize, DocumentationAside)>,
//     aside_trigger_bounds: Rc<RefCell<HashMap<usize, Bounds<Pixels>>>>,
//     // Submenu-related fields
//     submenu_state: SubmenuState,
//     hover_target: HoverTarget,
//     submenu_safety_threshold_x: Option<Pixels>,
//     submenu_trigger_bounds: Rc<Cell<Option<Bounds<Pixels>>>>,
//     submenu_trigger_mouse_down: bool,
//     ignore_blur_until: Option<Instant>,
// }

// #[derive(Clone)]
// pub struct DocumentationAside {
//     pub side: DocumentationSide,
//     pub render: Rc<dyn Fn(&mut App) -> AnyElement>,
// }
