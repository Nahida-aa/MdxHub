use settings::{Settings, SettingsStore};

use gpui::{
    AnyWindowHandle, App, Hsla, Pixels, ScrollHandle, SharedString, SystemWindowTab,
    SystemWindowTabController, WindowId, px,
};

use workspace::{
    // CloseWindow, ItemSettings, Workspace,
    Workspace,
    WorkspaceSettings, // item::{ClosePosition, ShowCloseButton},
};

#[derive(Clone)]
pub struct DraggedWindowTab {
    pub id: WindowId,
    pub ix: usize,
    pub handle: AnyWindowHandle,
    pub title: String,
    pub width: Pixels,
    pub is_active: bool,
    pub active_background_color: Hsla,
    pub inactive_background_color: Hsla,
}
pub struct SystemWindowTabs {
    tab_bar_scroll_handle: ScrollHandle,
    measured_tab_width: Pixels,
    last_dragged_tab: Option<DraggedWindowTab>,
}

impl SystemWindowTabs {
    pub fn new() -> Self {
        Self {
            tab_bar_scroll_handle: ScrollHandle::new(),
            measured_tab_width: px(0.),
            last_dragged_tab: None,
        }
    }
    pub fn init(cx: &mut App) {
        let mut was_use_system_window_tabs =
            WorkspaceSettings::get_global(cx).use_system_window_tabs;

        cx.observe_global::<SettingsStore>(move |cx| {
            let use_system_window_tabs = WorkspaceSettings::get_global(cx).use_system_window_tabs;
            if use_system_window_tabs == was_use_system_window_tabs {
                return;
            }
            was_use_system_window_tabs = use_system_window_tabs;

            let tabbing_identifier = if use_system_window_tabs {
                Some(String::from("zed"))
            } else {
                None
            };

            if use_system_window_tabs {
                SystemWindowTabController::init(cx);
            }

            cx.windows().iter().for_each(|handle| {
                let _ = handle.update(cx, |_, window, cx| {
                    window.set_tabbing_identifier(tabbing_identifier.clone());
                    if use_system_window_tabs {
                        let tabs = if let Some(tabs) = window.tabbed_windows() {
                            tabs
                        } else {
                            vec![SystemWindowTab::new(
                                SharedString::from(window.window_title()),
                                window.window_handle(),
                            )]
                        };

                        SystemWindowTabController::add_tab(cx, handle.window_id(), tabs);
                    }
                });
            });
        })
        .detach();

        cx.observe_new(|workspace: &mut Workspace, _, _| {
            // TODO:
            // workspace.register_action_renderer(|div, _, window, cx| {
            //     let window_id = window.window_handle().window_id();
            //     let controller = cx.global::<SystemWindowTabController>();

            //     let tab_groups = controller.tab_groups();
            //     let tabs = controller.tabs(window_id);
            //     let Some(tabs) = tabs else {
            //         return div;
            //     };

            //     div.when(tabs.len() > 1, |div| {
            //         div.on_action(move |_: &ShowNextWindowTab, window, cx| {
            //             SystemWindowTabController::select_next_tab(
            //                 cx,
            //                 window.window_handle().window_id(),
            //             );
            //         })
            //         .on_action(move |_: &ShowPreviousWindowTab, window, cx| {
            //             SystemWindowTabController::select_previous_tab(
            //                 cx,
            //                 window.window_handle().window_id(),
            //             );
            //         })
            //         .on_action(move |_: &MoveTabToNewWindow, window, cx| {
            //             SystemWindowTabController::move_tab_to_new_window(
            //                 cx,
            //                 window.window_handle().window_id(),
            //             );
            //             window.move_tab_to_new_window();
            //         })
            //     })
            //     .when(tab_groups.len() > 1, |div| {
            //         div.on_action(move |_: &MergeAllWindows, window, cx| {
            //             SystemWindowTabController::merge_all_windows(
            //                 cx,
            //                 window.window_handle().window_id(),
            //             );
            //             window.merge_all_windows();
            //         })
            //     })
            // });
        })
        .detach();
    }
}
