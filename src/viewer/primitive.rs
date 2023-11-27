use crate::viewer::pipeline::Pipeline;
use crate::viewer::uniforms::Uniforms;
use iced::widget::shader::{Storage, Transformation};
use iced::{Rectangle, Size};
use std::sync::Arc;

#[derive(Debug)]
pub struct Primitive {
    pub uniforms: Uniforms,
    pub shader: Arc<String>,
    pub version: usize,
}

impl iced::widget::shader::Primitive for Primitive {
    fn prepare(
        &self,
        format: iced::widget::shader::wgpu::TextureFormat,
        device: &iced::widget::shader::wgpu::Device,
        queue: &iced::widget::shader::wgpu::Queue,
        _target_size: Size<u32>,
        scale_factor: f32,
        transform: Transformation,
        storage: &mut Storage,
    ) {
        let should_store = storage
            .get::<Pipeline>()
            .map(|pipeline| pipeline.version < self.version)
            .unwrap_or(true);

        if should_store {
            storage.store(Pipeline::new(device, format, &self.shader, self.version));
        }

        let pipeline = storage.get_mut::<Pipeline>().unwrap();

        pipeline.prepare(queue, &self.uniforms.to_raw(scale_factor, transform));
    }

    fn render(
        &self,
        storage: &Storage,
        bounds: Rectangle<u32>,
        target: &iced::widget::shader::wgpu::TextureView,
        _target_size: Size<u32>,
        encoder: &mut iced::widget::shader::wgpu::CommandEncoder,
    ) {
        let pipeline = storage.get::<Pipeline>().unwrap();

        pipeline.render(encoder, target, bounds);
    }
}
