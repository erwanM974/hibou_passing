use std::fmt::format;
use graph_process_manager_core::manager::config::AbstractProcessParameterization;
use crate::process::ana::param::anakind::SimulationConfiguration;

pub struct AnalysisParameterization {
    pub use_simulation : Option<SimulationConfiguration>,
    pub use_locana : bool
}

impl AnalysisParameterization {
    pub fn new(use_simulation: Option<SimulationConfiguration>, use_locana: bool) -> Self {
        Self { use_simulation, use_locana }
    }
}

impl AbstractProcessParameterization for AnalysisParameterization {
    fn get_param_as_strings(&self) -> Vec<String> {
        let simu = match &self.use_simulation {
            None => {"no".to_string()},
            Some(sim_config) => {format!("{:}",sim_config)}
        };
        vec![ "process = analysis".to_string(),
              format!("simulation = {:}", simu),
              format!("local analysis = {}", self.use_locana.to_string()) ]
    }
}