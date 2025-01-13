#import bevy_sprite::mesh2d_functions::mesh2d_position_world_to_clip;

const Z_R_VALUE_INDEX: u32 = 0;
const Z_I_VALUE_INDEX: u32 = 1;
const C_R_VALUE_INDEX: u32 = 2;
const C_I_VALUE_INDEX: u32 = 3;
const P_R_VALUE_INDEX: u32 = 4;
const P_I_VALUE_INDEX: u32 = 5;
const PIXEL_X_INDEX: u32 = 6;
const PIXEL_Y_INDEX: u32 = 7;

const PARAM_ARRAY_SIZE: u32 = 16;

const LARGE_FLOAT: f32 = 1e38;
const MAX_INT_POW: u32 = 15;

// Exponent modes
const EXP_2: u32 = 0;
const EXP_0: u32 = 1;
const EXP_POS_INT: u32 = 2;
const EXP_NEG_INT: u32 = 3;
const EXP_REAL: u32 = 4;
const EXP_COMPLEX: u32 = 5;

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
    p: ComplexParameter,
    escape_radius: f32,
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
    p: vec2f,
    exp_mode: u32,
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
    let x = in.world_pos.x * material.scale + material.offset.x;
    let y = in.world_pos.y * material.scale + material.offset.y;
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
    param_array[P_R_VALUE_INDEX] = material.p.real_value;
    param_array[P_I_VALUE_INDEX] = material.p.imag_value;

    var out: FractalParams;
    out.z.x = param_array[material.initial_z.real_index];
    out.z.y = param_array[material.initial_z.imag_index];
    out.c.x = param_array[material.c.real_index];
    out.c.y = param_array[material.c.imag_index];
    out.p.x = param_array[material.p.real_index];
    out.p.y = param_array[material.p.imag_index];

    out.exp_mode = get_exp_mode(material);

    return out;
}

fn get_exp_mode(m: FractalMaterial) -> u32 {
    // if the exponent is constant
    if m.p.real_index == P_R_VALUE_INDEX && m.p.imag_index == P_I_VALUE_INDEX {
        if m.p.imag_value != 0 {
            // TODO: maybe if it's only imaginary we could do some smart thing as well
            return EXP_COMPLEX;
        }
        if m.p.real_value == 0.0 {
            return EXP_0;
        }
        if m.p.real_value == 2.0 {
            return EXP_2;
        }
        if fract(m.p.real_value) == 0.0 {
            let int_pow = i32(m.p.real_value);

            if int_pow < 0 {
                return EXP_NEG_INT;
            }
            return EXP_POS_INT;
        }
        return EXP_REAL;
    }
    return EXP_COMPLEX;
}

fn fractal(params: FractalParams) -> FractalResult {
    var z = params.z;
    let c = params.c;
    let p = params.p;
    var out: FractalResult;

    let r_squared = material.escape_radius * material.escape_radius;

    var i: u32 = 0;

    // if starting z is 0, there can be a lot of issues with exponentioation 
    // blowing up
    // so we just do a dummy iteration
    if z.x == 0 && z.y == 0.0 {
        z = c;

        if z.x * z.x + z.y * z.y > r_squared {
            out.exit_iteration = i;
            out.final_z = z;
            return out;
        }
        if material.iteration_count > 0 {
            i += 1u;
        }
    }

    // hoisting all the branches out of the loop
    switch params.exp_mode {
        case EXP_2 {
            for (; i < material.iteration_count; i += 1u) {
                z = complex_square(z) + c;

                if z.x * z.x + z.y * z.y > r_squared {
                    break;
                }
            }
        }
        case EXP_0 {
            for (; i < material.iteration_count; i += 1u) {
                let z_is_zero = z.x == 0.0 && z.y == 0.0;
                z = vec2(f32(!z_is_zero), 0.0) + c;

                if z.x * z.x + z.y * z.y > r_squared {
                    break;
                }
            }
        }
        case EXP_POS_INT {
            for (; i < material.iteration_count; i += 1u) {
                z = complex_pow_pos_int(z, u32(p.x)) + c;

                if z.x * z.x + z.y * z.y > r_squared {
                    break;
                }
            }
        }
        case EXP_NEG_INT {
            for (; i < material.iteration_count; i += 1u) {
                z = complex_pow_neg_int(z, i32(p.x)) + c;

                if z.x * z.x + z.y * z.y > r_squared {
                    break;
                }
            }
        }
        case EXP_REAL {
            for (; i < material.iteration_count; i += 1u) {
                z = complex_pow_real(z, p.x) + c;

                if z.x * z.x + z.y * z.y > r_squared {
                    break;
                }
            }
        }
        case EXP_COMPLEX {
            for (; i < material.iteration_count; i += 1u) {
                z = complex_pow_complex(z, p) + c;

                if z.x * z.x + z.y * z.y > r_squared {
                    break;
                }
            }
        }
        default: {}
    }

    out.exit_iteration = i;
    out.final_z = z;
    return out;
}

fn fractal_res_to_color(res: FractalResult) -> vec3f {
    // const escape_radius = 16.0;
    const curve_exp = 1.0;
    const brightness_max_iter = 200.0;

    let x = res.final_z.x;
    let y = res.final_z.y;
    let dist = (sqrt(x * x + y * y) - material.escape_radius) / (material.escape_radius * material.escape_radius / 4.0);
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

fn complex_pow_complex(z: vec2f, p: vec2f) -> vec2f {
    if z.x == 0.0 && z.y == 0.0 {
        return z;
    }

    return complex_exp(complex_mult(p, complex_ln(z)));
}

fn complex_exp(z: vec2f) -> vec2f {
    var a = z;

    return complex_from_polar(vec2(exp(z.x), z.y));
}

fn complex_ln(z: vec2f) -> vec2f {
    var polar = complex_to_polar(z);

    return vec2(log(polar.x), polar.y);
}

fn complex_pow_real(z: vec2f, p: f32) -> vec2f {
    let polar = complex_to_polar(z);

    return complex_from_polar(vec2(pow(polar.x, p), polar.y * p));
}

fn complex_pow_pos_int(z: vec2f, p: u32) -> vec2f {
    var x = z;
    var n = p;

    var y = vec2f(1.0, 0.0);
    while n > 1 {
        if n % 2 == 1 {
            y = complex_mult(x, y);
            n -= 1u;
        }
        x = complex_square(x);
        n /= 2u;
    }

    return complex_mult(x, y);
}

fn complex_pow_neg_int(z: vec2f, p: i32) -> vec2f {
    return complex_pow_pos_int(complex_inv(z), u32(-p));
}

fn complex_square(z: vec2f) -> vec2f {
    let new_zr = z.x * z.x - z.y * z.y;
    let new_zi = 2 * z.x * z.y;

    return vec2(new_zr, new_zi);
}

fn complex_to_polar(z: vec2f) -> vec2f {
    // if z.x == 0.0 && z.y == 0.0 {
    //     return z;
    // }

    return vec2(length(z), atan2(z.y, z.x));
}

fn complex_from_polar(polar: vec2f) -> vec2f {
    return vec2(polar.x * cos(polar.y), polar.x * sin(polar.y));
}

fn complex_mult(a: vec2f, b: vec2f) -> vec2f {
    // (ar * iai) * (br * ibi) =
    // (ar * br) - ai * bi + (ar * ibi) + (iai * br)
    return vec2(a.x * b.x - a.y * b.y, a.x * b.y + a.y * b.x);
}

fn complex_inv(z: vec2f) -> vec2f {
    var norm_sqr = z.x * z.x + z.y * z.y;
    // if norm_sqr == 0.0 {
    //     return z;
    // }

    return vec2(z.x / norm_sqr, -z.y / norm_sqr);
}