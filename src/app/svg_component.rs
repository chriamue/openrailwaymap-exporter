use crate::prelude::RailwayEdge;
use crate::prelude::{RailwayGraph, SvgEdge, SvgNode};
use crate::railway_algorithms::PathFinding;
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeRef;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub graph: Option<RailwayGraph>,
    pub view_width: f64,
    pub view_height: f64,
    pub on_select_node: Option<Callback<i64>>,
    pub start_node_id: Option<i64>,
    pub end_node_id: Option<i64>,
}

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
                .graph
                .edge_references()
                .map(|edge| {
                    let edge_data = edge.weight();

                    html! {
                        <SvgEdge edge={edge_data.clone()} scale_x={scale_x} scale_y={scale_y} min_coord={(min_coord.x, min_coord.y)} view_height={ctx.props().view_height} />
                    }
                })
                .collect();

            let svg_nodes: Vec<Html> = graph
                .graph
                .node_references()
                .map(|node| {
                    let node_data = node.weight();

                    html! {
                        <SvgNode node={node_data.clone()} scale_x={scale_x} scale_y={scale_y}
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
