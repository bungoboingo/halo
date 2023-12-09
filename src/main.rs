mod editor;
mod preferences;
mod theme;
mod viewer;

use crate::editor::{Editor, Event};
use crate::preferences::Preferences;
use crate::viewer::Viewer;
use iced::font::{Family, Stretch, Style, Weight};
use iced::widget::pane_grid::Configuration;
use iced::widget::{container, pane_grid, PaneGrid};
use iced::{
    executor, keyboard, window, Application, Background, Color, Command, Element, Font, Length,
    Subscription, Theme,
};
use std::sync::Arc;

const HALO: &str = "Halo";
pub type FragmentShader = String;

const JETBRAINS_MONO: Font = Font {
    family: Family::Name("JetBrains Mono"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

fn main() -> iced::Result {
    Halo::run(iced::Settings {
        fonts: vec![
            include_bytes!("../fonts/JetBrainsMono-Regular.ttf")
                .as_slice()
                .into(),
            include_bytes!("../fonts/halo-icons.ttf").as_slice().into(),
        ],
        window: window::Settings {
            size: (1600, 900),
            ..Default::default()
        },
        default_font: Font::MONOSPACE,
        ..Default::default()
    })
}

struct Halo {
    viewer: Viewer,
    editor: Editor,
    panes: pane_grid::State<Pane>,
}

//TODO toggle editor
#[derive(Clone, Debug)]
enum Message {
    PaneResized(pane_grid::ResizeEvent),
    Editor(editor::Message),
    KeyPressed {
        key: keyboard::KeyCode,
        modifiers: keyboard::Modifiers,
    },
    Loaded(Result<(Preferences, Arc<FragmentShader>), preferences::Error>),
}

impl Application for Halo {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            //TODO save settings
            Self {
                viewer: Viewer::default(),
                editor: Editor::default(),
                panes: pane_grid::State::with_configuration(Configuration::Split {
                    axis: pane_grid::Axis::Vertical,
                    ratio: 0.5,
                    a: Box::new(Configuration::Pane(Pane::Viewer)),
                    b: Box::new(Configuration::Pane(Pane::Editor)),
                }),
            },
            //TODO load last shader file from settings
            Command::perform(preferences::load(), Message::Loaded),
        )
    }

    fn title(&self) -> String {
        HALO.to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::Editor(msg) => {
                let (event, cmd) = self.editor.update(msg);

                match event {
                    Event::UpdatePipeline(shader) => {
                        self.viewer.last_valid_shader = shader;
                        self.viewer.version += 1;
                    }
                    _ => {}
                };

                return cmd.map(Message::Editor);
            }
            Message::PaneResized(pane_grid::ResizeEvent { split, ratio }) => {
                self.panes.resize(split, ratio);
            }
            Message::KeyPressed { key, modifiers } => {
                if let Some(msg) = self.editor.keypress(key, modifiers).map(Message::Editor) {
                    return self.update(msg);
                }
            }
            Message::Loaded(result) => {
                return self.update(Message::Editor(editor::Message::Init(result)));
            }
        }

        Command::none()
    }

    fn view(&self) -> Element<Message> {
        let panes = PaneGrid::new(&self.panes, |_id, pane, _is_maximized| {
            pane.view(&self.editor, &self.viewer).into()
        })
        .on_resize(10, Message::PaneResized);

        container(panes)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn theme(&self) -> Self::Theme {
        Theme::Dark
    }

    fn subscription(&self) -> Subscription<Self::Message> {
        keyboard::on_key_press(|key, modifiers| Some(Message::KeyPressed { key, modifiers }))
    }
}

enum Pane {
    Viewer,
    Editor,
}

impl Pane {
    fn view<'a>(&'a self, editor: &'a Editor, viewer: &'a Viewer) -> pane_grid::Content<Message> {
        match self {
            Self::Viewer => viewer.content(),
            Self::Editor => pane_grid::Content::new(editor.view().map(Message::Editor)).title_bar(
                pane_grid::TitleBar::new(editor.title_bar().map(Message::Editor)),
            ),
        }
    }
}

struct PaneStyle;

impl pane_grid::StyleSheet for PaneStyle {
    type Style = Theme;

    fn hovered_region(&self, style: &Self::Style) -> pane_grid::Appearance {
        pane_grid::Appearance {
            background: Background::Color(style.extended_palette().primary.base.color),
            border_width: 0.0,
            border_color: Color::BLACK,
            border_radius: 0.0.into(),
        }
    }

    fn picked_split(&self, style: &Self::Style) -> Option<pane_grid::Line> {
        Some(pane_grid::Line {
            color: style.extended_palette().primary.base.color,
            width: 10.0,
        })
    }

    fn hovered_split(&self, style: &Self::Style) -> Option<pane_grid::Line> {
        Some(pane_grid::Line {
            color: style.extended_palette().secondary.base.color,
            width: 10.0,
        })
    }
}
