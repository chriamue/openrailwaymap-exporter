use crate::prelude::overpass_api_client::{
    count_node_elements, count_way_elements, RailwayElement,
};
use crate::prelude::{from_railway_elements, generate_svg_string};
use crate::prelude::{OverpassApiClient, RailwayApiClient, RailwayGraph};
use wasm_bindgen::prelude::*;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::html::Scope;
use yew::prelude::*;

mod statistics;
use statistics::Statistics;

/// Represents the main application component.
pub struct App {
    link: Scope<Self>,
    input_area: String,
    graph_svg: String,
    loading: bool,
    switch_count: u32,
    track_count: u32,
    total_length: f64,
}

/// Represents the messages that can be sent to the `App` component.
pub enum Msg {
    /// Input changed.
    InputChanged(String),
    /// Button clicked.
    GetGraph,
    /// Update Graph with loaded data.
    GraphLoaded((Vec<RailwayElement>, RailwayGraph, String)),
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
            switch_count: 0,
            track_count: 0,
            total_length: 0.0,
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
                    let client = OverpassApiClient::new();

                    let api_json_value = {
                        if area_name.contains(",") {
                            client
                                .fetch_by_bbox(&area_name)
                                .await
                                .unwrap_or(client.fetch_by_area_name(&area_name).await.unwrap())
                        } else {
                            client.fetch_by_area_name(&area_name).await.unwrap()
                        }
                    };

                    let railway_elements = RailwayElement::from_json(&api_json_value).unwrap();
                    let graph = from_railway_elements(&railway_elements);
                    let svg_string = generate_svg_string(&graph).unwrap();
                    callback.emit((railway_elements, graph, svg_string.to_string()));
                });
            }
            Msg::GraphLoaded((railway_elements, graph, svg_string)) => {
                self.switch_count = count_node_elements(&railway_elements) as u32;
                self.track_count = count_way_elements(&railway_elements) as u32;
                self.total_length = graph.total_length();
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
                <div class="controls">
                    <input
                        value={self.input_area.clone()}
                        onchange={on_change}
                        placeholder="Enter area name"
                    />
                    <button onclick={self.link.callback(|_| Msg::GetGraph)}>{ "Get Graph" }</button>
                </div>
                <Statistics switches={self.switch_count} tracks={self.track_count} total_length={self.total_length} />
                {loading_message}
                <div>{svg}</div>
            </>
        }
    }
}

/// Initializes the main application component and renders it in the given root element.
///
/// This function is meant to be called from JavaScript via WebAssembly to initialize and render
/// the main `App` component inside the specified root element. It sets up the panic hook for
/// better error messages and uses Yew's renderer to attach the `App` component to the DOM.
///
/// # Arguments
///
/// * `root` - A `web_sys::Element` representing the root element where the `App` component will be rendered.
///
/// # Example
///
/// In an HTML file:
///
/// ```html
/// <body>
///     <div id="app" />
///     <script type="module">
///       import init, { init_app } from "./pkg/openrailwaymap_exporter.js";
///       var root = document.getElementById("app");
///       init().then(async () => {
///         try {
///           init_app(root);
///         } catch (e) {
///           console.error(e);
///         }
///       });
///     </script>
///   </body>
/// ```
#[wasm_bindgen]
pub fn init_app(root: web_sys::Element) {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::with_root(root).render();
}
