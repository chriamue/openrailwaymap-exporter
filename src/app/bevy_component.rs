use crate::app3d::init_with_graph;
use crate::prelude::RailwayGraph;
use wasm_bindgen::JsValue;
use wasm_bindgen_futures::JsFuture;
use yew::prelude::*;

pub struct BevyComponent {
    window_loop: Option<JsFuture>,
}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub graph: Option<RailwayGraph>,
}

fn show_graph(graph: RailwayGraph) {
    init_with_graph(graph);
}

pub enum Msg {}

impl Component for BevyComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self { window_loop: None }
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <canvas id="canvas"></canvas>
        }
    }

    fn changed(&mut self, ctx: &Context<Self>, _old_props: &Self::Properties) -> bool {
        if let Some(graph) = ctx.props().graph.clone() {
            let future = async move {
                show_graph(graph);
                Ok(JsValue::null())
            };

            let promise = wasm_bindgen_futures::future_to_promise(future);
            self.window_loop = Some(JsFuture::from(promise));
        }
        false
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(graph) = ctx.props().graph.clone() {
                let future = async move {
                    show_graph(graph);
                    Ok(JsValue::null())
                };

                let promise = wasm_bindgen_futures::future_to_promise(future);
                self.window_loop = Some(JsFuture::from(promise));
            }
        }
    }
}
