struct Uniforms {
    transform: mat4x4<f32>,
    position: vec2<f32>,
    scale: vec2<f32>,
    mouse: vec2<f32>,
    time: f32,
}

var<private> positions: array<vec2<f32>, 6> = array<vec2<f32>, 6>(
    vec2<f32>(0.0, 0.0),
    vec2<f32>(0.0, 1.0),
    vec2<f32>(1.0, 1.0),
    vec2<f32>(0.0, 0.0),
    vec2<f32>(1.0, 0.0),
    vec2<f32>(1.0, 1.0)
);

@group(0) @binding(0) var<uniform> uniforms: Uniforms;

@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    var transform: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(uniforms.scale.x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, uniforms.scale.y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(uniforms.position, 0.0, 1.0),
    );

    return uniforms.transform * transform * vec4<f32>(positions[index], 0.0, 1.0);
}

@fragment
fn fs_main(@builtin(position) clip_pos: vec4<f32>) -> @location(0) vec4<f32> {
    return vec4<f32>(1.0, 0.0, 0.0, 1.0);
}