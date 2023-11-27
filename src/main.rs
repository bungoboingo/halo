mod validation;
mod viewer;

use crate::viewer::Viewer;
use iced::alignment::Horizontal;
use iced::font::{Family, Stretch, Style, Weight};
use iced::theme::Button;
use iced::widget::shader::Shader;
use iced::widget::{button, container, text};
use iced::{executor, font, Application, Color, Command, Element, Length, Theme};
use std::path::PathBuf;

const TITLE: &str = "ShaderBuddy";

const JETBRAINS_MONO: iced::Font = iced::Font {
    family: Family::Name("JetBrains Mono"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
};
const PIXEL_FONT: iced::Font = iced::Font {
    family: Family::Name("Pixelify Sans"),
    weight: Weight::Bold,
    stretch: Stretch::Normal,
    style: Style::Normal,
};

fn main() -> iced::Result {
    ShaderBuddy::run(iced::Settings::default())
}

struct ShaderBuddy {
    shader_path: Option<PathBuf>,
    viewer: Viewer,
    error: Option<validation::Error>,
    show_intro: bool,
}

#[derive(Clone, Debug)]
enum Message {
    PathSet(PathBuf),
    ValidateShader(String),
    CloseIntro,
    FontLoaded(Result<(), font::Error>),
    // System file open dialogue
    OpenShaderFile,
    // Show/close the editor
    ToggleEditor,
}

impl Application for ShaderBuddy {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            //TODO load from saved state
            Self {
                shader_path: None,
                viewer: Viewer::default(),
                error: None,
                show_intro: true,
            },
            //TODO maybe should add a fonts::load or something for multiple..
            Command::batch(vec![
                font::load(include_bytes!("../fonts/JetBrainsMono-Regular.ttf").as_slice())
                    .map(Message::FontLoaded),
                font::load(include_bytes!("../fonts/PixelifySans-Bold.ttf").as_slice())
                    .map(Message::FontLoaded),
            ]),
        )
    }

    fn title(&self) -> String {
        TITLE.to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {
            Message::PathSet(path) => {
                self.shader_path = Some(path);
            }
            //triggers on editor save
            Message::ValidateShader(shader_src) => {
                match validation::validate(&shader_src) {
                    Ok(_) => {
                        self.viewer.shader = shader_src.into();
                        //increment version so we can update the pipeline with new shader
                        self.viewer.version += 1;
                        self.error = None;
                    }
                    Err(error) => {
                        //TODO display wgsl validation error somewhere
                        self.error = Some(error);
                    }
                }
            }
            Message::CloseIntro => {
                self.show_intro = false;
            }
            Message::OpenShaderFile => {
                //TODO
            }
            Message::ToggleEditor => {
                //TODO
            }
            _ => {}
        }

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, iced::Renderer<Self::Theme>> {
        let bg: Element<Message> = Shader::new(&self.viewer)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        container(bg)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

struct IntroStyle;

impl container::StyleSheet for IntroStyle {
    type Style = Theme;

    fn appearance(&self, _style: &Self::Style) -> container::Appearance {
        container::Appearance {
            text_color: Some(Color::WHITE),
            background: Some(Color::BLACK.into()),
            border_color: Color::WHITE,
            border_width: 1.0,
            ..Default::default()
        }
    }
}

impl button::StyleSheet for IntroStyle {
    type Style = Theme;

    fn active(&self, _style: &Self::Style) -> button::Appearance {
        button::Appearance {
            background: Some(Color::BLACK.into()),
            text_color: Color::WHITE,
            border_width: 1.0,
            border_color: Color::WHITE,
            ..Default::default()
        }
    }
}
