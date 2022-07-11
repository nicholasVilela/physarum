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

fn who_cell(x : i32, y : i32) -> i32 {
    let size = i32(constants.window_width);

    var _x = min(size, max(0, x));
    var _y = min(size, max(0, y));

    return  (_y * size) + _x ;
}

fn get_cell_index(x: f32, y: f32) -> i32 {
    let x = min(1.0, max(-1.0, x));
    let y = min(1.0, max(-1.0, y));

    let size = constants.window_width;
    let half = size / 2.0;

    let pos_x = (x * half) + half;
    let pos_y = (y * half) + half;

    let rounded_x = floor(pos_x);
    let rounded_y = floor(pos_y);

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

    let distance = 0.1;

    let cell_x = map.trail[index].position.x;
    let cell_y = map.trail[index].position.y;

    map.trail[index].value = map.trail[index].value - evaporation_rate * param.delta_time;

    let left_cell_distance = vec2<f32>(cell_x + distance, cell_y);
    let right_cell_distance = vec2<f32>(cell_x - distance, cell_y);

    let left_index = get_cell_index(left_cell_distance.x, left_cell_distance.y);
    let right_index = get_cell_index(right_cell_distance.x, right_cell_distance.y);

    let take_left = map.trail[left_index].value * diffusion_rate;
    map.trail[left_index].value = map.trail[left_index].value - take_left;

    let take_right = map.trail[right_index].value * diffusion_rate;
    map.trail[right_index].value = map.trail[right_index].value - take_right;

    map.trail[index].value = map.trail[index].value + (take_left + take_right) * diffusion_strength;

    // let cell_x = i32(i32(index) % i32(size));
    // let cell_y = i32(i32(index) / i32(size));

    // let center = map.trail[get_cell_index(cell_x, cell_y)];
    // map.trail[index].value = center.value * evaporation_rate;

    // var left_cell_distance = cell_x - distance;
    // var right_cell_distance = cell_x + distance;
    // var top_cell_distance = cell_y - distance;
    // var bottom_cell_distance = cell_y + distance;

    // if (left_cell_distance < 0.0) {
    //     left_cell_distance = 0.0;
    // }
    // else if (left_cell_distance > 1.0) {    
    //     left_cell_distance = 1.0;
    // }
    
    // if (right_cell_distance < 0.0) {
    //     right_cell_distance = 0.0;
    // }
    // else if (right_cell_distance > 1.0) {
    //     right_cell_distance = 1.0;
    // }

    // if (top_cell_distance < 0.0) {
    //     top_cell_distance = 0.0;
    // }
    // else if (top_cell_distance > 1.0) {
    //     top_cell_distance = 1.0;
    // }

    // if (bottom_cell_distance < 0.0) {
    //     bottom_cell_distance = 0.0;
    // }
    // else if (bottom_cell_distance > 1.0) {
    //     bottom_cell_distance = 1.0;
    // }

    // let _left = get_cell_index(left_cell_distance, cell_y);
    // let _right = get_cell_index(right_cell_distance, cell_y);
    // let _top = get_cell_index(cell_x, top_cell_distance);
    // let _bottom = get_cell_index(cell_x, bottom_cell_distance);

    // let _take_left = map.trail[_left].value * diffusion_rate;
    // map.trail[_left].value = map.trail[_left].value - _take_left;

    // let _take_right = map.trail[_right].value * diffusion_rate;
    // map.trail[_right].value = map.trail[_right].value - _take_right;
    
    // let _take_top = map.trail[_top].value * diffusion_rate;
    // map.trail[_top].value = map.trail[_top].value - _take_top;
    
    // let _take_bottom = map.trail[_bottom].value * diffusion_rate;
    // map.trail[_bottom].value = map.trail[_bottom].value - _take_bottom;

    // map.trail[index].value = map.trail[index].value + (_take_left + _take_right + _take_top + _take_bottom) * diffusion_strength;

    map.trail[index].value = min(1.0, max(0.0, map.trail[index].value));

    // if (map.trail[index].value > 1.0) {
    //     map.trail[index].value = 1.0;
    // }
    // else if (map.trail[index].value < 0.0) {
    //     map.trail[index].value = 0.0;
    // }
}
