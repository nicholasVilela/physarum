struct Trail {
    position: vec2<f32>;
    value: f32;
};

struct Map {
    trail: array<Trail>;
};


// fn who_cell (x : i32, y : i32) -> i32 {
//     var _x = x;
//     var _y = y;

//     let size = 500.0;

//     if ( _x >= i32(size) ) { _x = _x - i32(size); }
//     if ( _x < 0 ) { _x = _x + i32(size); }
//     if ( _y < 0 ) { _y = _y + i32(size); }
//     if ( _y >= i32(size) ) { _y = _y - i32(size); }

//     return  _y * i32(size) + _x ;
// }

// fn read_cell ( x : i32, y : i32) -> f32 {
//     return map[ who_cell(x, y) ];
// }

[[group(0), binding(0)]] var<storage, read_write> map: Map;

[[stage(compute), workgroup_size(32)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    let index = global_id.x;

    let size = 500.0;

    let cell_x = i32(i32(index) % i32(size));
    let cell_y = i32(i32(index) / i32(size));
    
    var trail = map.trail[index];

    trail.value = trail.value - 0.01;

    map.trail[index] = trail;
}
