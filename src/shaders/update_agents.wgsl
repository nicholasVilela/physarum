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
    seed: f32;
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

[[group(0), binding(0)]] var<uniform> constants: Constants;
[[group(0), binding(1)]] var<uniform> param: Param;
[[group(0), binding(2)]] var<storage, read_write> agent_src: Agents;
[[group(0), binding(3)]] var<storage, read> map_src: Map;
[[group(0), binding(4)]] var<storage, read_write> map_dst: Map;

fn get_cell_index(p: vec2<f32>) -> i32 {
    // let pos = vec2<f32>(min(1.0, max(-1.0, p.x)), min(1.0, max(-1.0, p.y)));

    var pos = p;

    if (pos.x > 1.0) {
        pos.x = 1.0;
    }
    else if (pos.x < -1.0) {
        pos.x = -1.0;
    }

    if (pos.y > 1.0) {
        pos.y = 1.0;
    }
    else if (pos.y < -1.0) {
        pos.y = -1.0;
    }

    let size = constants.window_width;
    let half = size / 2.0;

    let pos_x = (pos.x * half) + half;
    let pos_y = (pos.y * half) + half;

    // var pos_x = (pos.x + 1.0) / 2.0 * size;
    // var pos_y = (pos.y + 1.0) / 2.0 * size;

    // let rounded_x = min(size, max(0.0, floor(pos_x)));
    // let rounded_y = min(size, max(0.0, floor(pos_y)));

    let rounded_x = floor(pos_x);
    let rounded_y = floor(pos_y);

    let index = i32((size * rounded_y) + rounded_x);

    return index;
}

fn sense(agent: Agent, sensor_size: f32, sensor_distance: f32, sensor_angle_offset: f32) -> f32 {
    let width = constants.window_width;
    let height = constants.window_height;

    let sensor_angle = agent.angle + sensor_angle_offset;
    let sensor_direction = vec2<f32>(cos(sensor_angle), sin(sensor_angle));
    let sensor_position = agent.position + sensor_direction * sensor_distance;
    
    var sum = 0.0;

    {
        var x: f32 = -sensor_size;

        loop {
            var y: f32 = -sensor_size;

            if (x == sensor_size + 1.0) {
                break;
            }

            loop {
                if (y == sensor_size + 1.0) {
                    break;
                }

                var pos = vec2<f32>(sensor_position.x, sensor_position.y);

                let sample = map_dst.trail[get_cell_index(pos)];

                sum = sum + sample.value;

                continuing {
                    y = y + 1.0;
                }
            }

            continuing {
                x = x + 1.0;
            }
        }
    }

    return sum;
}

[[stage(compute), workgroup_size(32)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    var total = arrayLength(&agent_src.agents);
    var index = global_id.x;

    if (index >= total) {
        return;
    }

    let width = constants.window_width;
    let height = constants.window_height;

    let TAU = 6.28318530717958647692528676655900577;
    let PI = 3.14159265358979323846264338327950288;

    var agent = agent_src.agents[index];
    let seed = u32(agent.position.y * constants.window_width + agent.position.x) * u32(agent.seed);
    var random = hash(seed + hash(u32(agent.seed)) + hash(index + param.frame));

    // let sensor_size = 1.0;
    // let sensor_angle = 50.0;
    // let sensor_distance = 0.01;
    // let turn_speed = 10.0;
    // let move_speed = 0.3;
    // let forward_random_strength = -0.5;
    // let right_random_strength = 0.0;
    // let left_random_strength = 0.0;

    let sensor_size = 1.0;
    let sensor_angle = 20.0;
    let sensor_distance = 0.1;
    let turn_speed = 5.0;
    let move_speed = 1.0;
    let forward_random_strength = -0.5;
    let right_random_strength = 0.0;
    let left_random_strength = 0.0;

    let sensor_angle_rad = sensor_angle * (PI / 180.0);
    let weight_forward = sense(agent, sensor_size, sensor_distance, 0.0);
    let weight_left = sense(agent, sensor_size, sensor_distance, sensor_angle_rad);
    let weight_right = sense(agent, sensor_size, sensor_distance, -sensor_angle_rad);

    let mod_turn_speed = turn_speed * TAU;
    let random_steer_strength = scale_to_range_01(random);

    if (weight_forward > weight_left && weight_forward > weight_right) {
        agent.angle = agent.angle;
    }
    else if (weight_forward < weight_left && weight_forward < weight_right) {
        agent.angle = agent.angle + (random_steer_strength + forward_random_strength) * 2.0 * mod_turn_speed * param.delta_time;
    }
    else if (weight_right > weight_left) {
        agent.angle = agent.angle - (random_steer_strength + right_random_strength) * mod_turn_speed * param.delta_time;
    }
    else if (weight_left > weight_right) {
        agent.angle = agent.angle + (random_steer_strength + left_random_strength) * mod_turn_speed * param.delta_time;
    }

    var direction = vec2<f32>(cos(agent.angle), sin(agent.angle));
    var next_position = vec2<f32>(agent.position.x, agent.position.y) + direction * param.delta_time * move_speed;
    var next_angle = agent.angle;

    if (next_position.x <= -1.0 || next_position.x >= 1.0 || next_position.y <= -1.0 || next_position.y >= 1.0) {
        var random = hash(random + u32(agent.seed));
        var random_angle = scale_to_range_01(random) * TAU;

        // next_position.x = min(1.0, max(-1.0, next_position.x)); 
        // next_position.y = min(1.0, max(-1.0, next_position.y));

        if (next_position.x >= 1.0) {
            next_position.x = 1.0;
        }
        else if (next_position.x <= -1.0) {
            next_position.x = -1.0;
        }

        if (next_position.y >= 1.0) {
            next_position.y = 1.0;
        }
        else if (next_position.y <= -1.0) {
            next_position.y = -1.0;
        }

        next_angle = random_angle;
    }
    else {
        let map_index = get_cell_index(next_position);
        map_dst.trail[map_index].value = 1.0;
    }

    agent_src.agents[index] = Agent(next_position, next_angle, 0.0);
}
