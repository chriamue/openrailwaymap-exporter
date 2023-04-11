use crate::prelude::overpass_api_client::Coordinate;
use crate::prelude::RailwayEdge;
use yew::prelude::*;

/// A Yew component for visualizing railway edges as SVG paths.
pub struct SvgEdge {
    hovered: bool,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    /// The railway edge to visualize.
    pub edge: RailwayEdge,
    /// The scaling factor for the x-axis.
    pub scale_x: f64,
    /// The scaling factor for the y-axis.
    pub scale_y: f64,
    /// The height of the SVG view.
    pub view_height: f64,
    /// The minimum coordinates of the graph's bounding box.
    pub min_coord: Coordinate,
    pub stroke_color: Option<String>,
}

/// Messages for the `SvgEdge` component.
pub enum Msg {
    MouseEnter,
    MouseLeave,
}

impl Component for SvgEdge {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        SvgEdge { hovered: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MouseEnter => {
                self.hovered = true;
            }
            Msg::MouseLeave => {
                self.hovered = false;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let edge_data = &ctx.props().edge;
        let path_data: String = edge_data
            .path
            .0
            .iter()
            .enumerate()
            .map(|(i, coord)| {
                let x = (coord.x - ctx.props().min_coord.lon) * ctx.props().scale_x;
                let y = ctx.props().view_height
                    - (coord.y - ctx.props().min_coord.lat) * ctx.props().scale_y;

                if i == 0 {
                    format!("M {:.1} {:.1}", x, y)
                } else {
                    format!(" L {:.1} {:.1}", x, y)
                }
            })
            .collect::<Vec<String>>()
            .join("");

        let stroke_width = if self.hovered { 2 } else { 1 };

        html! {
            <path
                d={path_data}
                stroke={ctx.props().stroke_color.as_ref().unwrap_or(&"black".to_string()).to_string()}
                stroke-width={format!("{}", stroke_width)}
                fill="none"
                onmouseover={ctx.link().callback(|_| Msg::MouseEnter)}
                onmouseout={ctx.link().callback(|_| Msg::MouseLeave)}
            />
        }
    }
}
