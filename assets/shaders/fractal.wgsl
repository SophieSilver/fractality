#import bevy_sprite::mesh2d_functions::mesh2d_position_world_to_clip;

const Z_R_VALUE_INDEX: u32 = 0;
const Z_I_VALUE_INDEX: u32 = 1;
const PIXEL_X_INDEX: u32 = 2;
const PIXEL_Y_INDEX: u32 = 3;

const PARAM_ARRAY_SIZE: u32 = 16;

struct FractalMaterial {
    scale: f32,
    offset: vec2f,
    initial_z_values: vec2f,
    initial_z_indices: vec2u,
}

struct FractalVertexInput {
    @builtin(instance_index) index: u32,
    @location(0) position: vec2f
}

struct FragmentInput {
    @builtin(position) clip_pos: vec4f,
    @interpolate(linear) @location(0) world_pos: vec2f,
}

struct FractalResult {
    exit_iteration: u32,
    final_z: vec2f,
}

struct FractalParams {
    z: vec2f,
    c: vec2f,
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
    const iters = 250u;
    let params = get_fractal_params(x, y);
    let res = fractal(params);


    return vec4(fractal_res_to_color(res), 1.0);
}

fn get_fractal_params(x: f32, y: f32) -> FractalParams {
    var param_array: array<f32, PARAM_ARRAY_SIZE>;
    param_array[Z_R_VALUE_INDEX] = material.initial_z_values.x;
    param_array[Z_I_VALUE_INDEX] = material.initial_z_values.y;
    param_array[PIXEL_X_INDEX] = x;
    param_array[PIXEL_Y_INDEX] = y;

    var out: FractalParams;
    out.z.x = param_array[material.initial_z_indices.x];
    out.z.y = param_array[material.initial_z_indices.y];
    out.c = vec2(x, y);

    return out;
}

fn fractal(params: FractalParams) -> FractalResult {
    let z = params.z;
    let c = params.c;
    var out: FractalResult;

    const max_iters = 250u;
    const escape_radius = 2.0;

    let r_squared = escape_radius * escape_radius;

    var zr = z.x;
    var zi = z.y;
    let cr = c.x;
    let ci = c.y;

    var i: u32;
    for (i = 0u; i < max_iters; i += 1u) {
        let new_zr = zr * zr - zi * zi + cr;
        let new_zi = 2 * zr * zi + ci;
        zr = new_zr;
        zi = new_zi;

        if new_zr * new_zr + new_zi * new_zi > 4.0 {
            break;
        }
    }

    out.exit_iteration = i;
    out.final_z = vec2(zr, zi);
    return out;
}

fn fractal_res_to_color(res: FractalResult) -> vec3f {
    const max_iters = 250u;
    const escape_radius = 2.0;
    const curve_exp = 1.0;

    if res.exit_iteration == max_iters {
        return vec3(0.0, 0.0, 0.0);
    } else {
        let x = res.final_z.x;
        let y = res.final_z.y;
        let dist = sqrt(x * x + y * y) - escape_radius;
        let value = f32(res.exit_iteration) + 1.0 - saturate(dist);
        let t = value / f32(max_iters);
        let curved_t = pow(t, curve_exp);
        return mix(vec3(0.001), vec3(1.0), curved_t);
    }
}