use crate::viewer::uniforms;
use iced::widget::shader::wgpu;
use iced::Rectangle;
use std::borrow::Cow;

pub struct Pipeline {
    pub version: usize,
    uniforms: wgpu::Buffer,
    bind_group: wgpu::BindGroup,
    pipeline: wgpu::RenderPipeline,
}

impl Pipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        shader: &str,
        version: usize,
    ) -> Self {
        let uniforms = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("halo.pipeline.uniforms"),
            size: std::mem::size_of::<uniforms::Raw>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });

        let layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("halo.pipeline.uniform_bind_group_layout"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX_FRAGMENT,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("halo.pipeline.uniform_bind_group"),
            layout: &layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: wgpu::BindingResource::Buffer(uniforms.as_entire_buffer_binding()),
            }],
        });

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("halo.pipeline.layout"),
            bind_group_layouts: &[&layout],
            push_constant_ranges: &[],
        });

        let vertex_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("halo.pipeline.shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(concat!(
                include_str!("shaders/uniforms.wgsl"),
                "\n",
                include_str!("shaders/vertex_shader.wgsl"),
            ))),
        });

        let fragment_shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("halo.pipeline.fragment_shader"),
            source: wgpu::ShaderSource::Wgsl(Cow::Borrowed(
                format!("{}\n{}", include_str!("shaders/uniforms.wgsl"), shader).as_str(),
            )),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("halo.pipeline.pipeline"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: Default::default(),
            fragment: Some(wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            version,
            uniforms,
            bind_group,
            pipeline,
        }
    }

    pub fn prepare(&mut self, queue: &wgpu::Queue, uniforms: &uniforms::Raw) {
        queue.write_buffer(&self.uniforms, 0, bytemuck::bytes_of(uniforms));
    }

    pub fn render(
        &self,
        encoder: &mut wgpu::CommandEncoder,
        target: &wgpu::TextureView,
        bounds: Rectangle<u32>,
    ) {
        let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: Some("halo.render_pass"),
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: target,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Load,
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        pass.set_scissor_rect(bounds.x, bounds.y, bounds.width, bounds.height);

        pass.set_pipeline(&self.pipeline);
        pass.set_bind_group(0, &self.bind_group, &[]);
        pass.draw(0..6, 0..1);
    }
}
