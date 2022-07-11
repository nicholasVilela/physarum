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


[[group(0), binding(0)]] var<storage, read_write> map: Map;
[[group(0), binding(1)]] var<uniform> constants: Constants;
[[group(0), binding(2)]] var<uniform> param: Param;

fn who_cell(x : i32, y : i32) -> i32 {
    let size = i32(constants.window_width);

    var _x = min(size, max(0, x));
    var _y = min(size, max(0, y));

    // var _x = x;
    // var _y = y;

    // if ( _x >= size ) { _x = size; }
    // if ( _x < 0 ) { _x = 0; }
    // if ( _y < 0 ) { _y = 0;}
    // if ( _y >= size ) { _y = size; }

    return  (_y * size) + _x ;
}

// [[stage(compute), workgroup_size(32)]]
// fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
//     let index = global_id.x;

//     let size = constants.window_width;
//     let evaporation_rate = constants.evaporation_rate;
//     let diffusion_rate = constants.diffusion_rate;
//     let diffusion_strength = constants.diffusion_strength;

//     var trail: Trail = map.trail[index];
//     var sum: f32 = 0.0;

//     {
//         var x: f32 = -1.0;

//         loop {
//             if (x <= 1.0) {
//                 break;
//             }

//             {
//                 var y: f32 = -1.0;

//                 loop {
//                     if (y <= 1.0) {
//                         break;
//                     }

//                     let pos_x = i32(min(size - 1.0, max(0.0, f32(index) + x)));
//                     let pos_y = i32(min(size - 1.0, max(0.0, f32(index) + y)));

//                     sum = sum + map.trail[who_cell(pos_x, pos_y)].value;

//                     continuing {
//                         y = y + 1.0;
//                     }
//                 }
//             }

//             continuing {
//                 x = x + 1.0;
//             }
//         }
//     }

//     var blurred = sum / 9.0;
//     let diffuse_weight = constants.diffusion_rate * param.delta_time;

//     blurred = trail.value * (1.0 - diffuse_weight) + blurred * diffuse_weight;

//     trail.value = max(0.0, blurred - evaporation_rate * param.delta_time);

//     map.trail[index] = trail;
// }

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

// [[stage(compute), workgroup_size(32)]]
// fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
//     let index = global_id.x;

//     let size = constants.window_width;
//     let evaporation_rate = constants.evaporation_rate;
//     let diffusion_rate = constants.diffusion_rate;
//     let diffusion_strength = constants.diffusion_strength;

//     var trail = map.trail[index];

//     var sum = 0.0;

//     {
//         var x = -1;
//         loop {

//             continuing {

//             }
//         }
//     }

//     trail.value = max(0.0, trail.value - evaporation_rate * param.delta_time);

//     map.trail[index] = trail;
// }

[[stage(compute), workgroup_size(32)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    let size = constants.window_width;
    let index = global_id.x;

    // if (index >= u32(size * size)) {
    //     return;
    // }

    let evaporation_rate = constants.evaporation_rate;
    let diffusion_rate = constants.diffusion_rate;
    let diffusion_strength = constants.diffusion_strength;

    let distance = 0.1;

    let cell_x = map.trail[index].position.x;
    let cell_y = map.trail[index].position.y;

    // let cell_x = i32(i32(index) % i32(size));
    // let cell_y = i32(i32(index) / i32(size));

    let center = map.trail[get_cell_index(cell_x, cell_y)];
    map.trail[index].value = center.value * evaporation_rate;

    var left_cell_distance = cell_x - distance;
    var right_cell_distance = cell_x + distance;
    var top_cell_distance = cell_y - distance;
    var bottom_cell_distance = cell_y + distance;

    if (left_cell_distance < 0.0) {
        left_cell_distance = 0.0;
    }
    else if (left_cell_distance > 1.0) {    
        left_cell_distance = 1.0;
    }
    
    if (right_cell_distance < 0.0) {
        right_cell_distance = 0.0;
    }
    else if (right_cell_distance > 1.0) {
        right_cell_distance = 1.0;
    }

    if (top_cell_distance < 0.0) {
        top_cell_distance = 0.0;
    }
    else if (top_cell_distance > 1.0) {
        top_cell_distance = 1.0;
    }

    if (bottom_cell_distance < 0.0) {
        bottom_cell_distance = 0.0;
    }
    else if (bottom_cell_distance > 1.0) {
        bottom_cell_distance = 1.0;
    }

    let _left = get_cell_index(left_cell_distance, cell_y);
    let _right = get_cell_index(right_cell_distance, cell_y);
    let _top = get_cell_index(cell_x, top_cell_distance);
    let _bottom = get_cell_index(cell_x, bottom_cell_distance);

    let _take_left = map.trail[_left].value * diffusion_rate;
    map.trail[_left].value = map.trail[_left].value - _take_left;

    let _take_right = map.trail[_right].value * diffusion_rate;
    map.trail[_right].value = map.trail[_right].value - _take_right;
    
    let _take_top = map.trail[_top].value * diffusion_rate;
    map.trail[_top].value = map.trail[_top].value - _take_top;
    
    let _take_bottom = map.trail[_bottom].value * diffusion_rate;
    map.trail[_bottom].value = map.trail[_bottom].value - _take_bottom;

    map.trail[index].value = map.trail[index].value + (_take_left + _take_right + _take_top + _take_bottom) * diffusion_strength;

    map.trail[index].value = min(1.0, max(0.0, map.trail[index].value));

    // if (map.trail[index].value > 1.0) {
    //     map.trail[index].value = 1.0;
    // }
    // else if (map.trail[index].value < 0.0) {
    //     map.trail[index].value = 0.0;
    // }
}
