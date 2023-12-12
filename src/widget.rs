use crate::theme::Theme;

// theme type aliases
pub type Renderer = iced::Renderer<Theme>;
pub type Element<'a, Message> = iced::Element<'a, Message, Renderer>;

pub mod pane_grid {
    use crate::widget::Renderer;

    pub type PaneGrid<'a, Message> = iced::widget::PaneGrid<'a, Message, Renderer>;
    pub type TitleBar<'a, Message> = iced::widget::pane_grid::TitleBar<'a, Message, Renderer>;
    pub type Content<'a, Message> = iced::widget::pane_grid::Content<'a, Message, Renderer>;
}

pub mod text_editor {
    use crate::widget::Renderer;

    pub type TextEditor<'a, Highlighter, Message> =
        iced::widget::text_editor::TextEditor<'a, Highlighter, Message, Renderer>;
    pub type Content = iced::widget::text_editor::Content<Renderer>;
}
