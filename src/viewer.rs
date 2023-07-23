mod pipeline;
mod primitive;
mod uniforms;

use crate::viewer::primitive::Primitive;
use crate::viewer::uniforms::Uniforms;
use iced::advanced::Shell;
use iced::event::Status;
use iced::widget::shader::Event;
use iced::{mouse, Point, Rectangle};
use std::sync::Arc;
use std::time::Instant;
use iced::window::RedrawRequest;

pub struct Viewer {
    start: Instant,
    pub version: usize,
    pub shader: Arc<String>,
}

impl Default for Viewer {
    fn default() -> Self {
        Self {
            start: Instant::now(),
            version: 0,
            shader: Arc::new(include_str!("viewer/shaders/default_frag.wgsl").to_string()),
        }
    }
}

impl<Message> iced::widget::shader::Program<Message> for Viewer {
    type State = ();
    type Primitive = Primitive;

    fn update(
        &self,
        _state: &mut Self::State,
        _event: Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
        shell: &mut Shell<'_, Message>,
    ) -> (Status, Option<Message>) {
        shell.request_redraw(RedrawRequest::NextFrame);

        (Status::Ignored, None)
    }

    fn draw(
        &self,
        _state: &Self::State,
        cursor: mouse::Cursor,
        bounds: Rectangle,
    ) -> Self::Primitive {
        Primitive {
            uniforms: Uniforms {
                time: Instant::now() - self.start,
                mouse: match cursor {
                    mouse::Cursor::Available(pt) => pt,
                    //we go full circle..
                    mouse::Cursor::Unavailable => Point::new(-1.0, -1.0),
                },
                bounds,
            },
            shader: self.shader.clone(),
            version: self.version,
        }
    }
}
