@vertex
fn vs_main(@builtin(vertex_index) index: u32) -> @builtin(position) vec4<f32> {
    let pos = vec2<f32>(
        (vec2(1u, 2u) + index) % 6u < vec2(3u, 3u)
    );

    var transform: mat4x4<f32> = mat4x4<f32>(
        vec4<f32>(uniforms.scale.x, 0.0, 0.0, 0.0),
        vec4<f32>(0.0, uniforms.scale.y, 0.0, 0.0),
        vec4<f32>(0.0, 0.0, 1.0, 0.0),
        vec4<f32>(uniforms.position, 0.0, 1.0),
    );

    return uniforms.transform * transform * vec4<f32>(pos, 0.0, 1.0);
}
