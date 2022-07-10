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
    var _x = x;
    var _y = y;

    let size = constants.window_width;

    if ( _x >= i32(size) ) { _x = _x - i32(size); }
    if ( _x < 0 ) { _x = _x + i32(size); }
    if ( _y < 0 ) { _y = _y + i32(size); }
    if ( _y >= i32(size) ) { _y = _y - i32(size); }

    return  _y * i32(size) + _x ;
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
    let size = 500.0;
    let half = size / 2.0;

    var pos_x = (x * half) + half;
    var pos_y = (y * half) + half;

    let rounded_x = floor(pos_x);
    let rounded_y = floor(pos_y);

    let index = i32((size * rounded_y) + rounded_x);

    return index;
}

[[stage(compute), workgroup_size(32)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    let index = global_id.x;

    let size = constants.window_width;
    let evaporation_rate = constants.evaporation_rate;
    let diffusion_rate = constants.diffusion_rate;
    let diffusion_strength = constants.diffusion_strength;

    let distance = 1;

    // let cell_x = map.trail[index].position.x;
    // let cell_y = map.trail[index].position.y;

    let cell_x = i32(i32(index) % i32(size));
    let cell_y = i32(i32(index) / i32(size));

    // let cell_x = i32((map.trail[index].position.x * (size / 2.0)) + size / 2.0);
    // let cell_y = i32((map.trail[index].position.y * (size / 2.0)) + size / 2.0);

    let center_cell_index = who_cell(cell_x, cell_y);
    let center = map.trail[center_cell_index];

    map.trail[index].value = center.value * evaporation_rate;

    let _left = who_cell(cell_x - distance, cell_y);
    let _right = who_cell(cell_x + distance, cell_y);
    let _top = who_cell(cell_x, cell_y - distance);
    let _bottom = who_cell(cell_x, cell_y + distance);

    let _take_left = map.trail[_left].value * diffusion_rate;
    map.trail[_left].value = map.trail[_left].value - _take_left;

    let _take_right = map.trail[_right].value * diffusion_rate;
    map.trail[_right].value = map.trail[_right].value - _take_right;
    
    let _take_top = map.trail[_top].value * diffusion_rate;
    map.trail[_top].value = map.trail[_top].value - _take_top;
    
    let _take_bottom = map.trail[_bottom].value * diffusion_rate;
    map.trail[_bottom].value = map.trail[_bottom].value - _take_bottom;

    map.trail[index].value = map.trail[index].value + (_take_left + _take_right + _take_top + _take_bottom) * diffusion_strength;

    if (map.trail[index].value > 1.0) {
        map.trail[index].value = 1.0;
    }
    if (map.trail[index].value < 0.00001) {
        map.trail[index].value = 0.0;
    }
}
