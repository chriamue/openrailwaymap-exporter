use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use web_sys::{Document, HtmlCanvasElement, Window};
use yew::prelude::*;

pub struct Kiss3dComponent {
}

pub enum Msg {
}

impl Component for Kiss3dComponent {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Kiss3dComponent {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {

        html! {
            <>
                <canvas id="canvas" width="800" height="600"></canvas>
            </>
        }
    }
}