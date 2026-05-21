mod application_menu;

use crate::application_menu::{ApplicationMenu, show_menus};
use gpui::prelude::*;
use gpui::*;
use gpui_component::{h_flex, v_flex};
use platform_title_bar::PlatformTitleBar;
mod title_bar_settings;
use ui::utils::platform_title_bar_height;

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
    // workspace: WeakEntity<Workspace>,             // 所属窗口
    // multi_workspace: Option<WeakEntity<MultiWorkspace>>,
    application_menu: Option<Entity<ApplicationMenu>>, // 主菜单
                                                       // update_version: Entity<UpdateVersion>,        // 更新提示
                                                       // banner: Option<Entity<OnboardingBanner>>,     // 新手引导条
                                                       // ... 其他
}

impl TitleBar {
    pub fn new(
        children: impl IntoIterator<Item = AnyElement>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let platform_titlebar =
            cx.new(|cx| PlatformTitleBar::new("platform-title-bar", children, cx));
        Self { platform_titlebar }
    }

    pub fn set_children(
        &mut self,
        children: impl IntoIterator<Item = AnyElement>,
        cx: &mut Context<Self>,
    ) {
        let children: Vec<AnyElement> = children.into_iter().collect();
        self.platform_titlebar.update(cx, |ptb, _| {
            ptb.set_children(children);
        });
    }
}

impl Render for TitleBar {
    // 3. render() — 核心渲染逻辑
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        // 读取 TitleBarSettings（show_menus, button_layout...）

        let button_layout = title_bar_settings.button_layout;

        // 收集左侧 children 列表：
        // - 项目/仓库信息（项目名、分支、remote 连接状态）
        // - 协作头像列表
        // - 新手引导 banner
        // - 右侧：连接状态、更新提示、用户菜单按钮

        let mut children = <ArrayVec<_, 4>>::new();

        self.platform_titlebar.update(cx, |this, _| {
            this.set_button_layout(button_layout); // 布局描述, 定义 Linux 下窗口按钮（关闭/最小化/最大化）在标题栏的排列顺序和位置
            this.set_children(children); //所有 children 放进标题栏ni
        });
        self.platform_titlebar.clone().into_any_element()
    }
}
