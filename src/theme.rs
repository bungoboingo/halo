use iced::widget::{button, checkbox, container, pane_grid, scrollable, text, text_editor};
use iced::{application, Color};

//const OFF_WHITE: Color = Color::from_rgb8(242, 239, 233);

struct Palette {
    pub base: Color,
    pub base_darker: Color,
    pub base_darkest: Color,
    pub base_lighter: Color,
    pub base_lightest: Color,
    pub background: Color,
    pub text: Color,
    pub disabled: Color,
    pub accent: Color,
    pub accent_secondary: Color,
    pub error: Color,
}

impl Default for Palette {
    fn default() -> Self {
        Self {
            base: Color::from_rgb8(84, 111, 149),             // MEDIUM BLUE
            base_darker: Color::from_rgb8(38, 62, 99),        // DARK BLUE
            base_darkest: Color::from_rgb8(22, 30, 59),       // DARK BLUE
            base_lighter: Color::from_rgb8(113, 134, 162),    // LIGHTER BLUE
            base_lightest: Color::from_rgb8(158, 175, 195),   // LIGHTEST BLUE,
            background: Color::from_rgb8(12, 12, 30),         // DARKEST BLUE
            text: Color::from_rgb8(226, 226, 226),            // LIGHT GREY
            disabled: Color::from_rgb8(153, 158, 162),        // MEDIUM GREY,
            accent: Color::from_rgb8(235, 94, 85),            // SALMON
            accent_secondary: Color::from_rgb8(255, 159, 28), // GOLD
            error: Color::from_rgb8(255, 77, 77),             // ERROR RED
        }
    }
}

//borders
const BORDER_RADIUS: f32 = 5.0;
const BORDER_WIDTH: f32 = 2.0;

/// Reduces intensity by 10%
fn dim(color: Color) -> Color {
    Color {
        r: color.r * 0.70,
        g: color.g * 0.70,
        b: color.b * 0.70,
        a: color.a,
    }
}

#[derive(Default)]
pub enum Theme {
    Light,
    #[default]
    Dark,
}

impl Theme {
    pub fn palette(&self) -> Palette {
        match self {
            //TODO light palette
            Theme::Light => Palette::default(),
            Theme::Dark => Palette::default(),
        }
    }
}

#[derive(Default)]
pub struct Application;

impl application::StyleSheet for Theme {
    type Style = Application;

    fn appearance(&self, _style: &Self::Style) -> application::Appearance {
        let palette = self.palette();

        application::Appearance {
            background_color: palette.background,
            text_color: palette.text,
        }
    }
}

#[derive(Default)]
pub enum Container {
    Tooltip,
    Controls,
    Error,
    #[default]
    None,
}

impl container::StyleSheet for Theme {
    type Style = Container;

    fn appearance(&self, style: &Self::Style) -> container::Appearance {
        let palette = self.palette();

        match style {
            Container::Tooltip => container::Appearance {
                text_color: Some(palette.text),
                background: Some(palette.base_darker.into()),
                border_radius: BORDER_RADIUS.into(),
                border_width: BORDER_WIDTH,
                border_color: palette.base_darkest,
            },
            Container::None => container::Appearance::default(),
            Container::Controls => container::Appearance {
                text_color: Some(palette.text),
                background: Some(palette.base_darkest.into()),
                border_radius: Default::default(),
                border_width: 0.0,
                border_color: Default::default(),
            },
            Container::Error => container::Appearance {
                text_color: Some(palette.error),
                background: Some(palette.base_darkest.into()),
                border_radius: Default::default(),
                border_width: 1.0,
                border_color: palette.error,
            },
        }
    }
}

#[derive(Default, Clone)]
pub enum Text {
    #[default]
    Primary,
    Error,
}

impl text::StyleSheet for Theme {
    type Style = Text;

    fn appearance(&self, style: Self::Style) -> text::Appearance {
        let palette = self.palette();

        match style {
            Text::Primary => text::Appearance {
                color: Some(palette.text),
            },
            Text::Error => text::Appearance {
                color: Some(palette.error),
            },
        }
    }
}

#[derive(Default, Clone)]
pub struct PaneGrid;

impl pane_grid::StyleSheet for Theme {
    type Style = PaneGrid;

    fn hovered_region(&self, _style: &Self::Style) -> pane_grid::Appearance {
        let palette = self.palette();

        pane_grid::Appearance {
            background: palette.background.into(),
            border_width: 0.0,
            border_color: Default::default(),
            border_radius: Default::default(),
        }
    }

    fn picked_split(&self, _style: &Self::Style) -> Option<pane_grid::Line> {
        let palette = self.palette();

        Some(pane_grid::Line {
            color: palette.accent_secondary,
            width: 5.0,
        })
    }

    fn hovered_split(&self, _style: &Self::Style) -> Option<pane_grid::Line> {
        let palette = self.palette();

        Some(pane_grid::Line {
            color: palette.accent,
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

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        let palette = self.palette();

        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(palette.base.into()),
            border_radius: 2.0.into(),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: palette.text,
        }
    }

    fn hovered(&self, _style: &Self::Style) -> button::Appearance {
        let palette = self.palette();

        button::Appearance {
            shadow_offset: Default::default(),
            background: Some(palette.base_lighter.into()),
            border_radius: 2.0.into(),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: palette.text,
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

    fn active(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let palette = self.palette();

        checkbox::Appearance {
            background: palette.base.into(),
            icon_color: palette.text,
            border_radius: 2.0.into(),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: Some(palette.text),
        }
    }

    fn hovered(&self, _style: &Self::Style, _is_checked: bool) -> checkbox::Appearance {
        let palette = self.palette();

        checkbox::Appearance {
            background: palette.base_lighter.into(),
            icon_color: palette.text,
            border_radius: 2.0.into(),
            border_width: 0.0,
            border_color: Default::default(),
            text_color: Some(palette.text),
        }
    }
}

#[derive(Default)]
pub struct Editor;

impl text_editor::StyleSheet for Theme {
    type Style = Editor;

    fn active(&self, _style: &Self::Style) -> text_editor::Appearance {
        let palette = self.palette();

        text_editor::Appearance {
            background: palette.background.into(),
            border_radius: Default::default(),
            border_width: 0.0,
            border_color: Default::default(),
        }
    }

    fn focused(&self, _style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: self.palette().background.into(),
            border_radius: Default::default(),
            border_width: 0.0,
            border_color: Default::default(),
        }
    }

    fn placeholder_color(&self, _style: &Self::Style) -> Color {
        self.palette().disabled
    }

    fn value_color(&self, _style: &Self::Style) -> Color {
        self.palette().text
    }

    fn disabled_color(&self, _style: &Self::Style) -> Color {
        self.palette().disabled
    }

    fn selection_color(&self, _style: &Self::Style) -> Color {
        self.palette().base_darker
    }

    fn disabled(&self, _style: &Self::Style) -> text_editor::Appearance {
        text_editor::Appearance {
            background: Color::TRANSPARENT.into(),
            border_radius: Default::default(),
            border_width: 0.0,
            border_color: Default::default(),
        }
    }
}

#[derive(Default)]
pub struct Scrollable;

impl scrollable::StyleSheet for Theme {
    type Style = Scrollable;

    fn active(&self, _style: &Self::Style) -> scrollable::Scrollbar {
        let palette = self.palette();

        scrollable::Scrollbar {
            background: Some(palette.base_darker.into()),
            border_radius: BORDER_RADIUS.into(),
            border_width: 0.0,
            border_color: Default::default(),
            scroller: scrollable::Scroller {
                color: palette.error,
                border_radius: Default::default(),
                border_width: 0.0,
                border_color: Default::default(),
            },
        }
    }

    fn hovered(&self, style: &Self::Style, _is_mouse_over_scrollbar: bool) -> scrollable::Scrollbar {
        self.active(style)
    }
}
