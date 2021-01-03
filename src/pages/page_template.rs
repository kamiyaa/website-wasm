use web_sys::{ScrollBehavior, ScrollToOptions};
use yew::prelude::*;

use crate::components::{Footer, Header};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub children: Children,
}

#[derive(Clone, Debug)]
pub struct PageTemplate {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for PageTemplate {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        if let Some(window) = web_sys::window() {
            window.scroll_with_scroll_to_options(
                ScrollToOptions::new()
                    .behavior(ScrollBehavior::Smooth)
                    .top(0.0),
            );
        }
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
        <div class="base">
        <Header/>
            { self.props.children.clone() }
        <Footer/>
        </div>
                }
    }
}
