struct Uniforms {
    transform: mat4x4<f32>,
    position: vec2<f32>,
    scale: vec2<f32>,
    mouse: vec2<f32>,
    time: f32,
}

@group(0) @binding(0) var<uniform> uniforms: Uniforms;