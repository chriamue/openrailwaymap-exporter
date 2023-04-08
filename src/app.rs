use crate::generate_svg_string;
use crate::BasicOpenRailwayMapApiClient;
use crate::OpenRailwayMapApiClient;
use crate::RailwayElement;
use crate::RailwayGraph;
use wasm_bindgen::prelude::*;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::html::Scope;
use yew::prelude::*;

pub struct App {
    link: Scope<Self>,
    input_area: String,
    graph_svg: String,
    loading: bool,
}

pub enum Msg {
    InputChanged(String),
    GetGraph,
    GraphLoaded(String),
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            link: _ctx.link().clone(),
            input_area: String::new(),
            graph_svg: String::new(),
            loading: false,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::InputChanged(value) => {
                self.input_area = value;
            }
            Msg::GetGraph => {
                self.loading = true;
                let area_name = self.input_area.clone();
                let callback = self.link.callback(Msg::GraphLoaded);
                wasm_bindgen_futures::spawn_local(async move {
                    let client = BasicOpenRailwayMapApiClient::new();
                    let api_json_value = client.fetch_by_area_name(&area_name).await.unwrap();
                    let railway_elements = RailwayElement::from_json(&api_json_value).unwrap();
                    let graph = RailwayGraph::from_railway_elements(&railway_elements);
                    let svg_string = generate_svg_string(&graph).unwrap();
                    callback.emit(svg_string.to_string());
                });
            }
            Msg::GraphLoaded(svg_string) => {
                self.loading = false;
                self.graph_svg = svg_string;
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let on_change = self.link.callback(|e: Event| {
            let target: EventTarget = e
                .target()
                .expect("Event should have a target when dispatched");
            let value = target.unchecked_into::<HtmlInputElement>().value();
            Msg::InputChanged(value)
        });
        let svg = Html::from_html_unchecked(self.graph_svg.clone().into());

        let loading_message = if self.loading {
            html! { <p>{ "Loading..." }</p> }
        } else {
            html! {}
        };

        html! {
            <>
                <input
                    value={self.input_area.clone()}
                    onchange={on_change}
                    placeholder="Enter area name"
                />
                <button onclick={self.link.callback(|_| Msg::GetGraph)}>{ "Get Graph" }</button>
                {loading_message}
                <div>{svg}</div>
            </>
        }
    }
}

#[wasm_bindgen]
pub fn init_app(root: web_sys::Element) {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::with_root(root).render();
}
