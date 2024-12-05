struct VertexInput {
    @location(0) position: vec2f,
}

struct FragmentInput {
    @builtin(position) clip_pos: vec4f,
    @location(0) uv: vec2f
}

@group(2) @binding(0) var texture: texture_2d<f32>;
@group(2) @binding(1) var texture_sampler: sampler;

@vertex
fn vertex(in: VertexInput) -> FragmentInput {
    var out: FragmentInput;
    out.clip_pos = vec4(in.position, 0.0, 1.0);
    out.uv = vec2(in.position.x + 1.0, -in.position.y + 1.0) * 0.5;
    return out;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4f {
    return textureSampleLevel(texture, texture_sampler, in.uv, 0.0);
    // return vec4(in.uv, 0.0, 1.0);
}