#import bevy_sprite::mesh2d_functions::mesh2d_position_world_to_clip;

struct FractalMaterial {
    scale: f32,
    offset: vec2f,
}

struct FractalVertexInput {
    @builtin(instance_index) index: u32,
    @location(0) position: vec2<f32>
}

struct FragmentInput {
    @builtin(position) clip_pos: vec4f,
    @interpolate(linear) @location(0) world_pos: vec2f,
}

@group(2) @binding(0) var<uniform> material: FractalMaterial;

@vertex
fn vertex(in: FractalVertexInput) -> FragmentInput {
    var out: FragmentInput;
    out.clip_pos = mesh2d_position_world_to_clip(vec4(in.position.xy, 0.0, 1.0));
    out.world_pos = in.position.xy;

    return out;
}

@fragment
fn fragment(in: FragmentInput) -> @location(0) vec4f {
    // let x = f64(in.world_pos.x) * 0.000002 - 1.0;
    // let y = f64(in.world_pos.y) * 0.000002 + 0.3033229;
    let x = in.world_pos.x * material.scale + material.offset.x;
    let y = in.world_pos.y * material.scale + material.offset.y;

    var zr = 0.0;
    var zi = 0.0;
    let cr = x;
    let ci = y;

    let first_half = 2576980378u;
    let second_half = 1069128089u;

    const iters: u32 = 100;
    var i: u32;
    for (i = 0u; i < iters; i += 1u) {
        let new_zr = zr * zr - zi * zi + cr;
        let new_zi = 2 * zr * zi + ci;
        zr = new_zr;
        zi = new_zi;
        if new_zr * new_zr + new_zi * new_zi > 4.0 {
            break;
        }
    }

    var grad: vec3f;
    if i == iters {
        grad = vec3(0.0, 0.0, 0.0);
    } else {
        let dist = sqrt(zr * zr + zi * zi) - 2.0;
        let value = f32(i) + 1.0 - saturate(dist);
        let t = value / f32(iters);
        let curved_t = pow(t, 1.5);
        grad = mix(vec3(0.001), vec3(1.0), curved_t);
    }
    let color = grad * vec3(1.0);

    return vec4(color, 1.0);
}