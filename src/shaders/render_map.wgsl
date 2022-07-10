struct TrailOutput {
    [[location(0)]] value: f32;
    [[builtin(position)]] pos: vec4<f32>;
};


[[stage(vertex)]]
fn main_vs(
    [[location(0)]] pos: vec2<f32>,
    [[location(1)]] value: f32,
) -> TrailOutput {
    var trail_output: TrailOutput;

    trail_output.pos = vec4<f32>(pos.x, pos.y, 0.0, 1.0);
    trail_output.value = value;

    return trail_output;
}

[[stage(fragment)]]
fn main_fs(trail_output: TrailOutput) -> [[location(0)]] vec4<f32> {
    return vec4<f32>(1.0, 0.0, 1.0, trail_output.value);
}
