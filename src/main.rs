mod viewer;

use crate::viewer::Viewer;
use iced::{executor, Application, Command, Element, Length, Renderer, Theme};
use iced_graphics::custom;
use std::time::Instant;

fn main() -> iced::Result {
    ShaderBuddy::run(iced::Settings::default())
}

struct ShaderBuddy {
    start: Instant,
}

#[derive(Debug, Clone)]
enum Message {}

impl Application for ShaderBuddy {
    type Executor = executor::Default;
    type Message = Message;
    type Theme = Theme;
    type Flags = ();

    fn new(_flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            Self {
                start: Instant::now(),
            },
            Command::none(),
        )
    }

    fn title(&self) -> String {
        "Example".to_string()
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        match message {}

        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message, Renderer<Self::Theme>> {
        custom::Shader::new(Viewer::new(self.start))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}
