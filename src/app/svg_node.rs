use crate::prelude::RailwayNode;
use yew::prelude::*;

/// A Yew component for visualizing railway nodes as SVG circles.
pub struct SvgNode {
    hovered: bool,
    client_x: i32,
    client_y: i32,
    clicked: bool,
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
    pub min_coord: (f64, f64),
    /// Callback for when the circle is clicked.
    pub on_select: Option<Callback<i64>>,
}

/// Messages for the `SvgNode` component.
pub enum Msg {
    MouseEnter,
    MouseLeave,
    Click(MouseEvent),
}

impl Component for SvgNode {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        SvgNode {
            hovered: false,
            client_x: 0,
            client_y: 0,
            clicked: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MouseEnter => {
                self.hovered = true;
            }
            Msg::MouseLeave => {
                self.hovered = false;
            }
            Msg::Click(event) => {
                self.client_x = event.client_x();
                self.client_y = event.client_y();
                self.clicked = !self.clicked;
                if let Some(on_select) = &ctx.props().on_select {
                    let node_index = ctx.props().node.id;
                    on_select.emit(node_index);
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let radius = if self.hovered { 5.0 } else { 2.0 };
        let x = (ctx.props().node.lon - ctx.props().min_coord.0) * ctx.props().scale_x;
        let y = ctx.props().view_height
            - (ctx.props().node.lat - ctx.props().min_coord.1) * ctx.props().scale_y;

        let color = if self.clicked { "blue" } else { "red" };

        html! {
            <circle
                cx={format!("{}",x)}
                cy={format!("{}", y)}
                r={format!("{}", radius)}
                fill={color}
                onmouseover={ctx.link().callback(|_| Msg::MouseEnter)}
                onmouseout={ctx.link().callback(|_| Msg::MouseLeave)}
                onclick={ctx.link().callback(|event: MouseEvent| Msg::Click(event))}
            />
        }
    }
}
