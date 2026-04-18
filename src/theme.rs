use clap::ValueEnum;
use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
#[value(rename_all = "kebab-case")]
pub enum ThemeName {
    Default,
    TomorrowNightBlue,
    CursorDark,
    DarculaDark,
    DarculaLight,
    Dracula,
    Nord,
    OneDark,
    GruvboxDark,
    GruvboxLight,
    CatppuccinMocha,
    TokyoNight,
    SolarizedDark,
    SolarizedLight,
    MsDos,
}

impl ThemeName {
    pub fn display_name(self) -> &'static str {
        match self {
            Self::Default => "Default",
            Self::TomorrowNightBlue => "Tomorrow Night Blue",
            Self::CursorDark => "Cursor Dark",
            Self::DarculaDark => "Darcula Dark",
            Self::DarculaLight => "Darcula Light",
            Self::Dracula => "Dracula",
            Self::Nord => "Nord",
            Self::OneDark => "One Dark",
            Self::GruvboxDark => "Gruvbox Dark",
            Self::GruvboxLight => "Gruvbox Light",
            Self::CatppuccinMocha => "Catppuccin Mocha",
            Self::TokyoNight => "Tokyo Night",
            Self::SolarizedDark => "Solarized Dark",
            Self::SolarizedLight => "Solarized Light",
            Self::MsDos => "MS-DOS",
        }
    }

    pub fn description(self) -> &'static str {
        match self {
            Self::Default => "Balanced blue-grey theme based on the original Miro palette",
            Self::TomorrowNightBlue => "Deep blue low-glare theme used as the default",
            Self::CursorDark => "Dark editor-like palette inspired by Cursor",
            Self::DarculaDark => "JetBrains-style dark Darcula palette",
            Self::DarculaLight => "Light Darcula-inspired variant with softer contrast",
            Self::Dracula => "Purple-dark theme with vibrant accent colors",
            Self::Nord => "Arctic blue-grey palette with low saturation",
            Self::OneDark => "Slate-blue dark theme inspired by Atom One Dark",
            Self::GruvboxDark => "Warm earthy retro dark palette",
            Self::GruvboxLight => "Warm beige retro light palette",
            Self::CatppuccinMocha => "Pastel lavender dark theme",
            Self::TokyoNight => "City night dark palette with blue-violet accents",
            Self::SolarizedDark => "Scientifically-designed teal dark palette",
            Self::SolarizedLight => "Scientifically-designed ivory light palette",
            Self::MsDos => "Retro black & phosphor-green CRT terminal theme",
        }
    }

    pub fn cli_id(self) -> &'static str {
        match self {
            Self::Default => "default",
            Self::TomorrowNightBlue => "tomorrow-night-blue",
            Self::CursorDark => "cursor-dark",
            Self::DarculaDark => "darcula-dark",
            Self::DarculaLight => "darcula-light",
            Self::Dracula => "dracula",
            Self::Nord => "nord",
            Self::OneDark => "one-dark",
            Self::GruvboxDark => "gruvbox-dark",
            Self::GruvboxLight => "gruvbox-light",
            Self::CatppuccinMocha => "catppuccin-mocha",
            Self::TokyoNight => "tokyo-night",
            Self::SolarizedDark => "solarized-dark",
            Self::SolarizedLight => "solarized-light",
            Self::MsDos => "ms-dos",
        }
    }

    pub fn from_cli_id(s: &str) -> Option<ThemeName> {
        match s {
            "default" => Some(Self::Default),
            "tomorrow-night-blue" => Some(Self::TomorrowNightBlue),
            "cursor-dark" => Some(Self::CursorDark),
            "darcula-dark" => Some(Self::DarculaDark),
            "darcula-light" => Some(Self::DarculaLight),
            "dracula" => Some(Self::Dracula),
            "nord" => Some(Self::Nord),
            "one-dark" => Some(Self::OneDark),
            "gruvbox-dark" => Some(Self::GruvboxDark),
            "gruvbox-light" => Some(Self::GruvboxLight),
            "catppuccin-mocha" => Some(Self::CatppuccinMocha),
            "tokyo-night" => Some(Self::TokyoNight),
            "solarized-dark" => Some(Self::SolarizedDark),
            "solarized-light" => Some(Self::SolarizedLight),
            "ms-dos" => Some(Self::MsDos),
            _ => None,
        }
    }

    pub fn all() -> &'static [ThemeName] {
        &[
            ThemeName::TomorrowNightBlue,
            ThemeName::Default,
            ThemeName::CursorDark,
            ThemeName::DarculaDark,
            ThemeName::DarculaLight,
            ThemeName::Dracula,
            ThemeName::Nord,
            ThemeName::OneDark,
            ThemeName::GruvboxDark,
            ThemeName::GruvboxLight,
            ThemeName::CatppuccinMocha,
            ThemeName::TokyoNight,
            ThemeName::SolarizedDark,
            ThemeName::SolarizedLight,
            ThemeName::MsDos,
        ]
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Theme {
    pub id: ThemeName,
    pub app_background: Style,
    pub header: Style,
    pub header_border: Style,
    pub list_border: Style,
    pub selected_row: Style,
    pub title: Style,
    pub preview: Style,
    pub meta: Style,
    pub footer: Style,
    pub footer_hint: Style,
    pub footer_status: Style,
    pub dialog: Style,
    pub dialog_border: Style,
    pub empty_state: Style,
    pub codex_badge: Style,
    pub claude_badge: Style,
    pub opencode_badge: Style,
}

impl Theme {
    pub fn get(theme: ThemeName) -> Self {
        match theme {
            ThemeName::Default => default_theme(),
            ThemeName::TomorrowNightBlue => tomorrow_night_blue_theme(),
            ThemeName::CursorDark => cursor_dark_theme(),
            ThemeName::DarculaDark => darcula_dark_theme(),
            ThemeName::DarculaLight => darcula_light_theme(),
            ThemeName::Dracula => dracula_theme(),
            ThemeName::Nord => nord_theme(),
            ThemeName::OneDark => one_dark_theme(),
            ThemeName::GruvboxDark => gruvbox_dark_theme(),
            ThemeName::GruvboxLight => gruvbox_light_theme(),
            ThemeName::CatppuccinMocha => catppuccin_mocha_theme(),
            ThemeName::TokyoNight => tokyo_night_theme(),
            ThemeName::SolarizedDark => solarized_dark_theme(),
            ThemeName::SolarizedLight => solarized_light_theme(),
            ThemeName::MsDos => ms_dos_theme(),
        }
    }
}

fn default_theme() -> Theme {
    Theme {
        id: ThemeName::Default,
        app_background: Style::default().bg(Color::Rgb(0x14, 0x1a, 0x24)),
        header: Style::default()
            .fg(Color::Rgb(0xf8, 0xf0, 0xd8))
            .bg(Color::Rgb(0x1f, 0x36, 0x59))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x5a, 0x78, 0x9c)),
        list_border: Style::default().fg(Color::Rgb(0x5a, 0x78, 0x9c)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x3d, 0x23, 0x5e))
            .fg(Color::Rgb(0xff, 0xfa, 0xe6))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0xf4, 0xf4, 0xeb)),
        preview: Style::default().fg(Color::Rgb(0x90, 0xc6, 0xf9)),
        meta: Style::default().fg(Color::Rgb(0x94, 0x99, 0xa8)),
        footer: Style::default()
            .fg(Color::Rgb(0xd6, 0xdc, 0xe6))
            .bg(Color::Rgb(0x17, 0x1c, 0x26)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xff, 0xe0, 0x66))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0xd6, 0xdc, 0xe6)),
        dialog: Style::default()
            .fg(Color::Rgb(0xff, 0xf0, 0xe8))
            .bg(Color::Rgb(0x68, 0x23, 0x2e)),
        dialog_border: Style::default().fg(Color::Rgb(0x5a, 0x78, 0x9c)),
        empty_state: Style::default().fg(Color::Rgb(0xd6, 0xdc, 0xe6)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xff, 0xb3, 0x47))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x65, 0xd6, 0xad))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x5a, 0xa8, 0xf7))
            .add_modifier(Modifier::BOLD),
    }
}

fn tomorrow_night_blue_theme() -> Theme {
    Theme {
        id: ThemeName::TomorrowNightBlue,
        app_background: Style::default().bg(Color::Rgb(0x00, 0x24, 0x51)),
        header: Style::default()
            .fg(Color::Rgb(0xff, 0xff, 0xff))
            .bg(Color::Rgb(0x00, 0x3f, 0x8e))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x3f, 0x6f, 0xa8)),
        list_border: Style::default().fg(Color::Rgb(0x33, 0x5c, 0x8a)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x1b, 0x4b, 0x91))
            .fg(Color::Rgb(0xff, 0xff, 0xff))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0xf0, 0xf4, 0xf8)),
        preview: Style::default().fg(Color::Rgb(0x9c, 0xc7, 0xf4)),
        meta: Style::default().fg(Color::Rgb(0x7a, 0xa2, 0xc8)),
        footer: Style::default()
            .fg(Color::Rgb(0xe7, 0xef, 0xfb))
            .bg(Color::Rgb(0x00, 0x30, 0x6d)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xff, 0xd7, 0x5e))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0xd5, 0xe5, 0xf7)),
        dialog: Style::default()
            .fg(Color::Rgb(0xff, 0xf7, 0xf2))
            .bg(Color::Rgb(0x64, 0x2c, 0x3d)),
        dialog_border: Style::default().fg(Color::Rgb(0xd0, 0x87, 0x9a)),
        empty_state: Style::default().fg(Color::Rgb(0xc6, 0xd9, 0xed)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xff, 0xc5, 0x66))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x8f, 0xe0, 0xbd))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x6a, 0xb0, 0xff))
            .add_modifier(Modifier::BOLD),
    }
}

fn cursor_dark_theme() -> Theme {
    Theme {
        id: ThemeName::CursorDark,
        app_background: Style::default().bg(Color::Rgb(0x12, 0x14, 0x18)),
        header: Style::default()
            .fg(Color::Rgb(0xee, 0xf1, 0xf7))
            .bg(Color::Rgb(0x22, 0x28, 0x34))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x39, 0x47, 0x5c)),
        list_border: Style::default().fg(Color::Rgb(0x39, 0x47, 0x5c)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x2d, 0x3a, 0x52))
            .fg(Color::Rgb(0xf8, 0xfb, 0xff))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0xea, 0xee, 0xf5)),
        preview: Style::default().fg(Color::Rgb(0x97, 0xbd, 0xff)),
        meta: Style::default().fg(Color::Rgb(0x8b, 0x96, 0xa8)),
        footer: Style::default()
            .fg(Color::Rgb(0xd9, 0xdf, 0xea))
            .bg(Color::Rgb(0x1a, 0x1f, 0x29)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xff, 0xc8, 0x57))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0xd9, 0xdf, 0xea)),
        dialog: Style::default()
            .fg(Color::Rgb(0xff, 0xf0, 0xec))
            .bg(Color::Rgb(0x62, 0x2d, 0x37)),
        dialog_border: Style::default().fg(Color::Rgb(0xa1, 0x62, 0x73)),
        empty_state: Style::default().fg(Color::Rgb(0xd9, 0xdf, 0xea)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xff, 0xb8, 0x6c))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x79, 0xd2, 0xa6))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x5c, 0x8a, 0xff))
            .add_modifier(Modifier::BOLD),
    }
}

fn darcula_dark_theme() -> Theme {
    Theme {
        id: ThemeName::DarculaDark,
        app_background: Style::default().bg(Color::Rgb(0x2b, 0x2b, 0x2b)),
        header: Style::default()
            .fg(Color::Rgb(0xf1, 0xf1, 0xf1))
            .bg(Color::Rgb(0x3c, 0x3f, 0x41))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x5b, 0x60, 0x63)),
        list_border: Style::default().fg(Color::Rgb(0x5b, 0x60, 0x63)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x4e, 0x52, 0x57))
            .fg(Color::Rgb(0xff, 0xff, 0xff))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0xdc, 0xdc, 0xdc)),
        preview: Style::default().fg(Color::Rgb(0x98, 0xc3, 0xff)),
        meta: Style::default().fg(Color::Rgb(0xa4, 0xa4, 0xa4)),
        footer: Style::default()
            .fg(Color::Rgb(0xdc, 0xdc, 0xdc))
            .bg(Color::Rgb(0x31, 0x33, 0x35)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xf7, 0xce, 0x6a))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0xd0, 0xd0, 0xd0)),
        dialog: Style::default()
            .fg(Color::Rgb(0xff, 0xf2, 0xf2))
            .bg(Color::Rgb(0x6b, 0x34, 0x3d)),
        dialog_border: Style::default().fg(Color::Rgb(0xb9, 0x77, 0x82)),
        empty_state: Style::default().fg(Color::Rgb(0xc6, 0xc6, 0xc6)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xff, 0xc6, 0x6d))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x81, 0xd4, 0xbe))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x53, 0x9b, 0xf5))
            .add_modifier(Modifier::BOLD),
    }
}

fn darcula_light_theme() -> Theme {
    Theme {
        id: ThemeName::DarculaLight,
        app_background: Style::default().bg(Color::Rgb(0xf4, 0xf4, 0xf4)),
        header: Style::default()
            .fg(Color::Rgb(0x2f, 0x34, 0x39))
            .bg(Color::Rgb(0xdb, 0xdf, 0xe3))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0xa5, 0xad, 0xb8)),
        list_border: Style::default().fg(Color::Rgb(0xa5, 0xad, 0xb8)),
        selected_row: Style::default()
            .bg(Color::Rgb(0xca, 0xd9, 0xf7))
            .fg(Color::Rgb(0x1f, 0x2a, 0x36))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0x2d, 0x33, 0x39)),
        preview: Style::default().fg(Color::Rgb(0x3b, 0x6f, 0xb6)),
        meta: Style::default().fg(Color::Rgb(0x6b, 0x72, 0x7c)),
        footer: Style::default()
            .fg(Color::Rgb(0x2d, 0x33, 0x39))
            .bg(Color::Rgb(0xe5, 0xe8, 0xec)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xb0, 0x6b, 0x00))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0x4a, 0x52, 0x5c)),
        dialog: Style::default()
            .fg(Color::Rgb(0x3d, 0x15, 0x1b))
            .bg(Color::Rgb(0xf1, 0xd8, 0xdb)),
        dialog_border: Style::default().fg(Color::Rgb(0xbd, 0x8d, 0x95)),
        empty_state: Style::default().fg(Color::Rgb(0x5f, 0x67, 0x72)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xbc, 0x6f, 0x00))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x0c, 0x8a, 0x67))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x25, 0x63, 0xeb))
            .add_modifier(Modifier::BOLD),
    }
}

fn dracula_theme() -> Theme {
    Theme {
        id: ThemeName::Dracula,
        app_background: Style::default().bg(Color::Rgb(0x28, 0x2a, 0x36)),
        header: Style::default()
            .fg(Color::Rgb(0xf8, 0xf8, 0xf2))
            .bg(Color::Rgb(0x44, 0x47, 0x5a))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x62, 0x72, 0xa4)),
        list_border: Style::default().fg(Color::Rgb(0x62, 0x72, 0xa4)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x62, 0x72, 0xa4))
            .fg(Color::Rgb(0xf8, 0xf8, 0xf2))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0xf8, 0xf8, 0xf2)),
        preview: Style::default().fg(Color::Rgb(0x8b, 0xe9, 0xfd)),
        meta: Style::default().fg(Color::Rgb(0x62, 0x72, 0xa4)),
        footer: Style::default()
            .fg(Color::Rgb(0xf8, 0xf8, 0xf2))
            .bg(Color::Rgb(0x21, 0x22, 0x2c)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xff, 0xb8, 0x6c))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0xf8, 0xf8, 0xf2)),
        dialog: Style::default()
            .fg(Color::Rgb(0xff, 0xb8, 0xc6))
            .bg(Color::Rgb(0x4a, 0x10, 0x20)),
        dialog_border: Style::default().fg(Color::Rgb(0xff, 0x55, 0x55)),
        empty_state: Style::default().fg(Color::Rgb(0x62, 0x72, 0xa4)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xff, 0xb8, 0x6c))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x50, 0xfa, 0x7b))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x8b, 0xe9, 0xfd))
            .add_modifier(Modifier::BOLD),
    }
}

fn nord_theme() -> Theme {
    Theme {
        id: ThemeName::Nord,
        app_background: Style::default().bg(Color::Rgb(0x2e, 0x34, 0x40)),
        header: Style::default()
            .fg(Color::Rgb(0xec, 0xef, 0xf4))
            .bg(Color::Rgb(0x3b, 0x42, 0x52))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x4c, 0x56, 0x6a)),
        list_border: Style::default().fg(Color::Rgb(0x4c, 0x56, 0x6a)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x4c, 0x56, 0x6a))
            .fg(Color::Rgb(0xec, 0xef, 0xf4))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0xec, 0xef, 0xf4)),
        preview: Style::default().fg(Color::Rgb(0x88, 0xc0, 0xd0)),
        meta: Style::default().fg(Color::Rgb(0x61, 0x6e, 0x88)),
        footer: Style::default()
            .fg(Color::Rgb(0xd8, 0xde, 0xe9))
            .bg(Color::Rgb(0x29, 0x2e, 0x39)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xeb, 0xcb, 0x8b))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0xd8, 0xde, 0xe9)),
        dialog: Style::default()
            .fg(Color::Rgb(0xd8, 0xde, 0xe9))
            .bg(Color::Rgb(0x4c, 0x1a, 0x26)),
        dialog_border: Style::default().fg(Color::Rgb(0xbf, 0x61, 0x6a)),
        empty_state: Style::default().fg(Color::Rgb(0x61, 0x6e, 0x88)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xeb, 0xcb, 0x8b))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x88, 0xc0, 0xd0))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x5e, 0x81, 0xac))
            .add_modifier(Modifier::BOLD),
    }
}

fn one_dark_theme() -> Theme {
    Theme {
        id: ThemeName::OneDark,
        app_background: Style::default().bg(Color::Rgb(0x28, 0x2c, 0x34)),
        header: Style::default()
            .fg(Color::Rgb(0xab, 0xb2, 0xbf))
            .bg(Color::Rgb(0x35, 0x3b, 0x45))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x3e, 0x44, 0x51)),
        list_border: Style::default().fg(Color::Rgb(0x3e, 0x44, 0x51)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x3e, 0x44, 0x51))
            .fg(Color::Rgb(0xff, 0xff, 0xff))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0xab, 0xb2, 0xbf)),
        preview: Style::default().fg(Color::Rgb(0x61, 0xaf, 0xef)),
        meta: Style::default().fg(Color::Rgb(0x5c, 0x63, 0x70)),
        footer: Style::default()
            .fg(Color::Rgb(0xab, 0xb2, 0xbf))
            .bg(Color::Rgb(0x21, 0x25, 0x2b)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xe5, 0xc0, 0x7b))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0xab, 0xb2, 0xbf)),
        dialog: Style::default()
            .fg(Color::Rgb(0xf0, 0xe6, 0xe6))
            .bg(Color::Rgb(0x5c, 0x21, 0x22)),
        dialog_border: Style::default().fg(Color::Rgb(0xe0, 0x6c, 0x75)),
        empty_state: Style::default().fg(Color::Rgb(0x5c, 0x63, 0x70)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xe5, 0xc0, 0x7b))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x98, 0xc3, 0x79))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x61, 0xaf, 0xef))
            .add_modifier(Modifier::BOLD),
    }
}

fn gruvbox_dark_theme() -> Theme {
    Theme {
        id: ThemeName::GruvboxDark,
        app_background: Style::default().bg(Color::Rgb(0x28, 0x28, 0x28)),
        header: Style::default()
            .fg(Color::Rgb(0xeb, 0xdb, 0xb2))
            .bg(Color::Rgb(0x3c, 0x38, 0x36))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x50, 0x49, 0x45)),
        list_border: Style::default().fg(Color::Rgb(0x50, 0x49, 0x45)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x50, 0x49, 0x45))
            .fg(Color::Rgb(0xfb, 0xf1, 0xc7))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0xeb, 0xdb, 0xb2)),
        preview: Style::default().fg(Color::Rgb(0x83, 0xa5, 0x98)),
        meta: Style::default().fg(Color::Rgb(0x92, 0x83, 0x74)),
        footer: Style::default()
            .fg(Color::Rgb(0xd5, 0xc4, 0xa1))
            .bg(Color::Rgb(0x1d, 0x20, 0x21)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xfa, 0xbd, 0x2f))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0xd5, 0xc4, 0xa1)),
        dialog: Style::default()
            .fg(Color::Rgb(0xfb, 0xf1, 0xc7))
            .bg(Color::Rgb(0x62, 0x14, 0x14)),
        dialog_border: Style::default().fg(Color::Rgb(0xcc, 0x24, 0x1d)),
        empty_state: Style::default().fg(Color::Rgb(0x92, 0x83, 0x74)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xfa, 0xbd, 0x2f))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0xb8, 0xbb, 0x26))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x45, 0x85, 0x88))
            .add_modifier(Modifier::BOLD),
    }
}

fn gruvbox_light_theme() -> Theme {
    Theme {
        id: ThemeName::GruvboxLight,
        app_background: Style::default().bg(Color::Rgb(0xfb, 0xf1, 0xc7)),
        header: Style::default()
            .fg(Color::Rgb(0x28, 0x28, 0x28))
            .bg(Color::Rgb(0xd5, 0xc4, 0xa1))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0xa5, 0xad, 0xb8)),
        list_border: Style::default().fg(Color::Rgb(0xa5, 0xad, 0xb8)),
        selected_row: Style::default()
            .bg(Color::Rgb(0xbd, 0xae, 0x93))
            .fg(Color::Rgb(0x1d, 0x20, 0x21))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0x28, 0x28, 0x28)),
        preview: Style::default().fg(Color::Rgb(0x45, 0x85, 0x88)),
        meta: Style::default().fg(Color::Rgb(0x7c, 0x6f, 0x64)),
        footer: Style::default()
            .fg(Color::Rgb(0x50, 0x49, 0x45))
            .bg(Color::Rgb(0xec, 0xdb, 0xb2)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xb5, 0x76, 0x14))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0x66, 0x5c, 0x54)),
        dialog: Style::default()
            .fg(Color::Rgb(0xfb, 0xf1, 0xc7))
            .bg(Color::Rgb(0x9d, 0x00, 0x06)),
        dialog_border: Style::default().fg(Color::Rgb(0xcc, 0x24, 0x1d)),
        empty_state: Style::default().fg(Color::Rgb(0x7c, 0x6f, 0x64)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xb5, 0x76, 0x14))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x42, 0x7b, 0x58))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x07, 0x66, 0x78))
            .add_modifier(Modifier::BOLD),
    }
}

fn catppuccin_mocha_theme() -> Theme {
    Theme {
        id: ThemeName::CatppuccinMocha,
        app_background: Style::default().bg(Color::Rgb(0x1e, 0x1e, 0x2e)),
        header: Style::default()
            .fg(Color::Rgb(0xcd, 0xd6, 0xf4))
            .bg(Color::Rgb(0x31, 0x32, 0x44))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x45, 0x47, 0x5a)),
        list_border: Style::default().fg(Color::Rgb(0x45, 0x47, 0x5a)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x45, 0x47, 0x5a))
            .fg(Color::Rgb(0xcd, 0xd6, 0xf4))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0xcd, 0xd6, 0xf4)),
        preview: Style::default().fg(Color::Rgb(0x89, 0xb4, 0xfa)),
        meta: Style::default().fg(Color::Rgb(0x58, 0x5b, 0x70)),
        footer: Style::default()
            .fg(Color::Rgb(0xcd, 0xd6, 0xf4))
            .bg(Color::Rgb(0x18, 0x18, 0x25)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xf9, 0xe2, 0xaf))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0xcd, 0xd6, 0xf4)),
        dialog: Style::default()
            .fg(Color::Rgb(0xf2, 0xcd, 0xcd))
            .bg(Color::Rgb(0x4a, 0x16, 0x28)),
        dialog_border: Style::default().fg(Color::Rgb(0xf3, 0x8b, 0xa8)),
        empty_state: Style::default().fg(Color::Rgb(0x58, 0x5b, 0x70)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xfa, 0xb3, 0x87))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0xa6, 0xe3, 0xa1))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x89, 0xb4, 0xfa))
            .add_modifier(Modifier::BOLD),
    }
}

fn tokyo_night_theme() -> Theme {
    Theme {
        id: ThemeName::TokyoNight,
        app_background: Style::default().bg(Color::Rgb(0x1a, 0x1b, 0x26)),
        header: Style::default()
            .fg(Color::Rgb(0xc0, 0xca, 0xf5))
            .bg(Color::Rgb(0x24, 0x28, 0x3b))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x36, 0x4a, 0x82)),
        list_border: Style::default().fg(Color::Rgb(0x36, 0x4a, 0x82)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x36, 0x4a, 0x82))
            .fg(Color::Rgb(0xc0, 0xca, 0xf5))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0xc0, 0xca, 0xf5)),
        preview: Style::default().fg(Color::Rgb(0x7a, 0xa2, 0xf7)),
        meta: Style::default().fg(Color::Rgb(0x56, 0x5f, 0x89)),
        footer: Style::default()
            .fg(Color::Rgb(0xa9, 0xb1, 0xd6))
            .bg(Color::Rgb(0x16, 0x16, 0x1e)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xe0, 0xaf, 0x68))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0xa9, 0xb1, 0xd6)),
        dialog: Style::default()
            .fg(Color::Rgb(0xff, 0xc9, 0xd0))
            .bg(Color::Rgb(0x52, 0x12, 0x20)),
        dialog_border: Style::default().fg(Color::Rgb(0xf7, 0x76, 0x8e)),
        empty_state: Style::default().fg(Color::Rgb(0x56, 0x5f, 0x89)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xe0, 0xaf, 0x68))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x9e, 0xce, 0x6a))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x7a, 0xa2, 0xf7))
            .add_modifier(Modifier::BOLD),
    }
}

fn solarized_dark_theme() -> Theme {
    Theme {
        id: ThemeName::SolarizedDark,
        app_background: Style::default().bg(Color::Rgb(0x00, 0x2b, 0x36)),
        header: Style::default()
            .fg(Color::Rgb(0x83, 0x94, 0x96))
            .bg(Color::Rgb(0x07, 0x36, 0x42))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x09, 0x49, 0x58)),
        list_border: Style::default().fg(Color::Rgb(0x09, 0x49, 0x58)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x09, 0x49, 0x58))
            .fg(Color::Rgb(0xfd, 0xf6, 0xe3))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0xee, 0xe8, 0xd5)),
        preview: Style::default().fg(Color::Rgb(0x26, 0x8b, 0xd2)),
        meta: Style::default().fg(Color::Rgb(0x65, 0x7b, 0x83)),
        footer: Style::default()
            .fg(Color::Rgb(0x93, 0xa1, 0xa1))
            .bg(Color::Rgb(0x00, 0x21, 0x2b)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xb5, 0x89, 0x00))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0x93, 0xa1, 0xa1)),
        dialog: Style::default()
            .fg(Color::Rgb(0xfd, 0xf6, 0xe3))
            .bg(Color::Rgb(0x5a, 0x1a, 0x24)),
        dialog_border: Style::default().fg(Color::Rgb(0xdc, 0x32, 0x2f)),
        empty_state: Style::default().fg(Color::Rgb(0x65, 0x7b, 0x83)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xcb, 0x4b, 0x16))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x2a, 0xa1, 0x98))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x26, 0x8b, 0xd2))
            .add_modifier(Modifier::BOLD),
    }
}

fn solarized_light_theme() -> Theme {
    Theme {
        id: ThemeName::SolarizedLight,
        app_background: Style::default().bg(Color::Rgb(0xfd, 0xf6, 0xe3)),
        header: Style::default()
            .fg(Color::Rgb(0x65, 0x7b, 0x83))
            .bg(Color::Rgb(0xee, 0xe8, 0xd5))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0xa5, 0xad, 0xb8)),
        list_border: Style::default().fg(Color::Rgb(0xa5, 0xad, 0xb8)),
        selected_row: Style::default()
            .bg(Color::Rgb(0xd0, 0xcf, 0xc3))
            .fg(Color::Rgb(0x00, 0x2b, 0x36))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0x58, 0x6e, 0x75)),
        preview: Style::default().fg(Color::Rgb(0x26, 0x8b, 0xd2)),
        meta: Style::default().fg(Color::Rgb(0x93, 0xa1, 0xa1)),
        footer: Style::default()
            .fg(Color::Rgb(0x65, 0x7b, 0x83))
            .bg(Color::Rgb(0xf0, 0xeb, 0xe0)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0xb5, 0x89, 0x00))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0x58, 0x6e, 0x75)),
        dialog: Style::default()
            .fg(Color::Rgb(0x5c, 0x21, 0x22))
            .bg(Color::Rgb(0xf8, 0xd7, 0xda)),
        dialog_border: Style::default().fg(Color::Rgb(0xdc, 0x32, 0x2f)),
        empty_state: Style::default().fg(Color::Rgb(0x93, 0xa1, 0xa1)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0xcb, 0x4b, 0x16))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x2a, 0xa1, 0x98))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x26, 0x8b, 0xd2))
            .add_modifier(Modifier::BOLD),
    }
}

fn ms_dos_theme() -> Theme {
    Theme {
        id: ThemeName::MsDos,
        app_background: Style::default().bg(Color::Rgb(0x00, 0x00, 0x00)),
        header: Style::default()
            .fg(Color::Rgb(0x00, 0xff, 0x41))
            .bg(Color::Rgb(0x00, 0x1f, 0x08))
            .add_modifier(Modifier::BOLD),
        header_border: Style::default().fg(Color::Rgb(0x00, 0xb3, 0x2c)),
        list_border: Style::default().fg(Color::Rgb(0x00, 0xb3, 0x2c)),
        selected_row: Style::default()
            .bg(Color::Rgb(0x00, 0xff, 0x41))
            .fg(Color::Rgb(0x00, 0x00, 0x00))
            .add_modifier(Modifier::BOLD | Modifier::UNDERLINED),
        title: Style::default().fg(Color::Rgb(0x00, 0xff, 0x41)),
        preview: Style::default().fg(Color::Rgb(0x00, 0xcc, 0x35)),
        meta: Style::default().fg(Color::Rgb(0x00, 0x59, 0x16)),
        footer: Style::default()
            .fg(Color::Rgb(0x00, 0xb3, 0x2c))
            .bg(Color::Rgb(0x00, 0x00, 0x00)),
        footer_hint: Style::default()
            .fg(Color::Rgb(0x00, 0xff, 0x41))
            .add_modifier(Modifier::BOLD),
        footer_status: Style::default().fg(Color::Rgb(0x00, 0xb3, 0x2c)),
        dialog: Style::default()
            .fg(Color::Rgb(0x00, 0xff, 0x41))
            .bg(Color::Rgb(0x00, 0x1f, 0x08)),
        dialog_border: Style::default().fg(Color::Rgb(0x00, 0xff, 0x41)),
        empty_state: Style::default().fg(Color::Rgb(0x00, 0x59, 0x16)),
        codex_badge: Style::default()
            .fg(Color::Rgb(0x00, 0xff, 0x41))
            .add_modifier(Modifier::BOLD),
        claude_badge: Style::default()
            .fg(Color::Rgb(0x00, 0xcc, 0x35))
            .add_modifier(Modifier::BOLD),
        opencode_badge: Style::default()
            .fg(Color::Rgb(0x00, 0xcc, 0xff))
            .add_modifier(Modifier::BOLD),
    }
}

#[cfg(test)]
mod tests {
    use super::{Theme, ThemeName};

    #[test]
    fn resolves_tomorrow_night_blue_theme() {
        let theme = Theme::get(ThemeName::TomorrowNightBlue);
        assert_eq!(theme.id, ThemeName::TomorrowNightBlue);
    }

    #[test]
    fn resolves_default_theme() {
        let theme = Theme::get(ThemeName::Default);
        assert_eq!(theme.id, ThemeName::Default);
    }

    #[test]
    fn lists_supported_themes() {
        assert_eq!(
            ThemeName::all(),
            &[
                ThemeName::TomorrowNightBlue,
                ThemeName::Default,
                ThemeName::CursorDark,
                ThemeName::DarculaDark,
                ThemeName::DarculaLight,
                ThemeName::Dracula,
                ThemeName::Nord,
                ThemeName::OneDark,
                ThemeName::GruvboxDark,
                ThemeName::GruvboxLight,
                ThemeName::CatppuccinMocha,
                ThemeName::TokyoNight,
                ThemeName::SolarizedDark,
                ThemeName::SolarizedLight,
                ThemeName::MsDos,
            ]
        );
    }

    #[test]
    fn resolves_dracula_theme() {
        let theme = Theme::get(ThemeName::Dracula);
        assert_eq!(theme.id, ThemeName::Dracula);
    }

    #[test]
    fn resolves_nord_theme() {
        let theme = Theme::get(ThemeName::Nord);
        assert_eq!(theme.id, ThemeName::Nord);
    }

    #[test]
    fn resolves_one_dark_theme() {
        let theme = Theme::get(ThemeName::OneDark);
        assert_eq!(theme.id, ThemeName::OneDark);
    }

    #[test]
    fn resolves_gruvbox_dark_theme() {
        let theme = Theme::get(ThemeName::GruvboxDark);
        assert_eq!(theme.id, ThemeName::GruvboxDark);
    }

    #[test]
    fn resolves_gruvbox_light_theme() {
        let theme = Theme::get(ThemeName::GruvboxLight);
        assert_eq!(theme.id, ThemeName::GruvboxLight);
    }

    #[test]
    fn resolves_catppuccin_mocha_theme() {
        let theme = Theme::get(ThemeName::CatppuccinMocha);
        assert_eq!(theme.id, ThemeName::CatppuccinMocha);
    }

    #[test]
    fn resolves_tokyo_night_theme() {
        let theme = Theme::get(ThemeName::TokyoNight);
        assert_eq!(theme.id, ThemeName::TokyoNight);
    }

    #[test]
    fn resolves_solarized_dark_theme() {
        let theme = Theme::get(ThemeName::SolarizedDark);
        assert_eq!(theme.id, ThemeName::SolarizedDark);
    }

    #[test]
    fn resolves_solarized_light_theme() {
        let theme = Theme::get(ThemeName::SolarizedLight);
        assert_eq!(theme.id, ThemeName::SolarizedLight);
    }

    #[test]
    fn from_cli_id_parses_known_ids() {
        assert_eq!(ThemeName::from_cli_id("dracula"), Some(ThemeName::Dracula));
        assert_eq!(
            ThemeName::from_cli_id("tomorrow-night-blue"),
            Some(ThemeName::TomorrowNightBlue)
        );
        assert_eq!(
            ThemeName::from_cli_id("catppuccin-mocha"),
            Some(ThemeName::CatppuccinMocha)
        );
    }

    #[test]
    fn from_cli_id_returns_none_for_unknown() {
        assert_eq!(ThemeName::from_cli_id("unknown"), None);
        assert_eq!(ThemeName::from_cli_id(""), None);
    }

    #[test]
    fn cli_id_matches_kebab_case() {
        assert_eq!(ThemeName::TomorrowNightBlue.cli_id(), "tomorrow-night-blue");
        assert_eq!(ThemeName::GruvboxDark.cli_id(), "gruvbox-dark");
        assert_eq!(ThemeName::CatppuccinMocha.cli_id(), "catppuccin-mocha");
        assert_eq!(ThemeName::SolarizedLight.cli_id(), "solarized-light");
        assert_eq!(ThemeName::MsDos.cli_id(), "ms-dos");
    }

    #[test]
    fn resolves_ms_dos_theme() {
        let theme = Theme::get(ThemeName::MsDos);
        assert_eq!(theme.id, ThemeName::MsDos);
    }

    #[test]
    fn from_cli_id_parses_ms_dos() {
        assert_eq!(ThemeName::from_cli_id("ms-dos"), Some(ThemeName::MsDos));
    }
}
