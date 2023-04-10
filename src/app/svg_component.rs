use crate::prelude::{RailwayGraph, SvgEdge, SvgNode};
use petgraph::visit::IntoNodeReferences;
use petgraph::visit::NodeRef;
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub graph: Option<RailwayGraph>,
    pub view_width: f64,
    pub view_height: f64,
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
            let scale_x = ctx.props().view_width / (max_coord.lon - min_coord.lon);
            let scale_y = ctx.props().view_height / (max_coord.lat - min_coord.lat);

            let svg_edges: Vec<Html> = graph
                .graph
                .edge_references()
                .map(|edge| {
                    let edge_data = edge.weight();

                    html! {
                        <SvgEdge edge={edge_data.clone()} scale_x={scale_x} scale_y={scale_y} min_coord={min_coord.clone()} view_height={ctx.props().view_height} />
                    }
                })
                .collect();

            let svg_nodes: Vec<Html> = graph
                .graph
                .node_references()
                .map(|node| {
                    let node_data = node.weight();

                    html! {
                        <SvgNode node={node_data.clone()} scale_x={scale_x} scale_y={scale_y} min_coord={min_coord.clone()} view_height={ctx.props().view_height} />
                    }
                })
                .collect();

            html! {
                <svg xmlns="http://www.w3.org/2000/svg" viewBox={format!("0 0 {} {}", ctx.props().view_width, ctx.props().view_height)}>
                    { for svg_edges }
                    { for svg_nodes }
                </svg>
            }
        } else {
            html! { <></> }
        }
    }
}
