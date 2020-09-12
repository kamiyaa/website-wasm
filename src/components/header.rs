use web_sys::Element;
use yew::prelude::*;

use crate::data::{AppRoute, AppRouteAnchor, HEADER_LINKS, LOGO_NAME};

#[derive(Clone, Debug)]
pub struct Header {
    menu_ref: NodeRef,
    menu_btn_ref: NodeRef,
    link: ComponentLink<Self>,
}

impl Component for Header {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            menu_ref: NodeRef::default(),
            menu_btn_ref: NodeRef::default(),
            link,
        }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        if let Some(element) = self.menu_ref.cast::<Element>() {
            let class_list = element.class_list();
            class_list.toggle("is-open");
        }
        if let Some(element) = self.menu_btn_ref.cast::<Element>() {
            let class_list = element.class_list();
            class_list.toggle("is-open");
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let hamburger_btn = html! {
            <button
                class="hamburger-button"
                ref={self.menu_btn_ref.clone()}
                onclick={self.link.callback(|_| ())}
                >
                <div class="bar1"></div>
                <div class="bar2"></div>
                <div class="bar3"></div>
            </button>
        };
        let non_hamburger_menu = html! {
            <div class="non-hamburger-menu">
            {
                HEADER_LINKS.iter().map(|v| html!{
                    <li><div class="header-link">
                        <AppRouteAnchor route={v.route.clone()}>
                        {v.name}
                        </AppRouteAnchor>
                    </div></li>
                }).collect::<Html>()
            }
            </div>
        };
        let hamburger_menu = html! {
            <div ref={self.menu_ref.clone()} class="hamburger-menu">
                <ul>
                {
                    HEADER_LINKS.iter().map(|v| html!{
                        <li><AppRouteAnchor classes="logo-link-nth"
                            route={v.route.clone()}>
                            {v.name}
                            </AppRouteAnchor>
                        </li>
                    }).collect::<Html>()
                }
                </ul>
            </div>
        };

        html! {
        <div class="header">
            {hamburger_menu}
            <nav class="topnav">
                <div class="flex_group">
                    <div class="logo-link">
                    <AppRouteAnchor route={AppRoute::Index}>{LOGO_NAME}</AppRouteAnchor>
                    </div>
                    {non_hamburger_menu}
                </div>
                <div class="flex_group">{hamburger_btn}</div>
            </nav>
        </div>
                }
    }
}
