struct VertexOutput {
    @builtin(position) position: vec4<f32>,
    @location(0) uv: vec2<f32>,
};

struct BlitUniforms {
    /// Output scale factor.  1.0 = fill the letterbox rect exactly.
    /// < 1.0 → zoom out (black border, CRT overscan safe area).
    /// > 1.0 → zoom in (edges cropped).
    display_scale: f32,
};

@group(0) @binding(0) var tex:  texture_2d<f32>;
@group(0) @binding(1) var samp: sampler;
@group(0) @binding(2) var<uniform> blit_uniforms: BlitUniforms;

@vertex
fn vs(@builtin(vertex_index) vi: u32) -> VertexOutput {
    var positions = array<vec2<f32>, 3>(
        vec2<f32>(-1.0, -1.0),
        vec2<f32>(3.0,  -1.0),
        vec2<f32>(-1.0,  3.0),
    );
    var uvs = array<vec2<f32>, 3>(
        vec2<f32>(0.0, 1.0),
        vec2<f32>(2.0, 1.0),
        vec2<f32>(0.0, -1.0),
    );

    var out: VertexOutput;
    out.position = vec4<f32>(positions[vi], 0.0, 1.0);
    out.uv = uvs[vi];
    return out;
}

@fragment
fn fs(in: VertexOutput) -> @location(0) vec4<f32> {
    // Centre-scale the UV coordinates.
    //   scale > 1: zoom in  → UVs stay inside [0,1] (centre crop)
    //   scale < 1: zoom out → UVs go outside [0,1]; those pixels → black border
    // textureSample is always called (uniform control flow); select() picks black
    // where out_of_range without branching.
    let uv_scaled = vec2<f32>(0.5) + (in.uv - vec2<f32>(0.5)) / blit_uniforms.display_scale;
    let out_of_range = uv_scaled.x < 0.0 || uv_scaled.x > 1.0
                    || uv_scaled.y < 0.0 || uv_scaled.y > 1.0;
    let color = textureSample(tex, samp, uv_scaled);
    return select(color, vec4<f32>(0.0, 0.0, 0.0, 1.0), out_of_range);
}
