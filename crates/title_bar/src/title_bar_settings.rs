use gui::WindowButtonLayout;
use settings::{
    RegisterSetting,
    Settings,
    SettingsContent, // JSON 设置文件的整体结构
};

#[derive(
    Copy,
    Clone,
    Debug,
    RegisterSetting, //  derive macro 会生成 inventory::submit!
)]
pub struct TitleBarSettings {
    // pub show_branch_status_icon: bool,             // 显示分支状态
    // pub show_onboarding_banner: bool,              // 显示新手引导横幅
    // pub show_user_picture: bool,                   // 显示用户头像
    pub show_branch_name: bool,   // 显示分支名称
    pub show_project_items: bool, // 显示项目文件
    // pub show_sign_in: bool,                        // 显示登录按钮
    // pub show_user_menu: bool,                      // 显示用户菜单
    pub show_menus: bool,                          // 显示菜单栏
    pub button_layout: Option<WindowButtonLayout>, // 窗口按钮的排列布局
}

impl Settings for TitleBarSettings {
    fn from_settings(s: &SettingsContent) -> Self {
        let content = s.title_bar.clone().unwrap();
        TitleBarSettings {
            // show_branch_status_icon: content.show_branch_status_icon.unwrap(),
            // show_onboarding_banner: content.show_onboarding_banner.unwrap(),
            // show_user_picture: content.show_user_picture.unwrap(),
            show_branch_name: content.show_branch_name.unwrap(),
            show_project_items: content.show_project_items.unwrap(),
            // show_sign_in: content.show_sign_in.unwrap(),
            // show_user_menu: content.show_user_menu.unwrap(),
            show_menus: content.show_menus.unwrap(),
            button_layout: content.button_layout.unwrap_or_default().into_layout(), // 转换为 WindowButtonLayout 枚举
        }
    }
}
