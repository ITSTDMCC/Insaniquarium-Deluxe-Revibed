// Draws the game's 640x480 screen scaled to the window.
//
// Filter 1 (default): xBR level 2 (Hyllian's edge-directed upscaler, as in libretro's
// xbr-lv2): every output pixel starts as its source texel; where the texel sits on a
// diagonal or shallow edge, the side of the edge the pixel lies on is blended with the
// neighbour across it, so slopes come out as smooth lines instead of stair steps. The
// edge blend is about one screen pixel wide, so the result is anti-aliased at any scale.
// Filter 0: plain bilinear (the look before).

#import bevy_sprite::mesh2d_vertex_output::VertexOutput

struct Params {
    // x: filter (0 bilinear, 1 xBR)
    mode: vec4<f32>,
};

@group(#{MATERIAL_BIND_GROUP}) @binding(0) var<uniform> params: Params;
@group(#{MATERIAL_BIND_GROUP}) @binding(1) var screen: texture_2d<f32>;
@group(#{MATERIAL_BIND_GROUP}) @binding(2) var screen_sampler: sampler;

const Y_WEIGHT: f32 = 48.0;
const EQ_THRESHOLD: f32 = 15.0;
const LV2_COEFFICIENT: f32 = 2.0;

const Ao = vec4<f32>(1.0, -1.0, -1.0, 1.0);
const Bo = vec4<f32>(1.0, 1.0, -1.0, -1.0);
const Co = vec4<f32>(1.5, 0.5, -0.5, 0.5);
const Ax = vec4<f32>(1.0, -1.0, -1.0, 1.0);
const Bx = vec4<f32>(0.5, 2.0, -0.5, -2.0);
const Cx = vec4<f32>(1.0, 1.0, -0.5, 0.0);
const Ay = vec4<f32>(1.0, -1.0, -1.0, 1.0);
const By = vec4<f32>(2.0, 0.5, -2.0, -0.5);
const Cy = vec4<f32>(2.0, 0.0, -1.0, 0.5);
const Ci = vec4<f32>(0.25, 0.25, 0.25, 0.25);

var<private> base: vec2<i32>;
var<private> last: vec2<i32>;

fn texel(dx: i32, dy: i32) -> vec3<f32> {
    return textureLoad(screen, clamp(base + vec2<i32>(dx, dy), vec2<i32>(0), last), 0).rgb;
}

// Weighted luma, measured on gamma-encoded values like the original shader.
fn luma(c: vec3<f32>) -> f32 {
    return dot(pow(max(c, vec3<f32>(0.0)), vec3<f32>(1.0 / 2.2)), vec3<f32>(0.2126, 0.7152, 0.0722)) * Y_WEIGHT;
}

fn df(a: vec4<f32>, b: vec4<f32>) -> vec4<f32> {
    return abs(a - b);
}

fn c_df(a: vec3<f32>, b: vec3<f32>) -> f32 {
    let d = abs(a - b);
    return d.r + d.g + d.b;
}

fn eq(a: vec4<f32>, b: vec4<f32>) -> vec4<bool> {
    return df(a, b) < vec4<f32>(EQ_THRESHOLD);
}

fn ne(a: vec4<f32>, b: vec4<f32>) -> vec4<bool> {
    return df(a, b) > vec4<f32>(0.0);
}

fn weighted_distance(a: vec4<f32>, b: vec4<f32>, c: vec4<f32>, d: vec4<f32>, e: vec4<f32>, f: vec4<f32>, g: vec4<f32>, h: vec4<f32>) -> vec4<f32> {
    return df(a, b) + df(a, c) + df(d, e) + df(d, f) + 4.0 * df(g, h);
}

fn and4(a: vec4<bool>, b: vec4<bool>) -> vec4<bool> {
    return select(vec4<bool>(false), b, a);
}

fn or4(a: vec4<bool>, b: vec4<bool>) -> vec4<bool> {
    return select(b, vec4<bool>(true), a);
}

fn not4(a: vec4<bool>) -> vec4<bool> {
    return select(vec4<bool>(true), vec4<bool>(false), a);
}

fn mask(b: vec4<bool>, v: vec4<f32>) -> vec4<f32> {
    return select(vec4<f32>(0.0), v, b);
}

fn xbr(uv: vec2<f32>) -> vec3<f32> {
    let size = vec2<f32>(textureDimensions(screen));
    let pos = uv * size;
    base = vec2<i32>(floor(pos));
    last = vec2<i32>(size) - vec2<i32>(1);
    let fp = fract(pos);

    let A1 = texel(-1, -2); let B1 = texel(0, -2); let C1 = texel(1, -2);
    let A = texel(-1, -1); let B = texel(0, -1); let C = texel(1, -1);
    let D = texel(-1, 0); let E = texel(0, 0); let F = texel(1, 0);
    let G = texel(-1, 1); let H = texel(0, 1); let I = texel(1, 1);
    let G5 = texel(-1, 2); let H5 = texel(0, 2); let I5 = texel(1, 2);
    let A0 = texel(-2, -1); let G0 = texel(-2, 1);
    let C4 = texel(2, -1); let F4 = texel(2, 0); let I4 = texel(2, 1);
    let D0 = texel(-2, 0);

    let b = vec4<f32>(luma(B), luma(D), luma(H), luma(F));
    let c = vec4<f32>(luma(C), luma(A), luma(G), luma(I));
    let e = vec4<f32>(luma(E));
    let d = b.yzwx;
    let f = b.wxyz;
    let g = c.zwxy;
    let h = b.zwxy;
    let i = c.wxyz;
    let i4 = vec4<f32>(luma(I4), luma(C1), luma(A0), luma(G5));
    let i5 = vec4<f32>(luma(I5), luma(C4), luma(A1), luma(G0));
    let h5 = vec4<f32>(luma(H5), luma(F4), luma(B1), luma(D0));
    let f4 = h5.yzwx;

    // The lines past which a pixel is on the far side of each corner's edge.
    let fx = Ao * fp.y + Bo * fp.x;
    let fx_left = Ax * fp.y + Bx * fp.x;
    let fx_up = Ay * fp.y + By * fp.x;

    let lv0 = and4(ne(e, f), ne(e, h));
    let lv1 = and4(lv0, or4(or4(and4(not4(eq(f, b)), not4(eq(h, d))), and4(and4(eq(e, i), not4(eq(f, i4))), not4(eq(h, i5)))), or4(eq(e, g), eq(e, c))));
    let lv2_left = and4(ne(e, g), ne(d, g));
    let lv2_up = and4(ne(e, c), ne(b, c));

    // Edge blend about one screen pixel wide.
    let px_size = max(fwidth(pos.x), fwidth(pos.y));
    let delta = vec4<f32>(clamp(px_size, 0.05, 0.5));
    var fx45i = clamp((fx + delta - Co - Ci) / (2.0 * delta), vec4<f32>(0.0), vec4<f32>(1.0));
    var fx45 = clamp((fx + delta - Co) / (2.0 * delta), vec4<f32>(0.0), vec4<f32>(1.0));
    var fx30 = clamp((fx_left + delta - Cx) / (2.0 * delta), vec4<f32>(0.0), vec4<f32>(1.0));
    var fx60 = clamp((fx_up + delta - Cy) / (2.0 * delta), vec4<f32>(0.0), vec4<f32>(1.0));

    let wd1 = weighted_distance(e, c, g, i, h5, f4, h, f);
    let wd2 = weighted_distance(h, d, i5, f, i4, b, e, i);

    let edri = and4(wd1 <= wd2, lv0);
    let edr = and4(wd1 < wd2, lv1);
    let edr_left = and4(and4(LV2_COEFFICIENT * df(f, g) <= df(h, c), lv2_left), edr);
    let edr_up = and4(and4(df(f, g) >= LV2_COEFFICIENT * df(h, c), lv2_up), edr);

    fx45 = mask(edr, fx45);
    fx30 = mask(edr_left, fx30);
    fx60 = mask(edr_up, fx60);
    fx45i = mask(edri, fx45i);

    let px = df(e, f) <= df(e, h);
    let m = max(max(fx30, fx60), max(fx45, fx45i));

    var res1 = mix(E, select(H, F, px.x), m.x);
    res1 = mix(res1, select(B, D, px.z), m.z);
    var res2 = mix(E, select(F, B, px.y), m.y);
    res2 = mix(res2, select(D, H, px.w), m.w);
    return select(res1, res2, c_df(E, res2) >= c_df(E, res1));
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    if params.mode.x < 0.5 {
        return vec4<f32>(textureSample(screen, screen_sampler, in.uv).rgb, 1.0);
    }
    return vec4<f32>(xbr(in.uv), 1.0);
}
