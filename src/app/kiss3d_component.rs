use crate::prelude::RailwayGraph;
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point3, Vector3};
use kiss3d::scene::SceneNode;
use kiss3d::window::State;
use kiss3d::window::Window;
use std::borrow::BorrowMut;
use std::cell::RefCell;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

pub struct Kiss3dComponent {}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub graph: Option<RailwayGraph>,
}

#[derive(Clone)]
struct AppState {
    scene: SceneNode,
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {}
}

pub enum Msg {}

impl Component for Kiss3dComponent {
    type Message = Msg;
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, _: &Context<Self>) -> Html {
        html! {
            <canvas id="canvas" width="800" height="600"></canvas>
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            let mut window = Window::new("Kiss3d + Yew");
            window.set_light(Light::StickToCamera);
            let mut cube = window.add_cube(1.0, 1.0, 1.0);
            cube.set_color(0.0, 0.0, 1.0);
            let state = AppState { scene: cube };
            window.render_loop(state);
        }
    }
}
