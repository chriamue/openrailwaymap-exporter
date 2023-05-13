mod component;
mod edge;
mod node;
use std::error::Error;

pub use component::{Props, SvgComponent};
use yew::LocalServerRenderer;

use crate::prelude::RailwayGraph;

/// Generates an SVG string representation of a given RailwayGraph.
///
/// The SVG string can be used to visualize the graph.
///
/// # Arguments
///
/// * `graph` - A reference to a RailwayGraph.
///
/// # Returns
///
/// A `Result` containing an SVG-formatted `String` on success, or a `Box<dyn Error>` on failure.
pub fn generate_svg_string(graph: &RailwayGraph) -> Result<String, Box<dyn Error>> {
    let width = 2500.0;
    let height = 2500.0;
    let renderer = LocalServerRenderer::<SvgComponent>::with_props(Props {
        graph: Some(graph.clone()),
        view_width: width,
        view_height: height,
        on_select_node: None,
        start_node_id: None,
        end_node_id: None,
    })
    .hydratable(false);
    #[cfg(target_arch = "wasm32")]
    {
        wasm_bindgen_futures::spawn_local(async move {
            renderer.render().await;
        });
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "Not supported on wasm32 target",
        )))
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        let svg_graph = futures::executor::block_on(renderer.render());
        Ok(svg_graph)
    }
}
