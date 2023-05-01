//! This module contains types related to handling and processing metric commands during railway simulations.
//!
//! `MetricsCommand` is an enumeration of subcommands for working with metrics in the simulation.
//! It provides the `List` and `Get` subcommands to list available metrics and get a specific metric's value.
//!
//! This module integrates with the `Simulation` struct and the `MetricsHandler` trait to process and display metric information.

use crate::simulation::commands::SimulationCommand;
use crate::simulation::metrics::{ActionCountHandler, MetricsHandler, TargetReachedHandler};
use crate::simulation::Simulation;
use clap::Parser;

/// An enumeration of subcommands for working with metrics in the simulation.
#[derive(Parser, Debug)]
#[command(name = "metrics")]
pub enum MetricsCommand {
    /// Lists the names of all available metrics.
    #[command(name = "list")]
    List,

    /// Retrieves the value of a specific metric.
    #[command(name = "get")]
    Get(GetMetric),
}

/// A struct representing the `Get` subcommand for retrieving a specific metric's value.
#[derive(Parser, Debug)]
#[command(name = "get")]
pub struct GetMetric {
    /// The identifier of the metric to retrieve.
    #[clap(name = "ID")]
    pub metric_id: String,
}

/// Implements the `SimulationCommand` trait for the `MetricsCommand` enumeration.
impl SimulationCommand for MetricsCommand {
    /// Executes the metric command, either listing available metrics or getting a specific metric's value.
    ///
    /// # Returns
    ///
    /// An optional `String` containing the command's output.
    fn execute(&self, simulation: &mut Simulation) -> Option<String> {
        match self {
            MetricsCommand::List => {
                let metric_names = vec!["ActionCount", "TargetReached"];
                Some(format!("Available metrics: {:?}", metric_names))
            }
            MetricsCommand::Get(get_metric) => {
                let metric_value = match get_metric.metric_id.as_str() {
                    "ActionCount" => {
                        let handler = simulation.metrics_handlers.iter().find_map(|handler| {
                            handler.as_any().downcast_ref::<ActionCountHandler>()
                        });
                        handler.map(|h| h.get_value())
                    }
                    "TargetReached" => {
                        let handler = simulation.metrics_handlers.iter().find_map(|handler| {
                            handler.as_any().downcast_ref::<TargetReachedHandler>()
                        });
                        handler.map(|h| h.get_value())
                    }
                    _ => None,
                };

                if let Some(value) = metric_value {
                    Some(format!("{}: {}", get_metric.metric_id, value))
                } else {
                    Some(format!("Metric '{}' not found", get_metric.metric_id))
                }
            }
        }
    }
}
