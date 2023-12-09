use iced::widget::{button, checkbox, container, pane_grid, text, text_editor};
use iced::{application, Color};

// Base
const DARKEST_BLUE: Color = Color::from_rgb8(18, 42, 79);
const DARK_BLUE: Color = Color::from_rgb8(38, 62, 99);
const MEDIUM_BLUE: Color = Color::from_rgb8(84, 111, 149);
const NEUTRAL_BLUE: Color = Color::from_rgb8(113, 134, 162);
const LIGHT_BLUE: Color = Color::from_rgb8(158, 175, 195);
// Accents
const SALMON: Color = Color::from_rgb8(235, 94, 85);
const GOLD: Color = Color::from_rgb8(255, 159, 28);
const OFF_WHITE: Color = Color::from_rgb8(242, 239, 233);
const LIGHT_GREY: Color = Color::from_rgb8(226, 226, 226);

const DISABLED: Color = Color::from_rgb8(153,158,162);

//borders
const BORDER_RADIUS: f32 = 5.0;
const BORDER_WIDTH: f32 = 2.0;

/// Reduces intensity by 10%
fn dim(color: Color) -> Color {
    Color {
        r: color.r * 0.10,
        g: color.g * 0.10,
        b: color.b * 0.10,
        a: color.a,
    }
}

#[derive(Default)]
pub enum Theme {
    Light,
    #[default]
    Dark,
}

#[derive(Default)]
struct Application;

impl application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        match self {
            Theme::Light => application::Appearance {
                background_color: LIGHT_BLUE,
                text_color: DARKEST_BLUE,
            },
            Theme::Dark => application::Appearance {
                background_color: DARK_BLUE,
                text_color: LIGHT_GREY,
            },
        }
    }
}

#[derive(Default)]
pub enum Container {
    Tooltip,
    #[default]
    None,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        match self {
            Theme::Light => match style {
                Container::Tooltip => container::Appearance {
                    text_color: Some(DARKEST_BLUE),
                    background: Some(LIGHT_BLUE.into()),
                    border_radius: BORDER_RADIUS.into(),
                    border_width: BORDER_WIDTH,
                    border_color: DARK_BLUE,
                },
                Container::None => container::Appearance::default(),
            },
            Theme::Dark => match style {
                Container::Tooltip => container::Appearance {
                    text_color: Some(LIGHT_GREY),
                    background: Some(DARK_BLUE.into()),
                    border_radius: BORDER_RADIUS.into(),
                    border_width: BORDER_WIDTH,
                    border_color: DARKEST_BLUE,
                },
                Container::None => container::Appearance::default(),
            },
        }
    }
}

#[derive(Default, Clone)]
pub enum Text {
    #[default]
    Primary,
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, _style: Self::Style) -> text::Appearance {
        match self {
            Theme::Light => text::Appearance {
                color: Some(DARKEST_BLUE),
            },
            Theme::Dark => text::Appearance {
                color: Some(LIGHT_GREY),
            },
        }
    }
}

#[derive(Default, Clone)]
pub struct PaneGrid;

impl pane_grid::StyleSheet for Theme {
    type Style = PaneGrid;

    fn hovered_region(&self, style: &Self::Style) -> pane_grid::Appearance {
        match self {
            Theme::Light => pane_grid::Appearance {
                background: LIGHT_BLUE.into(),
                border_width: 0.0,
                border_color: Default::default(),
                border_radius: Default::default(),
            },
            Theme::Dark => pane_grid::Appearance {
                background: DARK_BLUE.into(),
                border_width: 0.0,
                border_color: Default::default(),
                border_radius: Default::default(),
            },
        }
    }

    fn picked_split(&self, style: &Self::Style) -> Option<pane_grid::Line> {
        Some(pane_grid::Line {
            color: SALMON,
            width: 5.0,
        })
    }

    fn hovered_split(&self, style: &Self::Style) -> Option<pane_grid::Line> {
        Some(pane_grid::Line {
            color: GOLD,
            width: 5.0,
        })
    }
}

#[derive(Default)]
pub enum Button {
    #[default]
    Control,
}

impl button::StyleSheet for Theme {
    type Style = Button;

    fn active(&self, style: &Self::Style) -> button::Appearance {
        match self {
            Theme::Light => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(DARK_BLUE.into()),
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Default::default(),
                text_color: LIGHT_GREY,
            },
            Theme::Dark => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(MEDIUM_BLUE.into()),
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Default::default(),
                text_color: DARKEST_BLUE,
            }
        }
    }

    fn hovered(&self, style: &Self::Style) -> button::Appearance {
        match self {
            Theme::Light => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(dim(MEDIUM_BLUE).into()),
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Default::default(),
                text_color: DARKEST_BLUE,
            },
            Theme::Dark => button::Appearance {
                shadow_offset: Default::default(),
                background: Some(dim(MEDIUM_BLUE).into()),
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Default::default(),
                text_color: MEDIUM_BLUE,
            }
        }
    }

    fn pressed(&self, style: &Self::Style) -> button::Appearance {
        self.hovered(style)
    }
}

#[derive(Default)]
pub struct Checkbox;

impl checkbox::StyleSheet for Theme {
    type Style = Checkbox;

    fn active(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        match self {
            Theme::Light => checkbox::Appearance {
                background: if is_checked {
                    MEDIUM_BLUE
                } else {
                    DISABLED
                }.into(),
                icon_color: LIGHT_GREY,
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Some(DARKEST_BLUE),
            },
            Theme::Dark => checkbox::Appearance {
                background: if is_checked {
                    MEDIUM_BLUE
                } else {
                    DISABLED
                }.into(),
                icon_color: LIGHT_GREY,
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Some(LIGHT_GREY),
            },
        }
    }

    fn hovered(&self, style: &Self::Style, is_checked: bool) -> checkbox::Appearance {
        match self {
            Theme::Light => checkbox::Appearance {
                background: if is_checked {
                    dim(MEDIUM_BLUE)
                } else {
                    DISABLED
                }.into(),
                icon_color: LIGHT_GREY,
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Some(DARKEST_BLUE),
            },
            Theme::Dark => checkbox::Appearance {
                background: if is_checked {
                    dim(MEDIUM_BLUE)
                } else {
                    DISABLED
                }.into(),
                icon_color: LIGHT_GREY,
                border_radius: 2.0.into(),
                border_width: 0.0,
                border_color: Default::default(),
                text_color: Some(LIGHT_GREY),
            },
        }
    }
}

#[derive(Default)]
struct Editor;

impl text_editor::StyleSheet for Theme {
    type Style = Editor;

    fn active(&self, style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: Color::TRANSPARENT.into(),
            border_radius: Default::default(),
            border_width: 0.0,
            border_color: Default::default(),
        }
    }

    fn focused(&self, style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: Color::TRANSPARENT.into(),
            border_radius: Default::default(),
            border_width: 0.0,
            border_color: Default::default(),
        }
    }

    fn placeholder_color(&self, style: &Self::Style) -> Color {
        DISABLED
    }

    fn value_color(&self, style: &Self::Style) -> Color {
        match self {
            Theme::Light => DARKEST_BLUE,
            Theme::Dark => LIGHT_GREY,
        }
    }

    fn disabled_color(&self, style: &Self::Style) -> Color {
        DISABLED
    }

    fn selection_color(&self, style: &Self::Style) -> Color {
        match self {
            Theme::Light => MEDIUM_BLUE,
            Theme::Dark => MEDIUM_BLUE,
        }
    }

    fn disabled(&self, style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: Color::TRANSPARENT.into(),
            border_radius: Default::default(),
            border_width: 0.0,
            border_color: Default::default(),
        }
    }
}
