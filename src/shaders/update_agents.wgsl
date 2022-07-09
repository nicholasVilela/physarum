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

    let rounded_x = floor(pos_x);
    let rounded_y = floor(pos_y);

    let index = i32((size * rounded_y) + rounded_x);

    return index;
}

[[group(0), binding(0)]] var<uniform> constants: Constants;
[[group(0), binding(1)]] var<uniform> param: Param;
[[group(0), binding(2)]] var<storage, read_write> agent_src: Agents;
[[group(0), binding(3)]] var<storage, read_write> map: Map;

[[stage(compute), workgroup_size(32)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    var total = arrayLength(&agent_src.agents);
    var index = global_id.x;

    if (index >= total) {
        return;
    }

    var agent = agent_src.agents[index];
    var random = hash(u32(agent.position.y * constants.window_width + agent.position.x) + hash(index + param.frame * 100000u));

    var direction = vec2<f32>(cos(agent.angle), sin(agent.angle));
    var next_position = vec2<f32>(agent.position.x, agent.position.y) + direction * param.delta_time * 1.0;
    var next_angle = agent.angle;

    if (next_position.x < -1.0 || next_position.x >= 1.0 || next_position.y < -1.0 || next_position.y >= 1.0) {
        var random = hash(random);
        var random_angle = scale_to_range_01(random) * 2.0 * 3.14159265359;

        next_angle = min(6.28, max(0.0, random_angle));
        next_position.x = min(1.0, max(-1.0, next_position.x)); 
        next_position.y = min(1.0, max(-1.0, next_position.y));
    }

    let map_index = get_cell_index(next_position.x, next_position.y);
    var trail = map.trail[map_index];
    trail.value = 1.0;
    map.trail[map_index] = trail;

    agent_src.agents[index] = Agent(next_position, next_angle, 0.0);
}
