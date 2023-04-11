use crate::railway_algorithms::PathFinding;
use crate::railway_model::RailwayGraph;
use yew::prelude::*;

/// `PathDisplay` is a Yew component that displays the shortest path and its length between
/// two nodes in a railway graph.
///
/// It takes a `RailwayGraph`, a start node ID, and an end node ID as properties. If the properties
/// are provided and a valid path exists, the component will display the node IDs of the path
/// and the total length of the path in meters.
pub struct PathDisplay {}

/// The properties for the `PathDisplay` component.
#[derive(Properties, PartialEq)]
pub struct Props {
    /// The `RailwayGraph` for which to find the shortest path.
    pub graph: Option<RailwayGraph>,
    /// The ID of the start node for the path.
    pub start_node_id: Option<i64>,
    /// The ID of the end node for the path.
    pub end_node_id: Option<i64>,
}

impl Component for PathDisplay {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let (Some(graph), Some(start), Some(end)) = (
            &ctx.props().graph,
            ctx.props().start_node_id,
            ctx.props().end_node_id,
        ) {
            if let Some(path_nodes) = graph.shortest_path_nodes(start, end) {
                let distance = graph
                    .shortest_path_distance(start, end)
                    .map(|d| format!("{:.2} meters", d))
                    .unwrap_or_else(|| "unknown".to_string());

                html! {
                    <div class="path-display">
                        <div>{ format!("Node IDs: {:?}", path_nodes) }</div>
                        <div>{ format!("Path length: {}", distance) }</div>
                    </div>
                }
            } else {
                html! { <div class="path-display">{ "No path found." }</div> }
            }
        } else {
            html! { <></> }
        }
    }
}
