pub struct SimulationParams {

}

impl SimulationParams {
    pub fn new(device: &wgpu::Device) -> GameResult<SimulationParams> {
        let simulation_params = SimulationParams {};

        return Ok(simulation_params);
    }
}
