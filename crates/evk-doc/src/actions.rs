use gpui::actions;

actions!(app, [
    /// Show the About Evk dialog.
    AboutEvk,
    /// Open a markdown file.
    OpenFile,
    /// Save the current file.
    SaveFile,
    /// Save the current file with a new name.
    SaveFileAs,
    /// Open a folder as the workspace root.
    OpenFolder,
    /// Toggle between light and dark themes.
    ToggleTheme,
    /// Set the dark theme.
    SetThemeDark,
    /// Set the light theme.
    SetThemeLight,
    /// Toggle the file tree sidebar visibility.
    ToggleTree,
    /// Quit the application.
    Quit,
]);
