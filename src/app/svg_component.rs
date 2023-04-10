use crate::prelude::{generate_svg_string, RailwayGraph};
use yew::prelude::*;

#[derive(PartialEq, Properties, Clone)]
pub struct Props {
    pub graph: Option<RailwayGraph>,
}

pub struct SvgComponent {}

impl Component for SvgComponent {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _: &Context<Self>, _: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if let Some(graph) = ctx.props().graph.as_ref() {
            let svg_string = generate_svg_string(&graph).unwrap();
            html! { <div>{ Html::from_html_unchecked(svg_string.into()) }</div> }
        } else {
            html! { <></> }
        }
    }
}
