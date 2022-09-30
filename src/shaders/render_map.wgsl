struct TrailOutput {
    [[builtin(position)]] pos: vec4<f32>;
    [[location(0)]] value: f32;
    [[location(1)]] species: u32;
};

struct Species {
    sensor_size: f32;
    sensor_angle: f32;
    sensor_distance: f32;
    turn_speed: f32;
    move_speed: f32;
    random_forward_strength: f32;
    random_left_strength: f32;
    random_right_strength: f32;
    weight: f32;
    color: vec3<f32>;
    color2: vec3<f32>;
};

struct SpeciesMap {
    species: array<Species>;
};


[[group(0), binding(0)]] var<storage, read> species_map: SpeciesMap;

[[stage(vertex)]]
fn main_vs(
    [[location(0)]] pos: vec2<f32>,
    [[location(1)]] value: f32,
    [[location(2)]] species: u32,
) -> TrailOutput {
    var trail_output: TrailOutput;

    trail_output.pos = vec4<f32>(pos.x, pos.y, 0.0, 1.0);
    trail_output.value = value;
    trail_output.species = species;

    return trail_output;
}

[[stage(fragment)]]
fn main_fs(trail_output: TrailOutput) -> [[location(0)]] vec4<f32> {
    var v = trail_output.value;

    let species = species_map.species[trail_output.species];
    let color = species.color;
    return vec4<f32>(color[0] * v, color[1] * v, color[2] * v, 1.0);
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
