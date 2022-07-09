struct SimulationParams {
    delta_time: f32;
    frame: u32;
};

struct Agent {
    position: vec2<f32>;
    angle: f32;
    species:f32;
};

struct Agents {
    agents: array<Agent>;
};

struct Trail {
    position: vec2<f32>;
    value: f32;
};

struct Map {
    trail: array<Trail>;
};

fn hash(state: u32) -> u32 {
    var res = state;

    res = res ^ 2747636419u;
    res = res * 2654435769u;
    res = res ^ (res >> 16u);
    res = res * 2654435769u;
    res = res ^ (res >> 16u);
    res = res * 2654435769u;

    return res;
}

fn scale_to_range_01(state: u32) -> f32 {
    return f32(state) / 4294967295.0;
}

fn get_cell_index(x: f32, y: f32) -> i32 {
    let size = 500.0;
    let half = size / 2.0;

    var pos_x = (x * half) + half;
    var pos_y = (y * half) + half;

    // if (pos_x < 0.0) { pos_x = pos_x + size; }
    // if (pos_y < 0.0) { pos_y = pos_y + size; }
    // if (pos_x > size - 1.0) { pos_x = pos_x - size; }
    // if (pos_y > size - 1.0) { pos_y = pos_y - size; }

    // let pos_x = ((x + 1.0) / 2.0) * size;
    // let pos_y = ((-y + 1.0) / 2.0) * size;

    let rounded_x = floor(pos_x);
    let rounded_y = floor(pos_y);

    let index = i32((size * rounded_y) + rounded_x);

    return index;
}

// fn get_cell_index(x: f32, y: f32) -> i32 {
//     let size = 500.0;

//     let world_x = ((x + 1.0) / 2.0) * size;
//     let world_y = ((-y + 1.0) / 2.0) * size;

//     var index_x = floor(world_x);
//     var index_y = floor(world_y);

    // if (index_x < 0.0) { index_x = index_x + size; }
    // if (index_y < 0.0) { index_y = index_y + size; }
    // if (index_x > size - 1.0) { index_x = index_x - size; }
    // if (index_y > size - 1.0) { index_y = index_y - size; }

//     let index = i32((size * index_y) + index_x);

//     return index;
// }

// fn get_cell_index(x: f32, y: f32) -> i32 {
//     let size = 500.0;

//     let world_x = (x + 1.0) / 2.0 * size;
//     let world_y = (-y + 1.0) / 2.0 * size;

//     var index_x = floor(world_x);
//     var index_y = floor(world_y);

    // if (index_x < 0.0) { index_x = index_x + size; }
    // if (index_y < 0.0) { index_y = index_y + size; }
    // if (index_x > size - 1.0) { index_x = index_x - size; }
    // if (index_y > size - 1.0) { index_y = index_y - size; }

//     // if (index_x < 0.0) { index_x = 0.0; }
//     // if (index_y < 0.0) { index_y = 0.0; }
//     // if (index_x > size - 1.0) { index_x = size - 1.0; }
//     // if (index_y > size - 1.0) { index_y = size - 1.0; }
    
//     return i32((index_y * (size)) + index_x);
// }

[[group(0), binding(0)]] var<uniform> simulation_params: SimulationParams;
[[group(0), binding(1)]] var<storage, read_write> agent_src: Agents;
[[group(0), binding(2)]] var<storage, read_write> map: Map;

[[stage(compute), workgroup_size(32)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    var total = arrayLength(&agent_src.agents);
    var index = global_id.x;

    if (index >= total) {
        return;
    }

    var agent = agent_src.agents[index];
    var random = hash(u32(agent.position.y * 500.0 + agent.position.x) + hash(index + simulation_params.frame * 100000u));

    var direction = vec2<f32>(cos(agent.angle), sin(agent.angle));
    var next_position = vec2<f32>(agent.position.x, agent.position.y) + direction * simulation_params.delta_time * 1.0;
    var next_angle = agent.angle;

    if (next_position.x < -1.0 || next_position.x >= 1.0 || next_position.y < -1.0 || next_position.y >= 1.0) {
        var random = hash(random);
        var random_angle = scale_to_range_01(random) * 2.0 * 3.14159265359;

        next_angle = min(6.28, max(0.0, random_angle));
        next_position.x = min(1.0, max(-1.0, next_position.x)); 
        next_position.y = min(1.0, max(-1.0, next_position.y));
    }

    // let map_index = get_cell_index(0.0, 0.0);
    let map_index = get_cell_index(next_position.x, next_position.y);
    map.trail[map_index].value = 1.0;

    // map.trail[125250].value = 1.0;

    agent_src.agents[index] = Agent(next_position, next_angle, 0.0);
}
