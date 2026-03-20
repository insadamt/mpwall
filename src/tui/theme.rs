use ratatui::style::Color;

#[derive(Debug, Clone, Copy, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Theme {
    LamessUi,
    Cyan,
    Monochrome,
}

impl Theme {
    pub fn label(&self) -> &'static str {
        match self {
            Theme::LamessUi  => "Lamess UI",
            Theme::Cyan      => "Cyan",
            Theme::Monochrome => "Monochrome",
        }
    }

    pub fn next(&self) -> Theme {
        match self {
            Theme::LamessUi  => Theme::Cyan,
            Theme::Cyan      => Theme::Monochrome,
            Theme::Monochrome => Theme::LamessUi,
        }
    }
}

impl Default for Theme {
    fn default() -> Self { Theme::LamessUi }
}

#[derive(Debug, Clone)]
pub struct ThemeColors {
    pub border_active:   Color,
    pub border_inactive: Color,
    pub highlight_fg:    Color,
    pub highlight_bg:    Color,
    pub text_primary:    Color,
    pub text_muted:      Color,
    pub success:         Color,
    pub danger:          Color,
    pub tab_active:      Color,
    pub title:           Color,
    pub status_bar_fg:   Color,
    pub help_border:     Color,
}

impl Theme {
    pub fn colors(&self) -> ThemeColors {
        match self {
            Theme::LamessUi => ThemeColors {
                border_active:   Color::Rgb(245, 208, 0),
                border_inactive: Color::Rgb(30, 30, 30),
                highlight_fg:    Color::Rgb(0, 0, 0),
                highlight_bg:    Color::Rgb(245, 208, 0),
                text_primary:    Color::Rgb(255, 255, 255),
                text_muted:      Color::Rgb(122, 122, 122),
                success:         Color::Rgb(57, 255, 135),
                danger:          Color::Rgb(255, 59, 59),
                tab_active:      Color::Rgb(245, 208, 0),
                title:           Color::Rgb(245, 208, 0),
                status_bar_fg:   Color::Rgb(122, 122, 122),
                help_border:     Color::Rgb(245, 208, 0),
            },
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
                status_bar_fg:   Color::DarkGray,
                help_border:     Color::Cyan,
            },
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
                status_bar_fg:   Color::Gray,
                help_border:     Color::White,
            },
        }
    }
}
