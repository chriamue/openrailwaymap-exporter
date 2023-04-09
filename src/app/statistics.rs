use yew::{html, Component, Context, Html, Properties};

pub struct Statistics {}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub switches: u32,
    pub tracks: u32,
    pub total_length: f64,
}

impl Component for Statistics {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="statistics">
                <h2>{ "Statistics" }</h2>
                <p>{ format!("Switches: {}", ctx.props().switches) }</p>
                <p>{ format!("Tracks: {}", ctx.props().tracks) }</p>
                <p>{ format!("Total Length: {:.0} m", ctx.props().total_length) }</p>
            </div>
        }
    }
}
