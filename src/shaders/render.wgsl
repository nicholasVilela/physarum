[[stage(vertex)]]
fn main_vs(
    [[location(0)]] data: vec3<f32>
) -> [[builtin(position)]] vec4<f32> {
    return vec4<f32>(data[0], data[1], 0.0, 1.0);
}

[[stage(fragment)]]
fn main_fs([[builtin(position)]] pos: vec4<f32>) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 1.0, 1.0, 1.0);
}
