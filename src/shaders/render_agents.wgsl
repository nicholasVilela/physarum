[[stage(vertex)]]
fn main_vs(
    [[location(0)]] pos: vec2<f32>
) -> [[builtin(position)]] vec4<f32> {
    return vec4<f32>(pos.x, pos.y, 0.0, 1.0);
}

[[stage(fragment)]]
fn main_fs() -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
