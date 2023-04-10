use crate::prelude::overpass_api_client::Coordinate;
use crate::prelude::RailwayNode;
use yew::prelude::*;

/// A Yew component for visualizing railway nodes as SVG circles.
pub struct SvgNode {
    hovered: bool,
}

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    /// The railway node to visualize.
    pub node: RailwayNode,
    /// The scaling factor for the x-axis.
    pub scale_x: f64,
    /// The scaling factor for the y-axis.
    pub scale_y: f64,
    /// The height of the SVG view.
    pub view_height: f64,
    /// The minimum coordinates of the graph's bounding box.
    pub min_coord: Coordinate,
}

/// Messages for the `SvgNode` component.
pub enum Msg {
    MouseEnter,
    MouseLeave,
}

impl Component for SvgNode {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        SvgNode { hovered: false }
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
        let radius = if self.hovered { 5.0 } else { 2.0 };
        let x = (ctx.props().node.lon - ctx.props().min_coord.lon) * ctx.props().scale_x;
        let y = ctx.props().view_height
            - (ctx.props().node.lat - ctx.props().min_coord.lat) * ctx.props().scale_y;

        html! {
            <circle
                cx={format!("{}",x)}
                cy={format!("{}", y)}
                r={format!("{}", radius)}
                fill={"red"}
                onmouseover={ctx.link().callback(|_| Msg::MouseEnter)}
                onmouseout={ctx.link().callback(|_| Msg::MouseLeave)}
            />
        }
    }
}
