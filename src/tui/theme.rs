use ratatui::style::Color;

/// Available UI themes
#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    Cyan,
    Monochrome,
    LamessUi,
}

impl Theme {
    pub fn label(&self) -> &'static str {
        match self {
            Theme::Cyan => "Cyan (default)",
            Theme::Monochrome => "Monochrome",
            Theme::LamessUi => "Lamess UI",
        }
    }

    pub fn next(&self) -> Theme {
        match self {
            Theme::Cyan => Theme::Monochrome,
            Theme::Monochrome => Theme::LamessUi,
            Theme::LamessUi => Theme::Cyan,
        }
    }
}

impl Default for Theme {
    fn default() -> Self { Theme::Cyan }
}

/// Resolved color tokens for a theme
#[derive(Debug, Clone)]
pub struct ThemeColors {
    /// Border of the active/focused panel
    pub border_active: Color,
    /// Border of inactive panels
    pub border_inactive: Color,
    /// Selected/highlighted item foreground
    pub highlight_fg: Color,
    /// Selected/highlighted item background
    pub highlight_bg: Color,
    /// Primary text
    pub text_primary: Color,
    /// Muted / secondary text
    pub text_muted: Color,
    /// Success state (running, on, enabled)
    pub success: Color,
    /// Danger / error state
    pub danger: Color,
    /// Tab bar active label
    pub tab_active: Color,
    /// Panel title color
    pub title: Color,
}

impl Theme {
    pub fn colors(&self) -> ThemeColors {
        match self {
            // ── Cyan ────────────────────────────────────────────────────────
            Theme::Cyan => ThemeColors {
                border_active:   Color::Cyan,
                border_inactive: Color::DarkGray,
                highlight_fg:    Color::Black,
                highlight_bg:    Color::Cyan,
                text_primary:    Color::White,
                text_muted:      Color::DarkGray,
                success:         Color::Green,
                danger:          Color::Red,
                tab_active:      Color::Cyan,
                title:           Color::Cyan,
            },
            // ── Monochrome ───────────────────────────────────────────────────
            Theme::Monochrome => ThemeColors {
                border_active:   Color::White,
                border_inactive: Color::DarkGray,
                highlight_fg:    Color::Black,
                highlight_bg:    Color::White,
                text_primary:    Color::White,
                text_muted:      Color::Gray,
                success:         Color::White,
                danger:          Color::Gray,
                tab_active:      Color::White,
                title:           Color::White,
            },
            // ── Lamess UI ────────────────────────────────────────────────────
            // Palette from brand guide:
            //   Background #000000  Surface #0F0F0F  Border #1E1E1E
            //   Signal Yellow #F5D000  Terminal White #FFFFFF
            //   Dim Gray #7A7A7A  Alert Red #FF3B3B  Confirm Green #39FF87
            Theme::LamessUi => ThemeColors {
                border_active:   Color::Rgb(245, 208, 0),   // Signal Yellow
                border_inactive: Color::Rgb(30, 30, 30),    // Grid Line
                highlight_fg:    Color::Rgb(0, 0, 0),       // pure black text on yellow
                highlight_bg:    Color::Rgb(245, 208, 0),   // Signal Yellow fill
                text_primary:    Color::Rgb(255, 255, 255),
                text_muted:      Color::Rgb(122, 122, 122), // Dim Gray
                success:         Color::Rgb(57, 255, 135),  // Confirm Green
                danger:          Color::Rgb(255, 59, 59),   // Alert Red
                tab_active:      Color::Rgb(245, 208, 0),
                title:           Color::Rgb(245, 208, 0),
            },
        }
    }
}
