use yew::prelude::*;

use super::SocialLinks;

#[derive(Clone, Debug)]
pub struct Footer {
    link: ComponentLink<Self>,
}

impl Component for Footer {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <footer class="footer">
            <div class="flex_group">
                <SocialLinks/>
            </div>
            <div class="flex_group">
            <p>{ "Built in WebAssembly using " }
                <a target="_blank" href="https://yew.rs/">{ "yew.rs" }</a>
            </p>
            </div>
        </footer>
                }
    }
}
