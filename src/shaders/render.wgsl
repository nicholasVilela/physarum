struct VertexOutput {
    [[builtin(position)]] position: vec4<f32>;
};


[[stage(vertex)]]
fn vs_main(
    [[location(0)]] position: vec2<f32>,
) -> VertexOutput {
    var out: VertexOutput;
    out.position = vec4(position, 1, 1);
    return out;
}

[[stage(fragment)]]
fn main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(in.position.z, in.position.z, in.position.z, 1.0);
}
