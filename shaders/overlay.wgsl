// Overlay shader for the host-level HUD (border frame, footer bar, and text).
//
// Two rendering modes selected by the `mode` vertex attribute:
//   0.0 — solid colour quad (border strips, footer background)
//   1.0 — font-atlas glyph quad (slide title and wall-clock text)

struct OverlayVertex {
    @location(0) position: vec2<f32>,
    @location(1) uv:       vec2<f32>,
    @location(2) color:    vec4<f32>,
    @location(3) mode:     f32,
};

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) uv:    vec2<f32>,
    @location(1) color: vec4<f32>,
    @location(2) mode:  f32,
};

@group(0) @binding(0) var font_tex: texture_2d<f32>;
@group(0) @binding(1) var font_smp: sampler;

@vertex
fn vs_main(v: OverlayVertex) -> VertexOutput {
    var out: VertexOutput;
    out.clip_position = vec4<f32>(v.position, 0.0, 1.0);
    out.uv    = v.uv;
    out.color = v.color;
    out.mode  = v.mode;
    return out;
}

@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    if in.mode < 0.5 {
        // Solid geometry — return the vertex colour directly.
        return in.color;
    }
    // Font glyph — sample the red channel of the atlas as alpha.
    let a = textureSample(font_tex, font_smp, in.uv).r;
    return vec4<f32>(in.color.rgb, in.color.a * a);
}
