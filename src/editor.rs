mod highlighter;

use crate::editor::highlighter::Highlighter;
use iced::widget::{button, checkbox, container, row, text, text_editor, tooltip};
use iced::{alignment, keyboard, theme, Alignment, Command, Element, Font, Length};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Message {
    Action(text_editor::Action),
    Validate,
    AutoValidate(bool),
    New,
    Open,
    Loaded(Result<(PathBuf, Arc<String>), file::Error>),
    Save,
    Saved(Result<PathBuf, file::Error>),
    Undo,
    Redo,
    Search,
    Indent,
}

pub enum Event {
    None,
    UpdatePipeline(String),
}

pub struct Editor {
    content: text_editor::Content,
    theme: iced::highlighter::Theme,
    shader_path: Option<PathBuf>,
    validation_status: validation::Status,
    auto_validate: bool,
    is_loading: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self {
            content: text_editor::Content::with_text(include_str!(
                "viewer/shaders/default_frag.wgsl"
            )),
            theme: iced::highlighter::Theme::Base16Mocha,
            shader_path: None,
            validation_status: validation::Status::default(),
            auto_validate: false,
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
            keyboard::KeyCode::Enter if modifiers.control() => Some(Message::Validate),
            keyboard::KeyCode::S if modifiers.command() => Some(Message::Save),
            keyboard::KeyCode::Z if modifiers.command() => Some(Message::Undo),
            keyboard::KeyCode::Y if modifiers.command() => Some(Message::Redo),
            keyboard::KeyCode::F if modifiers.command() => Some(Message::Search),
            keyboard::KeyCode::Tab => Some(Message::Indent),
            _ => None,
        }
    }

    pub fn update(&mut self, update: Message) -> (Event, Command<Message>) {
        match update {
            Message::Action(action) => {
                //TODO fix not being able to use hotkeys while text editor is focused
                //TODO auto validation after idk. Like 2 seconds after editing.
                let validate = if action.is_edit() {
                    self.validation_status = validation::Status::NeedsValidation;
                    self.auto_validate
                } else {
                    false
                };

                self.content.perform(action);

                if validate {
                    return self.update(Message::Validate);
                }
            }
            Message::New => {
                let empty_shader = include_str!("viewer/shaders/empty_frag.wgsl");

                self.shader_path = None;
                self.content = text_editor::Content::with_text(empty_shader);

                return (
                    Event::UpdatePipeline(empty_shader.to_string()),
                    Command::none(),
                );
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
                        println!("Failed to validate: {error:?}");
                        self.validation_status = validation::Status::Invalid(error);
                    }
                }
            }
            Message::AutoValidate(checked) => {
                self.auto_validate = checked;
            }
            Message::Undo => {
                //TODO!
            }
            Message::Redo => {
                //TODO!
            }
            Message::Indent => {
                //TODO!
            }
            Message::Search => {
                //TODO!
            }
        }

        (Event::None, Command::none())
    }

    pub fn view(&self) -> Element<Message> {
        let errors =
            if let validation::Status::Invalid(validation::Error::Parse { message, errors }) =
                &self.validation_status
            {
                errors
                    .iter()
                    .map(|(range, msg)| range)
                    .cloned()
                    .collect::<Vec<_>>()
            } else {
                vec![]
            };

        container(
            text_editor(&self.content)
                .highlight::<Highlighter>(
                    highlighter::Settings {
                        theme: iced::highlighter::Theme::Base16Mocha,
                        errors,
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
                    .width(24)
                    .height(34)
                    .center_y(),
                text(format!("{}", self.validation_status)),
                checkbox("Auto", self.auto_validate, Message::AutoValidate),
            ]
            .spacing(10)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .align_x(alignment::Horizontal::Left);

        let file_controls = container(
            row![
                control_button(new_icon, "Create a new shader", Message::New),
                control_button(open_icon, "Open a shader file", Message::Open),
                control_button(save_icon, "Save current shader", Message::Save),
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

fn control_button<'a>(
    content: impl Into<Element<'a, Message>>,
    label: &'a str,
    on_press: Message,
) -> Element<'a, Message> {
    let button = button(container(content).width(30).center_x());

    tooltip(button.on_press(on_press), label, tooltip::Position::Bottom)
        .style(theme::Container::Box)
        .into()
}

mod validation {
    use crate::editor::{icon, Message};
    use iced::Element;
    use naga::valid::Capabilities;
    use std::fmt::Formatter;
    use std::ops::Range;

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
        let shader = format!(
            "{}\n{}",
            include_str!("viewer/shaders/uniforms.wgsl"),
            shader
        );

        let parsed = naga::front::wgsl::parse_str(&shader).map_err(|parse_error| Error::Parse {
            message: parse_error.message().to_string(),
            errors: parse_error
                .labels()
                .filter_map(|(span, err)| span.to_range().map(|r| (r, err.to_string())))
                .collect::<Vec<_>>(),
        })?;

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
        #[error("Shader parsing error")]
        Parse {
            message: String,
            errors: Vec<(Range<usize>, String)>,
        },
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

//TODO colored icons once I have an actual theme
pub fn icon<'a, Message: 'static>(char: char) -> Element<'a, Message> {
    const FONT: Font = Font::with_name("halo-icons");

    text(char).font(FONT).into()
}
