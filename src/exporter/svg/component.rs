//! A module containing the `SvgComponent`, which renders a `RailwayGraph` as an SVG.
use super::{edge::SvgEdge, node::SvgNode};
use crate::prelude::RailwayEdge;
use crate::prelude::RailwayGraph;
use crate::prelude::RailwayGraphExt;
use crate::railway_algorithms::PathFinding;
use crate::types::NodeId;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeRef;
use yew::prelude::*;

/// Properties for the `SvgComponent`.
#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    /// The `RailwayGraph` to render.
    pub graph: Option<RailwayGraph>,
    /// The width of the SVG view.
    pub view_width: f64,
    /// The height of the SVG view.
    pub view_height: f64,
    /// A callback that gets called when a node is selected.
    pub on_select_node: Option<Callback<NodeId>>,
    /// The starting node ID for pathfinding.
    pub start_node_id: Option<NodeId>,
    /// The ending node ID for pathfinding.
    pub end_node_id: Option<NodeId>,
}

/// A component that renders a `RailwayGraph` as an SVG.
pub struct SvgComponent {}

impl Component for SvgComponent {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(graph) = ctx.props().graph.as_ref() {
            let (min_coord, max_coord) = graph.bounding_box();
            let scale_x = ctx.props().view_width / (max_coord.x - min_coord.x);
            let scale_y = ctx.props().view_height / (max_coord.y - min_coord.y);

            let svg_edges: Vec<Html> = graph
                .physical_graph.graph
                .edge_references()
                .map(|edge| {
                    let edge_data = edge.weight();

                    html! {
                        <SvgEdge edge={edge_data.clone()} scale_x={scale_x} scale_y={scale_y} min_coord={(min_coord.x, min_coord.y)} view_height={ctx.props().view_height} />
                    }
                })
                .collect();

            let svg_nodes: Vec<Html> = graph
                .physical_graph.graph
                .node_references()
                .map(|node| {
                    let node_data = node.weight();

                    html! {
                        <SvgNode node={*node_data} scale_x={scale_x} scale_y={scale_y}
                         min_coord={(min_coord.x, min_coord.y)} view_height={ctx.props().view_height}
                         on_select={ctx.props().on_select_node.clone()} />
                    }
                })
                .collect();

            let path_edges: Option<Vec<RailwayEdge>> =
                if let (Some(start_node_id), Some(end_node_id)) =
                    (ctx.props().start_node_id, ctx.props().end_node_id)
                {
                    // Use graph.shortest_path_edges to get the Vec of edge IDs
                    let path_edge_ids = graph.shortest_path_edges(start_node_id, end_node_id);

                    // If path_edge_ids is Some, map the edge IDs to RailwayEdge instances
                    path_edge_ids.map(|ids| {
                        ids.into_iter()
                            .filter_map(|id| {
                                graph
                                    .physical_graph
                                    .graph
                                    .edge_references()
                                    .find(|edge| edge.weight().id == id)
                                    .map(|edge| edge.weight().clone())
                            })
                            .collect::<Vec<RailwayEdge>>()
                    })
                } else {
                    None
                };

            let path_edges: Vec<Html> = path_edges
                .unwrap_or_default()
                .iter()
                .map(|edge_data| {
                    html! {
                        <SvgEdge
                            edge={edge_data.clone()}
                            scale_x={scale_x}
                            scale_y={scale_y}
                            min_coord={(min_coord.x, min_coord.y)}
                            view_height={ctx.props().view_height}
                            stroke_color={Some("red".to_string())}
                        />
                    }
                })
                .collect();

            html! {
                <svg xmlns="http://www.w3.org/2000/svg" viewBox={format!("0 0 {} {}", ctx.props().view_width, ctx.props().view_height)}>
                    { for svg_edges }
                    { for svg_nodes }
                    { for path_edges }
                </svg>
            }
        } else {
            html! { <></> }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;
    use crate::importer::overpass_importer::{
        from_railway_elements, Coordinate, ElementType, RailwayElement,
    };
    use yew::LocalServerRenderer;

    #[cfg_attr(not(target_arch = "wasm32"), tokio::test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    async fn test_render() {
        let elements = vec![
            RailwayElement {
                id: 1,
                element_type: ElementType::Node,
                lat: Some(50.1109),
                lon: Some(8.6821),
                tags: Some(HashMap::new()),
                nodes: None,
                geometry: None,
            },
            RailwayElement {
                id: 2,
                element_type: ElementType::Node,
                lat: Some(50.1122),
                lon: Some(8.6833),
                tags: Some(HashMap::new()),
                nodes: None,
                geometry: None,
            },
            RailwayElement {
                id: 3,
                element_type: ElementType::Way,
                lat: None,
                lon: None,
                tags: Some(HashMap::new()),
                nodes: Some(vec![1, 2]),
                geometry: Some(vec![
                    Coordinate {
                        lat: 50.1109,
                        lon: 8.6821,
                    },
                    Coordinate {
                        lat: 50.1122,
                        lon: 8.6833,
                    },
                ]),
            },
        ];

        let railway_graph = from_railway_elements(&elements);
        let props = Props {
            graph: Some(railway_graph),
            view_width: 200.0,
            view_height: 100.0,
            on_select_node: None,
            start_node_id: None,
            end_node_id: None,
        };

        let rendered = LocalServerRenderer::<SvgComponent>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("<svg"));
        assert!(rendered.contains("<path"));
        assert!(rendered.contains("stroke=\"black\""));
        assert!(rendered.contains("stroke-width=\"1\""));
        assert!(rendered.contains("<circle"));
        assert!(rendered.contains("cx="));
        assert!(rendered.contains("cy="));
        assert!(rendered.contains("r=\"2\""));
        assert!(rendered.contains("fill=\"red\""));
    }
}
