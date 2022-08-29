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
        p.x = 0;
    }
    else if (p.x < 0) {
        p.x = size;
    }

    if (p.y > size) {
        p.y = 0;
    }
    else if (p.y < 0) {
        p.y = size;
    }

    return (p.y * size) + p.x;
}

[[stage(compute), workgroup_size(32)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    let size = constants.window_width;
    let index = global_id.x;

    let evaporation_rate = constants.evaporation_rate;
    let diffusion_rate = constants.diffusion_rate;

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

    let m_l = map_src.trail[l_index].value - l_value;
    let m_r = map_src.trail[r_index].value - r_value;
    let m_t = map_src.trail[t_index].value - t_value;
    let m_b = map_src.trail[b_index].value - b_value;

    let average = (m_l + m_r + m_t + m_b) / 4.0;

    map_dst.trail[index].value = average;
}
