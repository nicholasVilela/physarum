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

    var r = 0.0;
    var g = v;
    var b = 0.0;
    
    var min = 0.33;
    var max = 0.66;

    if (v > min && v < max) {
        r = min;
        g = v - min;
    }
    else if (v > max) {
        r = min;
        g = min;
        b = v - max;
        r = min - b;
    }

    return vec4<f32>(r, g, b, 1.0);
}
