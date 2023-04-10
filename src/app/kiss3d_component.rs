use crate::prelude::RailwayGraph;
use crate::prelude::RailwayNode;
use instant::Instant;
use kiss3d::camera::{ArcBall, Camera};
use kiss3d::event::{Action, Key, Modifiers, WindowEvent};
use kiss3d::light::Light;
use kiss3d::nalgebra::{Point2, Point3};
use kiss3d::planar_camera::PlanarCamera;
use kiss3d::post_processing::PostProcessingEffect;
use kiss3d::renderer::Renderer;
use kiss3d::text::Font;
use kiss3d::window::{State, Window};
use petgraph::visit::{EdgeRef, IntoNodeReferences, NodeRef};
use std::rc::Rc;
use yew::prelude::*;

pub struct Kiss3dComponent {}

#[derive(PartialEq, Properties)]
pub struct Props {
    pub graph: Option<RailwayGraph>,
}

struct AppState {
    graph: RailwayGraph,
    x_scale: f32,
    y_scale: f32,
    camera: Box<ArcBall>,
    font: Rc<Font>,
    current_node_index: usize,
    total_nodes: usize,
    last_frame_time: Instant,
}

impl AppState {
    fn get_3d_coordinates(&self, coord: &RailwayNode) -> Point3<f32> {
        Point3::new(
            coord.lon as f32 * self.x_scale,
            coord.lat as f32 * self.y_scale,
            0.0,
        )
    }

    fn add_nodes(&mut self, window: &mut Window) {
        for node in self.graph.graph.node_references() {
            let position = self.get_3d_coordinates(node.weight());

            let mut sphere = window.add_sphere(2.0);
            sphere.set_color(0.0, 1.0, 0.0);
            sphere.set_local_translation(position.into());
        }
    }

    fn add_lines(&mut self, window: &mut Window) {
        for edge in self.graph.graph.edge_references() {
            let start_node = self.graph.graph.node_weight(edge.source()).unwrap();
            let end_node = self.graph.graph.node_weight(edge.target()).unwrap();

            let start = self.get_3d_coordinates(start_node);
            let end = self.get_3d_coordinates(end_node);
            window.set_line_width(2.0);
            window.draw_line(&start, &end, &Point3::new(1.0, 0.0, 1.0));
        }
    }

    fn add_node_count_text(&mut self, window: &mut Window) {
        let text = format!("Nodes: {}", self.graph.graph.node_count());
        window.draw_text(
            &text,
            &Point2::new(0.0, 25.0),
            30.0,
            &self.font,
            &Point3::new(1.0, 1.0, 1.0),
        );
    }

    fn add_fps_text(&mut self, window: &mut Window) {
        let now = Instant::now();
        let time_delta = now - self.last_frame_time;
        self.last_frame_time = now;

        let fps = 1.0 / time_delta.as_secs_f32();
        let fps_text = format!("{:.2} FPS", fps);
        window.draw_text(
            &fps_text,
            &Point2::new(5.0, 5.0),
            30.0,
            &self.font,
            &Point3::new(1.0, 1.0, 1.0),
        );
    }

    fn look_at_node(&mut self, node_index: usize) {
        if let Some(node) = self.graph.graph.node_references().nth(node_index) {
            let node_data = node.weight();

            // Update the camera target to look at the selected node
            let target_position = self.get_3d_coordinates(&node_data);
            self.camera.as_mut().set_at(target_position);
        }
    }
}

impl State for AppState {
    fn step(&mut self, window: &mut Window) {
        self.add_nodes(window);
        self.add_lines(window);
        self.add_node_count_text(window);
        self.add_fps_text(window);

        for event in window.events().iter() {
            match event.value {
                WindowEvent::Key(Key::N, Action::Release, _) => {
                    self.current_node_index = (self.current_node_index + 1) % self.total_nodes;

                    self.look_at_node(self.current_node_index);
                }
                _ => {}
            }
        }
    }

    fn cameras_and_effect_and_renderer(
        &mut self,
    ) -> (
        Option<&mut dyn Camera>,
        Option<&mut dyn PlanarCamera>,
        Option<&mut dyn Renderer>,
        Option<&mut dyn PostProcessingEffect>,
    ) {
        (Some(&mut *self.camera), None, None, None)
    }
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

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render {
            if let Some(graph) = ctx.props().graph.clone() {
                let mut window = Window::new("Kiss3d + Yew");

                window.set_light(Light::StickToCamera);
                let (min_coord, max_coord) = graph.bounding_box();
                let width = 1000.0;
                let height = 1000.0;

                let x_scale = width / (max_coord.lon - min_coord.lon) as f32;
                let y_scale = height / (max_coord.lat - min_coord.lat) as f32;

                let scene_center_x = (max_coord.lon + min_coord.lon) as f32 * x_scale / 2.0;
                let scene_center_y = (max_coord.lat + min_coord.lat) as f32 * y_scale / 2.0;
                let scene_center_z = 0.0;

                let target_pos = Point3::new(scene_center_x, scene_center_y, scene_center_z);

                // Calculate the camera position based on the scene's dimensions
                let camera_distance = 0.5 * f32::max(width, height);
                let camera_pos = Point3::new(
                    scene_center_x,
                    scene_center_y - camera_distance,
                    camera_distance,
                );

                let mut camera = ArcBall::new(camera_pos, target_pos);
                camera.set_drag_modifiers(Some(Modifiers::Shift));

                let total_nodes = graph.graph.node_count();

                let mut state = AppState {
                    graph,
                    x_scale,
                    y_scale,
                    camera: Box::new(camera),
                    font: Font::default(),
                    current_node_index: 0,
                    total_nodes,
                    last_frame_time: Instant::now(),
                };
                state.look_at_node(0);
                window.render_loop(state);
            }
        }
    }
}
