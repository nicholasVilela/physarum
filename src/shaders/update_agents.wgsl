struct SimulationParams {
    delta_time: f32;
    frame: u32;
};

struct Agent {
    data: vec4<f32>;
};

struct Agents {
    agents: array<Agent>;
};

struct Map {
    trail: array<f32>;
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
    let world_x = ( x + 1.0) / 2.0 *size;
    let world_y = ( -y + 1.0) / 2.0 *size;

    var index_x = floor( world_x );
    var index_y = floor( world_y );

    if (index_x < 0.0 ) { index_x = index_x +size; }
    if (index_y < 0.0 ) { index_y = index_y +size; }
    if (index_y >size - 1.0 ) { index_y = index_y -size; }
    if (index_x >size - 1.0 ) { index_x = index_x -size; }
    
    return i32(index_y *size + index_x);
}

[[group(0), binding(0)]] var<uniform> simulation_params: SimulationParams;
[[group(0), binding(1)]] var<storage, read_write> agent_src: Agents;
[[group(0), binding(2)]] var<storage, read_write> map: Map;
// [[group(0), binding(2)]] var<storage, read_write> agent_dst: Agents;

[[stage(compute), workgroup_size(32)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    var total = arrayLength(&agent_src.agents);
    var index = global_id.x;

    if (index >= total) {
        return;
    }

    var agent = agent_src.agents[index];
    var random = hash(u32(agent.data[1] * 500.0 + agent.data[0]) + hash(index + simulation_params.frame * 100000u));

    var direction = vec2<f32>(cos(agent.data[2]), sin(agent.data[2]));
    var next_position = vec2<f32>(agent.data[0], agent.data[1]) + direction * simulation_params.delta_time * 1.0;
    var next_angle = agent.data[2];
    // agent.data[0] = agent.data[0] + 0.001;

    // agent.position = agent.position + vec2<f32>(0.001, 0.0);
    // var direction = vec2<f32>(cos(0.0), sin(0.0));
    // var next_position = agent.position + direction * simulation_params.delta_time * 1.0;
    // var next_angle = agent.angle;

    if (next_position.x < -1.0 || next_position.x >= 1.0 || next_position.y < -1.0 || next_position.y >= 1.0) {
        var random = hash(random);
        var random_angle = scale_to_range_01(random) * 2.0 * 3.14159265359;

        next_angle = min(6.28, max(0.0, random_angle));
        next_position.x = min(1.0, max(-1.0, next_position.x)); 
        next_position.y = min(1.0, max(-1.0, next_position.y));
    }

    let map_index = get_cell_index(next_position.x, next_position.y);
    map.trail[map_index] = 1.0;
    var data = vec4<f32>(next_position.x, next_position.y, next_angle, 0.0);
    agent_src.agents[index] = Agent(data);
}
