mod application_menu;
use crate::application_menu::{
    ApplicationMenu,
    // show_menus
};
use arrayvec::ArrayVec;
use gpui::prelude::*;
use gpui::*;
use gpui_component::{Disableable as _, h_flex, v_flex};
use platform_title_bar::PlatformTitleBar;
mod title_bar_settings;
use gpui::{
    //
    Render,
};
use project::{
    //
    Project,
};
use settings::Settings as _; // Rust 里调用 trait 方法需要 trait 本身在当前作用域可见
use title_bar_settings::TitleBarSettings;
use ui::{Button, Color, LabelSize, PlatformStyle, utils::platform_title_bar_height};
use workspace::{MultiWorkspace, Workspace};

const MAX_PROJECT_NAME_LENGTH: usize = 40;

pub fn title_bar_options() -> TitlebarOptions {
    TitlebarOptions {
        title: None,
        appears_transparent: true,
        traffic_light_position: Some(point(px(9.0), px(9.0))),
    }
}

// 1. init() — 标题栏的创建时机
pub fn init(cx: &mut App) {
    PlatformTitleBar::init(cx);
    // 通过 cx.observe_new::<Workspace>() 监听，每当有一个 Workspace 窗口被创建，就自动初始化一个 TitleBar，并挂到 Workspace 上
}

// 2. struct TitleBar — 它持有什么
pub struct TitleBar {
    platform_titlebar: Entity<PlatformTitleBar>, // ← 操作系统底层控件
    project: Entity<Project>,                    // 当前项目
    // user_store: Entity<UserStore>,                // 用户系统（头像、登录）
    // client: Arc<Client>,                          // 网络客户端（协作）
    workspace: WeakEntity<Workspace>, // 所属窗口
    multi_workspace: Option<WeakEntity<MultiWorkspace>>,
    application_menu: Option<Entity<ApplicationMenu>>, // 主菜单
                                                       // update_version: Entity<UpdateVersion>,        // 更新提示
                                                       // banner: Option<Entity<OnboardingBanner>>,     // 新手引导条
                                                       // ... 其他
}

impl Render for TitleBar {
    // 3. render() — 核心渲染逻辑
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 读取 TitleBarSettings（show_menus, button_layout...）
        let title_bar_settings = *TitleBarSettings::get_global(cx);
        let button_layout = title_bar_settings.button_layout;

        // 收集左侧 children 列表：
        // - 项目/仓库信息（项目名、分支、remote 连接状态）
        // - 协作头像列表
        // - 新手引导 banner
        // - 右侧：连接状态、更新提示、用户菜单按钮
        let show_menus = false; // show_menus(cx);
        let mut children = <ArrayVec<_, 4>>::new();

        let mut project_name = None;
        let mut repository = None;
        let mut linked_worktree_name = None;
        if let Some(worktree) = self.effective_active_worktree(cx) {}
        // 应用菜单：只在 show_menus = false（紧凑模式）时显示，同时通过 menu.all_menus_shown() 检测是否有菜单展开，展开时隐藏项目信息腾出空间
        // 受限模式：render_restricted_mode() 渲染一个指示器
        // 阻止点击穿透：on_mouse_down 调用 stop_propagation()，防止点击标题栏中间区域触发窗口拖动
        // 整体逻辑：菜单展开 → 隐藏项目信息；菜单收起 → 显示项目信息，紧凑模式下标题栏空间动态复用
        children.push(
            h_flex()
                .h_full()
                .gap_0p5()
                .map(|title_bar| {
                    // 如果 render_project_items 为 true，依次渲染 project host（远程项目标识）、project name（项目名称）、git branch（分支名 + worktree 名）
                    let mut render_project_items = title_bar_settings.show_branch_name
                        || title_bar_settings.show_project_items;
                    title_bar
                        .when_some(
                            self.application_menu.clone().filter(|_| !show_menus),
                            |title_bar, menu| {
                                render_project_items &=
                                    !menu.update(cx, |menu, cx| menu.all_menus_shown(cx));
                                title_bar.child(menu)
                            },
                        )
                        .children(self.render_restricted_mode(cx))
                        .when(render_project_items, |title_bar| {
                            title_bar
                                .when(title_bar_settings.show_project_items, |title_bar| {
                                    title_bar
                                        .children(self.render_project_host(cx))
                                        .child(self.render_project_name(project_name, window, cx))
                                })
                                .when_some(
                                    repository.filter(|_| is_git_enabled),
                                    |title_bar, repository| {
                                        title_bar.children(self.render_worktree_and_branch(
                                            repository,
                                            linked_worktree_name,
                                            cx,
                                        ))
                                    },
                                )
                        })
                })
                .on_mouse_down(MouseButton::Left, |_, _, cx| cx.stop_propagation())
                .into_any_element(),
        );
        self.platform_titlebar.update(cx, |this, _| {
            this.set_button_layout(button_layout); // 布局描述, 定义 Linux 下窗口按钮（关闭/最小化/最大化）在标题栏的排列顺序和位置
            this.set_children(children); //所有 children 放进标题栏ni
        });
        self.platform_titlebar.clone().into_any_element()
    }
}

impl TitleBar {
    pub fn new(
        id: impl Into<ElementId>,
        workspace: &Workspace,
        multi_workspace: Option<WeakEntity<MultiWorkspace>>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let project = workspace.project().clone();

        let platform_titlebar = cx.new(|cx| {
            let mut titlebar = PlatformTitleBar::new(id, cx);
            if let Some(mw) = multi_workspace.clone() {
                titlebar = titlebar.with_multi_workspace(mw);
            }
            titlebar
        });
        let platform_style = PlatformStyle::platform();

        let application_menu = match platform_style {
            PlatformStyle::Mac => {
                if option_env!("ZED_USE_CROSS_PLATFORM_MENU").is_some() {
                    Some(cx.new(|cx| ApplicationMenu::new(window, cx)))
                } else {
                    None
                }
            }
            PlatformStyle::Linux | PlatformStyle::Windows => {
                Some(cx.new(|cx| ApplicationMenu::new(window, cx)))
            }
        };
        let mut this = Self {
            platform_titlebar,
            application_menu,
            workspace: workspace.weak_handle(),
            multi_workspace,
            project,
        };

        this
    }

    // 项目处于"受限模式"时显示警告按钮
    //     - 检查 project 的 WorktreeStore 是否有未受信任的 worktree
    // - 没有 → 返回 None，不渲染任何东西
    // - 有 → 渲染一个 "Restricted Mode" 按钮：
    // - 黄色警告图标 + 文字（TintColor::Warning / Color::Warning）
    // - 悬停显示 tooltip：解释受限模式 + 提供"信任项目"的快捷键提示
    // - 点击弹出信任安全弹窗（show_worktree_trust_security_modal）
    pub fn render_restricted_mode(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        return None; // TODO
        // let has_restricted_worktrees =
        //     TrustedWorktrees::has_restricted_worktrees(&self.project.read(cx).worktree_store(), cx);
        // if !has_restricted_worktrees {
        //     return None;
        // }

        // let button = Button::new("restricted_mode_trigger", "Restricted Mode")
        //     .style(ButtonStyle::Tinted(TintColor::Warning))
        //     .label_size(LabelSize::Small)
        //     .color(Color::Warning)
        //     .start_icon(
        //         Icon::new(IconName::Warning)
        //             .size(IconSize::Small)
        //             .color(Color::Warning),
        //     )
        //     .tooltip(|_, cx| {
        //         Tooltip::with_meta(
        //             "You're in Restricted Mode",
        //             Some(&ToggleWorktreeSecurity),
        //             "Mark this project as trusted and unlock all features",
        //             cx,
        //         )
        //     })
        //     .on_click({
        //         cx.listener(move |this, _, window, cx| {
        //             this.workspace
        //                 .update(cx, |workspace, cx| {
        //                     workspace.show_worktree_trust_security_modal(true, window, cx)
        //                 })
        //                 .log_err();
        //         })
        //     });

        // if cfg!(macos_sdk_26) {
        //     // Make up for Tahoe's traffic light buttons having less spacing around them
        //     Some(div().child(button).ml_0p5().into_any_element())
        // } else {
        //     Some(button.into_any_element())
        // }
    }

    pub fn render_project_host(&self, cx: &mut Context<Self>) -> Option<AnyElement> {
        // if self.project.read(cx).is_via_remote_server() {
        //     return self.render_remote_project_connection(cx);
        // }

        // if self.project.read(cx).is_disconnected(cx) {
        //     return Some(
        //         Button::new("disconnected", "Disconnected")
        //             .disabled(true)
        //             .color(Color::Disabled)
        //             .label_size(LabelSize::Small)
        //             .into_any_element(),
        //     );
        // }
        return Some(
            Button::new("disconnected", "Disconnected")
                .disabled(true)
                .color(Color::Disabled)
                .label_size(LabelSize::Small)
                .into_any_element(),
        );

        // let host = self.project.read(cx).host()?;
        // let host_user = self.user_store.read(cx).get_cached_user(host.user_id)?;
        // let participant_index = self
        //     .user_store
        //     .read(cx)
        //     .participant_indices()
        //     .get(&host_user.legacy_id)?;

        // Some(
        //     Button::new("project_owner_trigger", host_user.github_login.clone())
        //         .color(Color::Player(participant_index.0))
        //         .label_size(LabelSize::Small)
        //         .tooltip(move |_, cx| {
        //             let tooltip_title = format!(
        //                 "{} is sharing this project. Click to follow.",
        //                 host_user.github_login
        //             );

        //             Tooltip::with_meta(tooltip_title, None, "Click to Follow", cx)
        //         })
        //         .on_click({
        //             let host_peer_id = host.peer_id;
        //             cx.listener(move |this, _, window, cx| {
        //                 this.workspace
        //                     .update(cx, |workspace, cx| {
        //                         workspace.follow(host_peer_id, window, cx);
        //                     })
        //                     .log_err();
        //             })
        //         })
        //         .into_any_element(),
        // )
    }

    fn render_project_name(
        &self,
        name: Option<SharedString>,
        _: &mut Window,
        cx: &mut Context<Self>,
    ) -> impl IntoElement {
        let workspace = self.workspace.clone();

        let is_project_selected = name.is_some();

        let display_name = if let Some(ref name) = name {
            util::truncate_and_trailoff(name, MAX_PROJECT_NAME_LENGTH)
        } else {
            "Open Recent Project".to_string()
        };

        let is_sidebar_open = self
            .multi_workspace
            .as_ref()
            .and_then(|mw| mw.upgrade())
            .map(|mw| mw.read(cx).sidebar_open())
            .unwrap_or(false)
            && PlatformTitleBar::is_multi_workspace_enabled(cx);

        let is_threads_list_view_active = self
            .multi_workspace
            .as_ref()
            .and_then(|mw| mw.upgrade())
            .map(|mw| mw.read(cx).is_threads_list_view_active(cx))
            .unwrap_or(false);

        if is_sidebar_open && is_threads_list_view_active {
            return self
                .render_recent_projects_popover(display_name, is_project_selected, cx)
                .into_any_element();
        }

        let focus_handle = workspace
            .upgrade()
            .map(|w| w.read(cx).focus_handle(cx))
            .unwrap_or_else(|| cx.focus_handle());

        let window_project_groups: Vec<_> = self
            .multi_workspace
            .as_ref()
            .and_then(|mw| mw.upgrade())
            .map(|mw| mw.read(cx).project_group_keys())
            .unwrap_or_default();

        PopoverMenu::new("recent-projects-menu")
            .menu(move |window, cx| {
                Some(recent_projects::RecentProjects::popover(
                    workspace.clone(),
                    window_project_groups.clone(),
                    false,
                    focus_handle.clone(),
                    window,
                    cx,
                ))
            })
            .trigger_with_tooltip(
                Button::new("project_name_trigger", display_name)
                    .label_size(LabelSize::Small)
                    .when(self.worktree_count(cx) > 1, |this| {
                        this.end_icon(
                            Icon::new(IconName::ChevronDown)
                                .size(IconSize::XSmall)
                                .color(Color::Muted),
                        )
                    })
                    .selected_style(ButtonStyle::Tinted(TintColor::Accent))
                    .when(!is_project_selected, |s| s.color(Color::Muted)),
                move |_window, cx| {
                    Tooltip::for_action(
                        "Recent Projects",
                        &zed_actions::OpenRecent {
                            create_new_window: false,
                        },
                        cx,
                    )
                },
            )
            .anchor(gpui::Corner::TopLeft)
            .into_any_element()
    }

    /// Returns the worktree to display in the title bar.
    /// - Prefer the worktree owning the project's active repository
    /// - Fall back to the first visible worktree
    pub fn effective_active_worktree(&self, cx: &App) -> Option<Entity<project::Worktree>> {
        let project = self.project.read(cx);

        if let Some(repo) = project.active_repository(cx) {
            let repo = repo.read(cx);
            let repo_path = &repo.work_directory_abs_path;

            for worktree in project.visible_worktrees(cx) {
                let worktree_path = worktree.read(cx).abs_path();
                if worktree_path == *repo_path || worktree_path.starts_with(repo_path.as_ref()) {
                    return Some(worktree);
                }
            }
        }

        project.visible_worktrees(cx).next()
    }
}
