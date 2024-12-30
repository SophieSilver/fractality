#import bevy_sprite::mesh2d_functions::mesh2d_position_world_to_clip;

const Z_R_VALUE_INDEX: u32 = 0;
const Z_I_VALUE_INDEX: u32 = 1;
const C_R_VALUE_INDEX: u32 = 2;
const C_I_VALUE_INDEX: u32 = 3;
const PIXEL_X_INDEX: u32 = 4;
const PIXEL_Y_INDEX: u32 = 5;

const PARAM_ARRAY_SIZE: u32 = 16;

struct ComplexParameter {
    real_value: f32,
    real_index: u32,
    imag_value: f32,
    imag_index: u32,
}

struct Parameter {
    value: f32,
    index: u32,
}

struct FractalMaterial {
    iteration_count: u32,
    scale: f32,
    offset: vec2f,
    initial_z: ComplexParameter,
    c: ComplexParameter,
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
    param_array[Z_R_VALUE_INDEX] = material.initial_z.real_value;
    param_array[Z_I_VALUE_INDEX] = material.initial_z.imag_value;
    param_array[C_R_VALUE_INDEX] = material.c.real_value;
    param_array[C_I_VALUE_INDEX] = material.c.imag_value;
    param_array[PIXEL_X_INDEX] = x;
    param_array[PIXEL_Y_INDEX] = y;

    var out: FractalParams;
    out.z.x = param_array[material.initial_z.real_index];
    out.z.y = param_array[material.initial_z.imag_index];
    out.c.x = param_array[material.c.real_index];
    out.c.y = param_array[material.c.imag_index];

    return out;
}

fn fractal(params: FractalParams) -> FractalResult {
    let z = params.z;
    let c = params.c;
    var out: FractalResult;

    const escape_radius = 2.0;

    let r_squared = escape_radius * escape_radius;

    var zr = z.x;
    var zi = z.y;
    let cr = c.x;
    let ci = c.y;

    var i: u32;
    for (i = 0u; i < material.iteration_count; i += 1u) {
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
    const escape_radius = 2.0;
    const curve_exp = 1.0;
    const brightness_max_iter = 200.0;

    let x = res.final_z.x;
    let y = res.final_z.y;
    let dist = sqrt(x * x + y * y) - escape_radius;
    let value = f32(res.exit_iteration) + 1.0 - saturate(dist);
    let t = value / brightness_max_iter;

    var brightness = 0.0;
    if res.exit_iteration == material.iteration_count {
        brightness = 0.0;
    } else {
        // let curved_t = pow(t, curve_exp);
        let curved_t = t;
        brightness = mix(0.001, 1.0, curved_t);
    }

    var color = hsv2rgb(vec3(value * 0.01 + 0.6, 1.0, 1.0));

    return color * brightness;
}

fn hsv2rgb(hsv: vec3f) -> vec3f {
    let k = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    let p = abs(fract(hsv.rrr + k.rgb) * 6.0 - k.www);
    return hsv.b * mix(k.rrr, saturate(p - k.rrr), hsv.g);
}