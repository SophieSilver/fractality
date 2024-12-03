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

    var zr = x;
    var zi = y;
    let cr = mix(0.1, x, 0.6);
    let ci = mix(0.6, y, 0.6);

    const iters: u32 = 100;
    var i: u32;
    for (i = 0u; i < iters; i += 1u) {
        let new_zr = zr * zr - zi * zi + cr;
        let new_zi = 2 * zr * zi + ci;
        zr = new_zr;
        zi = new_zi;

        if zr * zr + zi * zi > 4.0 {
            break;
        }
    }

    var grad: vec3f;
    let dist = f32(sqrt(zr * zr + zi * zi)) - 2.0;
    let curved_dist = 1 - exp(4.0 * -dist);
    if i == iters {
        grad = vec3(0.0, 0.0, 0.0);
    } else {
        grad = mix(vec3(0.001), vec3(1.0), pow(saturate((f32(i) + 1 - saturate(dist)) / f32(iters)), 1.0));
    }
    let color = grad * vec3(1.0);

    return vec4(color, 1.0);
}