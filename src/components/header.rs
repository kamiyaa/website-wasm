use yew::prelude::*;

use crate::data::LOGO_NAME;
use crate::routes::{AppRoute, Link};

#[derive(Clone, Debug)]
pub struct Header {
    link: ComponentLink<Self>,
}

impl Component for Header {
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
        <header class="header">
            <nav class="topnav">
                <div class="logo-link">
                    <Link route={AppRoute::Index}>{LOGO_NAME}</Link>
                </div>
            </nav>
        </header>
                }
    }
}
