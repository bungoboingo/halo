mod pipeline;
mod primitive;

use crate::viewer::primitive::{Primitive, Uniforms};
use iced::mouse::Cursor;
use iced::{mouse, Point, Rectangle};
use iced_graphics::custom::Program;
use std::time::Instant;

pub struct Viewer {
    start: Instant,
}

impl Viewer {
    pub fn new(start: Instant) -> Self {
        Self { start }
    }
}

impl<Message> Program<Message> for Viewer {
    type State = ();
    type Primitive = Primitive;

    fn draw(
        &self,
        _state: &Self::State,
        cursor: mouse::Cursor,
        bounds: Rectangle,
    ) -> Self::Primitive {
        Primitive {
            uniforms: Uniforms {
                position: bounds.position(),
                time: Instant::now() - self.start,
                scale: bounds.size(),
                mouse: match cursor {
                    Cursor::Available(pt) => pt,
                    Cursor::Unavailable => Point::new(-1.0, -1.0),
                },
            },
        }
    }

    //TODO this is awkward
    fn state(&self) -> &Self::State {
        &()
    }

    fn mouse_interaction(
        &self,
        _state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        if let mouse::Cursor::Available(point) = cursor {
            if bounds.contains(point) {
                return mouse::Interaction::Crosshair;
            }
        }

        mouse::Interaction::Idle
    }
}
