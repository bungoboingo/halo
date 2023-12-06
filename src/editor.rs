use iced::alignment::{Horizontal, Vertical};
use iced::highlighter::Highlighter;
use iced::widget::{button, container, row, text, text_editor};
use iced::{alignment, highlighter, keyboard, Alignment, Color, Command, Element, Font, Length};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Message {
    Action(text_editor::Action),
    Validate,
    New,
    Open,
    Loaded(Result<(PathBuf, Arc<String>), file::Error>),
    Save,
    Saved(Result<PathBuf, file::Error>),
}

pub enum Event {
    None,
    UpdatePipeline(String),
}

pub struct Editor {
    content: text_editor::Content,
    theme: highlighter::Theme,
    shader_path: Option<PathBuf>,
    validation_status: validation::Status,
    is_loading: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            content: text_editor::Content::with_text(include_str!(
                "viewer/shaders/default_frag.wgsl"
            )),
            theme: highlighter::Theme::Base16Mocha,
            shader_path: None,
            validation_status: validation::Status::default(),
            is_loading: false,
        }
    }
}

impl Editor {
    pub fn keypress(
        &self,
        key: keyboard::KeyCode,
        modifiers: keyboard::Modifiers,
    ) -> Option<Message> {
        match key {
            keyboard::KeyCode::Enter if modifiers.shift() => Some(Message::Validate),
            keyboard::KeyCode::S if modifiers.command() => Some(Message::Save),
            _ => None,
        }
    }

    pub fn update(&mut self, update: Message) -> (Event, Command<Message>) {
        match update {
            Message::Action(action) => {
                if action.is_edit() {
                    self.validation_status = validation::Status::NeedsValidation
                }
                self.content.perform(action);
            }
            Message::New => {
                //TODO
            }
            Message::Open => {
                //TODO
            }
            Message::Loaded(result) => {
                if let Ok((path, shader)) = result {
                    self.shader_path = Some(path);
                    self.content = text_editor::Content::with_text(&shader);
                }

                //TODO show loading error somewhere
                self.is_loading = false;
            }
            Message::Save => {
                return if self.is_loading {
                    (Event::None, Command::none())
                } else {
                    let shader = self.content.text();

                    (
                        Event::None,
                        Command::perform(
                            file::save(self.shader_path.clone(), shader),
                            Message::Saved,
                        ),
                    )
                }
            }
            Message::Saved(_result) => {
                //TODO show some success feedback
            }
            Message::Validate => {
                self.validation_status = validation::Status::Validating;

                let shader = self.content.text();

                match validation::validate(&shader) {
                    Ok(_) => {
                        self.validation_status = validation::Status::Validated;
                        return (Event::UpdatePipeline(shader), Command::none());
                    }
                    Err(error) => {
                        self.validation_status = validation::Status::Invalid(error);
                    }
                }
            }
        }

        (Event::None, Command::none())
    }

    pub fn view(&self) -> Element<Message> {
        container(
            text_editor(&self.content)
                .highlight::<Highlighter>(
                    highlighter::Settings {
                        theme: self.theme,
                        // for now, close enough :-P
                        extension: "rs".to_string(), //TODO wgsl
                    },
                    |highlight, _theme| highlight.to_format(),
                )
                .on_action(Message::Action),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }

    pub fn title_bar(&self) -> Element<Message> {
        let new_icon = icon('\u{e804}');
        let open_icon = icon('\u{f115}');
        let save_icon = icon('\u{e800}');

        let validation_controls = container(
            row![
                container(self.validation_status.icon())
                    .width(34)
                    .height(34)
                    .center_y(),
                text(format!("{}", self.validation_status)),
            ]
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Left);

        let file_controls = container(
            row![
                button(new_icon).width(34).height(34).on_press(Message::New),
                button(open_icon)
                    .width(34)
                    .height(34)
                    .on_press(Message::Open),
                button(save_icon)
                    .width(34)
                    .height(34)
                    .on_press(Message::Save),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Right);

        row![validation_controls, file_controls]
            .width(Length::Fill)
            .padding([10, 15, 10, 15])
            .align_items(Alignment::Center)
            .into()
    }
}

mod validation {
    use crate::editor::{icon, Message};
    use iced::Element;
    use naga::valid::Capabilities;
    use std::fmt::Formatter;

    #[derive(Default, Debug)]
    pub enum Status {
        #[default]
        Validated,
        Validating,
        Invalid(Error),
        NeedsValidation,
    }

    impl std::fmt::Display for Status {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let str = match self {
                Status::Validated => "Validated!",
                Status::Validating => "Validating...",
                Status::Invalid(_) => "Invalid shader!",
                Status::NeedsValidation => "Needs validation!",
            };

            write!(f, "{str}")
        }
    }

    impl Status {
        pub fn icon(&self) -> Element<Message> {
            match self {
                Status::Validated => icon('\u{e801}'),
                Status::Invalid(_) => icon('\u{e802}'),
                Status::Validating => icon('\u{e803}'),
                Status::NeedsValidation => icon('\u{e803}'),
            }
        }
    }

    //assumes shader is wgsl
    pub fn validate(shader: &str) -> Result<(), Error> {
        //parse separately so we can show errors instead of panicking on pipeline creation
        let parsed =
            naga::front::wgsl::parse_str(shader).map_err(|err| Error::Parse(err.to_string()))?;

        naga::valid::Validator::new(
            naga::valid::ValidationFlags::default(),
            Capabilities::all(), //TODO get from device capabilities
        )
        .validate(&parsed)
        .map_err(|err| Error::Validation(err.to_string()))?;

        Ok(())
    }

    #[derive(thiserror::Error, Debug)]
    pub enum Error {
        #[error("Shader parsing error: {0}")]
        Parse(String),
        #[error("Validation error: {0}")]
        Validation(String),
    }
}

pub mod file {
    use std::path::{Path, PathBuf};
    use std::sync::Arc;
    use tokio::io;

    pub async fn load(path: PathBuf) -> Result<(PathBuf, Arc<String>), Error> {
        let contents = tokio::fs::read_to_string(&path)
            .await
            .map(Arc::new)
            .map_err(|error| Error::IoError(error.kind()))?;

        Ok((path, contents))
    }

    pub async fn save(path: Option<PathBuf>, contents: String) -> Result<PathBuf, Error> {
        let path = if let Some(path) = path {
            path
        } else {
            //TODO this lags UI
            rfd::AsyncFileDialog::new()
                .save_file()
                .await
                .as_ref()
                .map(rfd::FileHandle::path)
                .map(Path::to_owned)
                .ok_or(Error::SaveDialogueClosed)?
        };

        tokio::fs::write(&path, contents)
            .await
            .map_err(|error| Error::IoError(error.kind()))?;

        Ok(path)
    }

    #[derive(Debug, Clone)]
    pub(crate) enum Error {
        IoError(io::ErrorKind),
        SaveDialogueClosed,
    }
}

pub fn icon<'a, Message: 'static>(char: char) -> Element<'a, Message> {
    const FONT: Font = Font::with_name("halo-icons");

    text(char)
        .font(FONT)
        .horizontal_alignment(Horizontal::Center)
        .vertical_alignment(Vertical::Center)
        .into()
}
