struct Constants {
    window_height: f32;
    window_width: f32;
    evaporation_rate: f32;
    diffusion_rate: f32;
    diffusion_strength: f32;
};

struct Trail {
    position: vec2<f32>;
    value: f32;
};

struct Map {
    trail: array<Trail>;
};


fn who_cell(x : i32, y : i32) -> i32 {
    var _x = x;
    var _y = y;

    let size = 500.0;

    if ( _x >= i32(size) ) { _x = _x - i32(size); }
    if ( _x < 0 ) { _x = _x + i32(size); }
    if ( _y < 0 ) { _y = _y + i32(size); }
    if ( _y >= i32(size) ) { _y = _y - i32(size); }

    return  _y * i32(size) + _x ;
}

[[group(0), binding(0)]] var<storage, read_write> map: Map;
[[group(0), binding(1)]] var<uniform> constants: Constants;

[[stage(compute), workgroup_size(32)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    let index = global_id.x;

    let size = constants.window_width;
    let evaporation_rate = constants.evaporation_rate;
    let diffusion_rate = constants.diffusion_rate;
    let diffusion_strength = constants.diffusion_strength;

    let cell_x = i32(i32(index) % i32(size));
    let cell_y = i32(i32(index) / i32(size));

    let center_cell_index = who_cell(cell_x, cell_y);
    let center = map.trail[center_cell_index];

    map.trail[index].value = map.trail[index].value - (evaporation_rate * center.value);

    let _left = who_cell(cell_x - 1, cell_y);
    let _right = who_cell(cell_x + 1, cell_y);
    let _top = who_cell(cell_x, cell_y - 1);
    let _bottom = who_cell(cell_x, cell_y + 1);

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
