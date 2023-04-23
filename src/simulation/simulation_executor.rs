use crate::simulation::Simulation;
use std::time::{Duration, Instant};

/// A struct to execute a simulation with a specific frame rate and runtime.
#[derive(Debug)]
pub struct SimulationExecutor {
    fps: u32,
    run_time: Duration,
    elapsed_time: Duration,
    /// if true, the simulation executor will sleep the thread until fps is reached
    pub sleep_enabled: bool,
}

impl SimulationExecutor {
    /// Creates a new `SimulationExecutor` instance.
    ///
    /// # Arguments
    ///
    /// * `fps` - The desired frame rate for the simulation execution.
    /// * `run_time_secs` - The total runtime of the simulation in seconds.
    ///
    /// # Returns
    ///
    /// A new `SimulationExecutor` instance.
    pub fn new(fps: u32, run_time_secs: u64) -> Self {
        Self {
            fps,
            run_time: Duration::from_secs(run_time_secs),
            elapsed_time: Duration::from_secs(0),
            sleep_enabled: false,
        }
    }

    /// Executes the simulation for the specified runtime with the specified frame rate.
    ///
    /// # Arguments
    ///
    /// * `simulation` - A mutable reference to the `Simulation` instance to be executed.
    pub fn execute(&mut self, simulation: &mut Simulation) {
        let start_time = Instant::now();
        let frame_duration = Duration::from_secs_f64(1.0 / self.fps as f64);

        while self.elapsed_time < self.run_time {
            let frame_start_time = Instant::now();

            if self.sleep_enabled {
                simulation.update(frame_duration);

                let frame_elapsed = frame_start_time.elapsed();
                if frame_duration > frame_elapsed {
                    std::thread::sleep(frame_duration - frame_elapsed);
                }

                self.elapsed_time = start_time.elapsed();
            } else {
                simulation.update(frame_duration);
                self.elapsed_time += frame_duration;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{simulation::Simulation, tests::test_graph_1};
    use approx::assert_relative_eq;

    #[test]
    fn test_simulation_executor() {
        let fps = 60;
        let run_time_secs = 5;

        let graph = test_graph_1();
        let mut simulation: Simulation = Simulation::new(graph);

        let mut simulation_executor = SimulationExecutor::new(fps, run_time_secs);
        simulation_executor.execute(&mut simulation);

        assert_relative_eq!(
            simulation_executor.elapsed_time.as_secs_f64(),
            Duration::from_secs(run_time_secs).as_secs_f64(),
            epsilon = 1.0 / fps as f64
        );
    }
}
