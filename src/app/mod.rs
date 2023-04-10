//! The `App` module contains the main application component and its associated messages.
//!
//! The `App` component is responsible for managing the state of the application, handling user input, and
//! rendering the UI. The component uses the `RailwayApiClient` to fetch data from the Overpass API, and it
//! displays the railway graph in either an SVG or a 3D view using the `Kiss3dComponent`.
//!
//! The `Msg` enum represents the different messages that can be sent to the `App` component to trigger
//! state updates and UI changes.

use crate::prelude::from_railway_elements;
use crate::prelude::overpass_api_client::{
    count_node_elements, count_way_elements, RailwayElement,
};
use crate::prelude::{OverpassApiClient, RailwayApiClient, RailwayGraph};
use wasm_bindgen::prelude::*;
use web_sys::EventTarget;
use web_sys::HtmlInputElement;
use yew::html::Scope;
use yew::prelude::*;

mod kiss3d_component;
use kiss3d_component::Kiss3dComponent;

mod svg_component;
use svg_component::SvgComponent;

mod statistics;
use statistics::Statistics;

/// Represents the main application component.
pub struct App {
    link: Scope<Self>,
    input_area: String,
    loading: bool,
    switch_count: u32,
    track_count: u32,
    total_length: f64,
    show_svg: bool,
    graph: Option<RailwayGraph>,
}

/// Represents the messages that can be sent to the `App` component.
pub enum Msg {
    /// Input changed.
    InputChanged(String),
    /// Button clicked.
    GetGraph,
    /// Update Graph with loaded data.
    GraphLoaded((Vec<RailwayElement>, RailwayGraph)),
    /// Toggle between svg and 3d.
    ToggleView,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        App {
            link: _ctx.link().clone(),
            input_area: String::new(),
            loading: false,
            switch_count: 0,
            track_count: 0,
            total_length: 0.0,
            show_svg: true,
            graph: None,
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
                    callback.emit((railway_elements, graph));
                });
            }
            Msg::GraphLoaded((railway_elements, graph)) => {
                self.switch_count = count_node_elements(&railway_elements) as u32;
                self.track_count = count_way_elements(&railway_elements) as u32;
                self.total_length = graph.total_length();
                self.loading = false;
                self.graph = Some(graph);
            }
            Msg::ToggleView => {
                self.show_svg = !self.show_svg;
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

        let on_toggle_view = self.link.callback(|_| Msg::ToggleView);

        let view_content = if self.show_svg {
            html! {
                <SvgComponent graph={self.graph.clone()} />
            }
        } else {
            html! {
                <Kiss3dComponent graph={self.graph.clone()} />
            }
        };

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
                    <div>
                    <button onclick={on_toggle_view}>
                        { if self.show_svg { "Show 3D View" } else { "Show SVG" } }
                    </button>
                </div>
                </div>
                <Statistics switches={self.switch_count} tracks={self.track_count} total_length={self.total_length} />
                {loading_message}
                { view_content }
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

#[cfg(test)]
mod tests {
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}
