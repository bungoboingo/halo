fn dot2(in: vec2<f32>) -> f32 {
    return dot(in, in);
}

//yoinked from https://iquilezles.org/articles/distfunctions2d/ <3
fn heart_sd(in: vec2<f32>) -> f32 {
    let p = vec2<f32>(abs(in.x), in.y);

    if (p.y + p.x > 1.0) {
        return sqrt(dot2(p - vec2<f32>(0.25, 0.75))) - sqrt(2.0)/4.0;
    }

    return sqrt(min(dot2(p - vec2<f32>(0.0, 1.0)), dot2(p - 0.5 * max(p.x + p.y, 0.0)))) * sign(p.x - p.y);
}

fn palette(t: f32) -> vec3<f32> {
    let a = vec3<f32>(0.5, 0.5, 0.5	);
    let b = vec3<f32>(0.5, 0.5, 0.5	);
    let c = vec3<f32>(1.0, 1.0, 1.0);
    let d = vec3<f32>(0.00, 0.33, 0.67);

    return a + b * cos(6.28318 * (c * t + d));
}

//modified from https://www.youtube.com/watch?v=f4s1h2YETNY!
@fragment
fn fs_main(@builtin(position) clip_pos: vec4<f32>) -> @location(0) vec4<f32> {
    var uv = (clip_pos.xy * 2.0 - uniforms.scale ) / uniforms.scale.y;
    uv.y = -uv.y + 0.65; //flip coords & center heart
    var uv0 = uv;
    var final_color = vec3<f32>(0.0);

    for (var i = 0; i < 4; i++) {
        var d = heart_sd(uv) * exp(heart_sd(uv0));

        var col = palette(heart_sd(uv0) + f32(i) * .4 + uniforms.time);

        d = sin(d * 4.0 - uniforms.time) / 4.0;
        d = abs(d);

        d = 0.03 / d;

        final_color += col * d;
    }

    return vec4<f32>(final_color, 1.0);
}
