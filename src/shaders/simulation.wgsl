struct Agent {
    position: vec2<f32>;
    angle: f32;
};

struct Agents {
    agents: array<Agent>;
};

[[group(0), binding(0)]] var<storage, read> agent_src: Agents;
[[group(0), binding(1)]] var<storage, read_write> agent_dst: Agents;

[[stage(compute), workgroup_size(32)]]
fn main([[builtin(global_invocation_id)]] global_id: vec3<u32>) {
    var index = global_id.x;

    var target_position = agent_src.agents[index].position;
    var target_angle = agent_src.agents[index].angle;

    target_position.x = target_position.x + 0.1;

    agent_dst.agents[index] = Agent(vec2<f32>(0.0, 0.0), target_angle);
}
