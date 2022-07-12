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


[[group(0), binding(0)]] var<storage, read> map_src: Map;
[[group(0), binding(1)]] var<storage, read_write> map_dst: Map;
[[group(0), binding(2)]] var<uniform> constants: Constants;
[[group(0), binding(3)]] var<uniform> param: Param;

fn who_cell(pos: vec2<i32>) -> i32 {
    let size = i32(constants.window_width);

    var x = min(size, max(0, pos.x));
    var y = min(size, max(0, pos.y));

    return  (y * size) + x;
}

fn get_cell_index(p: vec2<f32>) -> i32 {
    // let pos = vec2<f32>(min(1.0, max(-1.0, p.x)), min(1.0, max(-1.0, p.y)));

    var pos = p;

    if (pos.x >= 1.0) {
        pos.x = 1.0;
    }
    else if (pos.x <= 0.0) {
        pos.x = 0.0;
    }

    if (pos.y >= 1.0) {
        pos.y = 1.0;
    }
    else if (pos.y <= 0.0) {
        pos.y = 0.0;
    }

    let size = constants.window_width;
    let half = size / 2.0;

    // let pos_x = (pos.x * half) + half;
    // let pos_y = (pos.y * half) + half;

    var pos_x = (pos.x + 1.0) / 2.0 * size;
    var pos_y = (pos.y + 1.0) / 2.0 * size;

    let rounded_x = min(size, max(0.0, floor(pos_x)));
    let rounded_y = min(size, max(0.0, floor(pos_y)));

    let index = i32((size * rounded_y) + rounded_x);

    return index;
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

    let distance = 1.0;

    let pos = map_src.trail[index].position;

    let l_pos = vec2<f32>(pos.x + distance, pos.y);
    let r_pos = vec2<f32>(pos.x - distance, pos.y);
    let t_pos = vec2<f32>(pos.x, pos.y - distance);
    let b_pos = vec2<f32>(pos.x, pos.y + distance);

    let l_index = get_cell_index(l_pos);
    let r_index = get_cell_index(r_pos);
    let t_index = get_cell_index(t_pos);
    let b_index = get_cell_index(b_pos);

    let l_value = map_src.trail[l_index].value * diffusion_rate;
    let r_value = map_src.trail[r_index].value * diffusion_rate;
    let t_value = map_src.trail[t_index].value * diffusion_rate;
    let b_value = map_src.trail[b_index].value * diffusion_rate;

    map_dst.trail[l_index].value = map_src.trail[l_index].value - l_value;
    map_dst.trail[r_index].value = map_src.trail[r_index].value - r_value;
    map_dst.trail[t_index].value = map_src.trail[t_index].value - t_value;
    map_dst.trail[b_index].value = map_src.trail[b_index].value - b_value;

    map_dst.trail[index].value = map_src.trail[index].value + (l_value + r_value + t_value + b_value) * diffusion_strength;
    map_dst.trail[index].value = map_dst.trail[index].value - evaporation_rate * param.delta_time;
    // map_dst.trail[index].value = min(1.0, max(0.0, map_dst.trail[index].value));
}
