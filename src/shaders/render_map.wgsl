struct TrailOutput {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] value: f32;
    [[location(10)]] species: u32;
};

struct Species {
    sensor_size: f32;
    sensor_angle: f32;
    sensor_distance: f32;
    turn_speed: f32;
    move_speed: f32;
    forward_bias: f32;
    left_bias: f32;
    right_bias: f32;
    weight: f32;
    color_r: f32;
    color_g: f32;
    color_b: f32;
};

struct SpeciesMap {
    species: array<Species>;
};


[[group(0), binding(0)]] var<storage, read> species_map: SpeciesMap;

[[stage(vertex)]]
fn main_vs(
    [[location(0)]] pos_x: f32,
    [[location(1)]] pos_y: f32,
    [[location(2)]] value: f32,
    [[location(3)]] species: u32,
) -> TrailOutput {
    var trail_output: TrailOutput;

    trail_output.pos = vec4<f32>(pos_x, pos_y, 0.0, 1.0);
    trail_output.value = value;
    trail_output.species = species;

    return trail_output;
}

[[stage(fragment)]]
fn main_fs(trail_output: TrailOutput) -> [[location(0)]] vec4<f32> {
    var v = trail_output.value;
    let species = species_map.species[trail_output.species];

    return vec4<f32>(species.color_r * v, species.color_g * v, species.color_b * v, 1.0);

    // return vec4<f32>(0.0 * v, 0.0 * v, 1.0 * v, 1.0);

    // var r = v;
    // var g = 0.0;
    // var b = 0.0;
    
    // var min = 0.33;
    // var max = 0.66;

    // if (v > min && v < max) {
    //     r = min;
    //     g = v - min;
    // }
    // else if (v > max) {
    //     r = min;
    //     g = min;
    //     b = v - max;
    //     r = min - b;
    // }

    // return vec4<f32>(g, b, r, 1.0);
    // return vec4<f32>(b, r, g, 1.0);
    // return vec4<f32>(b, g, r, 1.0);
}
