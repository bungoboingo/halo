@fragment
fn fs_main(@builtin(position) clip_pos: vec4<f32>) -> @location(0) vec4<f32> {
    var uv = (clip_pos.xy * 2.0 - uniforms.scale ) / uniforms.scale.y;
    uv.y = -uv.y;
    var final_color = vec3<f32>(0.0);


    return vec4<f32>(final_color, 1.0);
}