use std::time::Duration;
use iced::{Point, Rectangle, Size};
use iced_graphics::custom::Storage;
use iced_graphics::Transformation;
use crate::viewer::pipeline::Pipeline;

#[derive(Debug)]
pub struct Primitive {
    pub uniforms: Uniforms,
}

#[derive(Debug)]
pub struct Uniforms {
    pub time: Duration,
    pub position: Point,
    pub scale: Size,
    pub mouse: Point,
}

impl iced_graphics::custom::Primitive for Primitive {
    fn prepare(
        &self,
        format: wgpu::TextureFormat,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _target_size: Size<u32>,
        scale_factor: f32,
        transform: Transformation,
        storage: &mut Storage,
    ) {
        if let Some(pipeline) = storage.get_mut_or_init::<Pipeline>(|| {
            Pipeline::new(device, format)
        }) {
            pipeline.prepare(queue, &self.uniforms, scale_factor, transform);
        }
    }

    fn render(
        &self,
        storage: &Storage,
        bounds: Rectangle<u32>,
        target: &wgpu::TextureView,
        _target_size: Size<u32>,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        if let Some(pipeline) = storage.get::<Pipeline>() {
            pipeline.render(encoder, target, bounds);
        }
    }
}