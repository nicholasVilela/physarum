struct Agent {
    position: vec2<f32>;
    angle: f32;
};

[[stage(vertex)]]
fn main_vs(
    [[location(0)]] position: vec2<f32>,
    [[location(1)]] angle: f32,
) -> [[builtin(position)]] vec4<f32> {
    return vec4<f32>(position, 0.0, 1.0);
}

[[stage(fragment)]]
fn main_fs([[builtin(position)]] pos: vec4<f32>) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
