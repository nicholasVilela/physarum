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
    var v = trail_output.value;
    var c1 = v;
    var c2 = 0.0;
    var c3 = 0.0;

    if (v > 0.33 && v < 0.66) {
        c1 = 0.33;
        c2 = v - 0.33;
    }
    if (v > 0.66) {
        c1 = 0.33;
        c2 = 0.33;
        c3 = v - 0.66;
        c1 = 0.33 - c3;
    }

    return vec4<f32>(c1,c2,c3, 1.0);

    // return vec4<f32>(1.0, 0.0, 1.0, trail_output.value);
}
