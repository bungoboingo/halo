use iced::widget::Transformation;
use iced::{Point, Rectangle};
use std::time::Duration;

#[derive(Debug)]
pub struct Uniforms {
    pub time: Duration,
    pub mouse: Point,
    pub bounds: Rectangle,
}
impl Uniforms {
    pub fn to_raw(&self, scale_factor: f32, transform: Transformation) -> Raw {
        Raw {
            transform: transform.into(),
            position: [self.bounds.x * scale_factor, self.bounds.y * scale_factor],
            scale: [self.bounds.width * scale_factor, self.bounds.height * scale_factor],
            mouse: self.mouse.into(),
            time: self.time.as_secs_f32(),
            _padding: 0.0,
        }
    }
}

#[derive(Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Raw {
    pub transform: glam::Mat4,
    pub position: [f32; 2],
    pub scale: [f32; 2],
    pub mouse: [f32; 2],
    pub time: f32,
    pub _padding: f32,
}
