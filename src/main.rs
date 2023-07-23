mod validation;
mod viewer;

use crate::viewer::Viewer;
use iced::advanced::layout::{Limits, Node};
use iced::advanced::widget::Tree;
use iced::advanced::{overlay, renderer, widget, Clipboard, Layout, Overlay, Shell, Widget};
use iced::alignment::Horizontal;
use iced::font::{Family, Stretch, Style, Weight};
use iced::theme::{Button, Container};
use iced::widget::{button, column, container, row, text};
use iced::{
    advanced, event, executor, font, mouse, Alignment, Application, Color, Command, Element, Event,
    Length, Point, Rectangle, Size, Theme,
};
use std::path::PathBuf;
use iced::widget::shader::Shader;

const TITLE: &str = "ShaderBuddy";

const JETBRAINS_MONO: iced::Font = iced::Font {
    family: Family::Name("JetBrains Mono"),
    weight: Weight::Normal,
    stretch: Stretch::Normal,
    style: Style::Normal,
    monospaced: true,
};
const PIXEL_FONT: iced::Font = iced::Font {
    family: Family::Name("Pixelify Sans"),
    weight: Weight::Bold,
    stretch: Stretch::Normal,
    style: Style::Normal,
    monospaced: false,
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
        let bg = Shader::new(&self.viewer)
            .width(Length::Fill)
            .height(Length::Fill)
            .into();

        let controls = container(
            row![
                intro_button("Open", Message::OpenShaderFile),
                intro_button("Go!", Message::ToggleEditor),
            ]
            .spacing(80)
            .align_items(Alignment::Center),
        )
        .width(Length::Fill)
        .center_x();

        let fg: Element<'_, Message> = container(
            column![
                container(text(TITLE).font(PIXEL_FONT).size(64))
                    .padding([20, 40])
                    .style(Container::Custom(Box::new(IntroStyle))),
                controls,
            ]
            .width(500)
            .spacing(20),
        )
        .center_x()
        .center_y()
        .into();

        let content: Element<'_, Message> = if self.show_intro {
            Intro {
                bg,
                fg,
                on_close: Message::CloseIntro,
            }
            .into()
        } else {
            bg
        };

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }
}

struct Intro<'a, Message, Renderer> {
    bg: Element<'a, Message, Renderer>,
    fg: Element<'a, Message, Renderer>,
    on_close: Message,
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for Intro<'a, Message, Renderer>
where
    Renderer: advanced::Renderer,
    Message: Clone,
{
    fn width(&self) -> Length {
        self.bg.as_widget().width()
    }

    fn height(&self) -> Length {
        self.bg.as_widget().height()
    }

    fn layout(&self, tree: &mut Tree, renderer: &Renderer, limits: &Limits) -> Node {
        self.bg
            .as_widget()
            .layout(&mut tree.children[0], renderer, limits)
    }

    fn draw(
        &self,
        state: &Tree,
        renderer: &mut Renderer,
        theme: &<Renderer as advanced::Renderer>::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.bg.as_widget().draw(
            &state.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.bg), Tree::new(&self.fg)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.bg, &self.fg]);
    }

    fn on_event(
        &mut self,
        state: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        self.bg.as_widget_mut().on_event(
            &mut state.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        state: &'b mut Tree,
        layout: Layout<'_>,
        _renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        Some(overlay::Element::new(
            layout.position(),
            Box::new(IntroDialogue {
                content: &mut self.fg,
                tree: &mut state.children[1],
                size: layout.bounds().size(),
                on_close: self.on_close.clone(),
            }),
        ))
    }
}

struct IntroDialogue<'a, 'b, Message, Renderer> {
    content: &'b mut Element<'a, Message, Renderer>,
    tree: &'b mut Tree,
    size: Size,
    on_close: Message,
}

impl<'a, 'b, Message, Renderer> Overlay<Message, Renderer>
    for IntroDialogue<'a, 'b, Message, Renderer>
where
    Renderer: advanced::Renderer,
    Message: Clone,
{
    fn layout(&mut self, renderer: &Renderer, _bounds: Size, position: Point) -> Node {
        let limits = Limits::new(Size::ZERO, self.size)
            .width(Length::Fill)
            .height(Length::Fill);

        let mut child = self
            .content
            .as_widget()
            .layout(self.tree, renderer, &limits);

        child.align(Alignment::Center, Alignment::Center, limits.max());

        let mut node = Node::with_children(self.size, vec![child]);
        node.move_to(position);

        node
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        renderer.fill_quad(
            renderer::Quad {
                bounds: layout.bounds(),
                border_radius: Default::default(),
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Color::TRANSPARENT,
        );

        self.content.as_widget().draw(
            self.tree,
            renderer,
            theme,
            style,
            layout.children().next().unwrap(),
            cursor,
            &layout.bounds(),
        );
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation<Message>,
    ) {
        self.content.as_widget().operate(
            self.tree,
            layout.children().next().unwrap(),
            renderer,
            operation,
        );
    }

    fn on_event(
        &mut self,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) -> event::Status {
        let content_bounds = layout.children().next().unwrap().bounds();

        if let Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)) = &event {
            if !cursor.is_over(content_bounds) {
                shell.publish(self.on_close.clone());
                return event::Status::Captured;
            }
        }

        self.content.as_widget_mut().on_event(
            self.tree,
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            &layout.bounds(),
        )
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            self.tree,
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'c>(
        &'c mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'c, Message, Renderer>> {
        self.content
            .as_widget_mut()
            .overlay(self.tree, layout.children().next().unwrap(), renderer)
    }
}

impl<'a, Message, Renderer> From<Intro<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Renderer: 'a + advanced::Renderer,
    Message: 'a + Clone,
{
    fn from(intro: Intro<'a, Message, Renderer>) -> Self {
        Element::new(intro)
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

fn intro_button(label: &'static str, on_press: Message) -> Element<'_, Message> {
    button(
        text(label)
            .font(JETBRAINS_MONO)
            .size(16)
            .width(Length::Fill)
            .horizontal_alignment(Horizontal::Center),
    )
    .padding(5)
    .width(80)
    .on_press(on_press)
    .style(Button::custom(IntroStyle))
    .into()
}
