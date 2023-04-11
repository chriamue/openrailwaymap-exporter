use crate::prelude::RailwayGraph;
use yew::prelude::*;

/// The `NodeContextMenu` component represents a context menu for a node.
pub struct NodeContextMenu {}

pub enum Msg {
    FromHere,
    ToHere,
}

/// The `Props` struct represents the properties of the `NodeContextMenu` component.
#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub node_id: Option<i64>,
    pub graph: Option<RailwayGraph>,
    /// The `on_from_here` property is an optional callback that will be called when the "From here" button is clicked.
    pub on_from_here: Option<Callback<i64>>,
    /// The `on_to_here` property is an optional callback that will be called when the "To here" button is clicked.
    pub on_to_here: Option<Callback<i64>>,
}

impl Component for NodeContextMenu {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        NodeContextMenu {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FromHere => {
                if let (Some(on_from_here), Some(node_id)) =
                    (&ctx.props().on_from_here, ctx.props().node_id)
                {
                    on_from_here.emit(node_id);
                }
            }
            Msg::ToHere => {
                if let (Some(on_to_here), Some(node_id)) =
                    (&ctx.props().on_to_here, ctx.props().node_id)
                {
                    on_to_here.emit(node_id);
                }
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        match (ctx.props().node_id, &ctx.props().graph) {
            (Some(node_id), Some(graph)) => {
                let node_index = graph.node_indices.get(&node_id).unwrap();
                let node = graph.graph.node_weight(node_index.clone()).unwrap();
                let node_id = format!("Node: {}\n", node.id,);
                let node_coordinates = format!("Latitude: {}, Longitude: {}", node.lat, node.lon);

                html! {
                    <div class="node-context-menu">
                        <h2>{node_id}</h2>
                        <p>{node_coordinates}</p>
                        <button onclick={ctx.link().callback(|_| Msg::FromHere)}>
                            { "Start here" }
                        </button>
                        <button onclick={ctx.link().callback(|_| Msg::ToHere)}>
                            { "End here" }
                        </button>
                    </div>
                }
            }
            _ => html! {
                <></>
            },
        }
    }
}
