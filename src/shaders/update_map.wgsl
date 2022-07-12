struct Constants {
    window_height: f32;
    window_width: f32;
    evaporation_rate: f32;
    diffusion_rate: f32;
    diffusion_strength: f32;
};

struct Param {
    delta_time: f32;
    frame: u32;
};

struct Trail {
    position: vec2<f32>;
    value: f32;
};

struct Map {
    trail: array<Trail>;
};


[[group(0), binding(0)]] var<uniform> constants: Constants;
[[group(0), binding(1)]] var<uniform> param: Param;
[[group(0), binding(2)]] var<storage, read> map_src: Map;
[[group(0), binding(3)]] var<storage, read_write> map_dst: Map;

fn who_cell(pos: vec2<i32>) -> i32 {
    let size = i32(constants.window_width);

    var p = pos;

    if (p.x > size) {
        p.x = size;
    }
    else if (p.x < 0) {
        p.x = 0;
    }

    if (p.y > size) {
        p.y = size;
    }
    else if (p.y < 0) {
        p.y = 0;
    }

    return  (p.y * size) + p.x;
}

[[stage(compute), workgroup_size(32)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    let size = constants.window_width;
    let index = global_id.x;

    if (index >= u32(size * size)) {
        return;
    }

    let evaporation_rate = constants.evaporation_rate;
    let diffusion_rate = constants.diffusion_rate;
    let diffusion_strength = constants.diffusion_strength;

    let distance = 1;

    let pos = vec2<i32>(i32(index) % i32(size), i32(index) / i32(size));

    let l_pos = vec2<i32>(pos.x + distance, pos.y);
    let r_pos = vec2<i32>(pos.x - distance, pos.y);
    let t_pos = vec2<i32>(pos.x, pos.y - distance);
    let b_pos = vec2<i32>(pos.x, pos.y + distance);

    let l_index = who_cell(l_pos);
    let r_index = who_cell(r_pos);
    let t_index = who_cell(t_pos);
    let b_index = who_cell(b_pos);

    var l_value = map_src.trail[l_index].value * diffusion_rate;
    var r_value = map_src.trail[r_index].value * diffusion_rate;
    var t_value = map_src.trail[t_index].value * diffusion_rate;
    var b_value = map_src.trail[b_index].value * diffusion_rate;

    // map_dst.trail[index].value = map_src.trail[index].value * evaporation_rate;

    map_dst.trail[l_index].value = map_src.trail[l_index].value - l_value;
    map_dst.trail[r_index].value = map_src.trail[r_index].value - r_value;
    map_dst.trail[t_index].value = map_src.trail[t_index].value - t_value;
    map_dst.trail[b_index].value = map_src.trail[b_index].value - b_value;

    map_dst.trail[index].value = map_src.trail[index].value + (l_value + r_value + t_value + b_value) * diffusion_strength;

    // if (map_dst.trail[index].value > 1.0) {
    //     map_dst.trail[index].value = 1.0;
    // }
    // else if (map_dst.trail[index].value < 0.00001) {
    //     map_dst.trail[index].value = 0.0;
    // }
}
