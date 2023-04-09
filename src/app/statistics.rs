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

#[cfg(test)]
mod tests {
    use super::*;

    #[wasm_bindgen_test::wasm_bindgen_test]
    async fn test_render() {
        let props = Props {
            switches: 5,
            tracks: 10,
            total_length: 1500.0,
        };
        let rendered = yew::LocalServerRenderer::<Statistics>::with_props(props)
            .render()
            .await;

        assert!(rendered.contains("<h2>Statistics</h2>"));
        assert!(rendered.contains("Switches: 5"));
        assert!(rendered.contains("Tracks: 10"));
        assert!(rendered.contains("Total Length: 1500 m"));
    }
}
