mod file;
mod highlighter;
mod validation;

use crate::editor::highlighter::Highlighter;
use crate::preferences::Preferences;
use crate::{preferences, FragmentShader, JETBRAINS_MONO};
use iced::widget::{button, checkbox, container, row, text, text_editor, tooltip};
use iced::{alignment, keyboard, theme, Alignment, Command, Element, Font, Length};
use std::path::PathBuf;
use std::sync::Arc;

#[derive(Clone, Debug)]
pub enum Message {
    Init(Result<(Preferences, Arc<FragmentShader>), preferences::Error>),
    Action(text_editor::Action),
    Validate,
    AutoValidate(bool),
    New,
    Open,
    Opened(Result<(PathBuf, Arc<FragmentShader>), file::Error>),
    Save,
    Saved(Result<PathBuf, file::Error>),
    Undo,
    Redo,
    Search,
    Indent,
    PreferencesSaved(Result<(), preferences::Error>),
}

pub enum Event {
    None,
    UpdatePipeline(Arc<FragmentShader>),
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
            auto_validate: true,
            is_loading: true,
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
            Message::Init(result) => {
                let event = match result {
                    Ok((prefs, shader)) => {
                        self.auto_validate = prefs.auto_validate;
                        self.shader_path = prefs.last_shader_path;
                        self.content = text_editor::Content::with_text(&shader);
                        Event::UpdatePipeline(shader)
                    }
                    Err(e) => {
                        println!("Error loading prefs: {e:?}");
                        Event::None
                    }
                };

                self.is_loading = false;
                return (event, Command::none());
            }
            Message::Action(action) => {
                //TODO fix not being able to use hotkeys while text editor is focused
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
                    Event::UpdatePipeline(Arc::new(empty_shader.to_string())),
                    Command::none(),
                );
            }
            Message::Open => {
                let cmd = if self.is_loading {
                    Command::none()
                } else {
                    self.is_loading = true;
                    Command::perform(file::open(), Message::Opened)
                };

                return (Event::None, cmd);
            }
            Message::Opened(result) => {
                let event = if let Ok((path, shader)) = result {
                    self.shader_path = Some(path);
                    self.content = text_editor::Content::with_text(&shader);
                    Event::UpdatePipeline(shader)
                } else {
                    Event::None
                };

                //TODO loading error msg
                self.is_loading = false;

                return (event, self.save_prefs());
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
            Message::Saved(result) => {
                if let Ok(path) = result {
                    self.shader_path = Some(path);
                }
                //TODO handle error
                return (Event::None, self.save_prefs());
            }
            Message::Validate => {
                self.validation_status = validation::Status::Validating;

                let shader = self.content.text();

                match validation::validate(&shader) {
                    Ok(_) => {
                        self.validation_status = validation::Status::Validated;
                        return (Event::UpdatePipeline(Arc::new(shader)), Command::none());
                    }
                    Err(error) => {
                        println!("Failed to validate: {error:?}");
                        self.validation_status = validation::Status::Invalid(error);
                    }
                }
            }
            Message::AutoValidate(checked) => {
                self.auto_validate = checked;
                return (Event::None, self.save_prefs());
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
            Message::PreferencesSaved(_) => {
                println!("Prefs saved");
            }
        }

        (Event::None, Command::none())
    }

    fn save_prefs(&self) -> Command<Message> {
        let prefs = Preferences {
            last_shader_path: self.shader_path.clone(),
            auto_validate: self.auto_validate,
        };

        Command::perform(preferences::save(prefs), Message::PreferencesSaved)
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
                .font(JETBRAINS_MONO)
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
                    .height(24)
                    .center_y(),
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

//TODO colored icons once I have an actual theme
pub fn icon<'a, Message: 'static>(char: char) -> Element<'a, Message> {
    const FONT: Font = Font::with_name("halo-icons");

    text(char).font(FONT).into()
}
