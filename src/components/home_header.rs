use web_sys::Element;
use yew::prelude::*;

use crate::components::SocialLinks;
use crate::data::{HEADER_LINKS, LOGO_NAME};
use crate::routes::{AppRoute, Link};

fn about_content() -> Html {
    html! {
    <div>
        <p>{ "Hi, my name is Jeff Zhao." }
        <br/>
        { "I'm currently a 4th year student studying Computer Science at the "}<b>{ "University of Toronto" }</b>{ "." }
        <br/>
        { "I enjoy working with open source software, learning new technologies" }
        <br/>
        { "and coming up with innovative ways to solve problems." }
        <br/>
        </p>
        <p>{ "Feel free to look around!" }
        </p>
    </div>
        }
}

#[derive(Clone, Debug)]
pub struct HomeHeader {
    link: ComponentLink<Self>,
}

impl Component for HomeHeader {
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
        let header_links = HEADER_LINKS
            .iter()
            .map(|v| {
                html! {
                    <li><div class="header-link">
                        <a href={v.route}>
                        {v.name}
                        </a>
                    </div></li>
                }
            })
            .collect::<Html>();

        html! {
        <header class="home-header">
            <div class="home-header-section">
            <img
                class="home-profile"
                alt="profile"
                src="https://github.com/kamiyaa/kamiyaa.github.io/releases/download/0.1.0/IMG_20200825_162748_1.jpg"/>
            </div>
            <div class="home-header-section">
                <div class="logo-link">
                    <Link route={AppRoute::Index}>{LOGO_NAME}</Link>
                </div>
                {about_content()}
                <ul>{header_links}</ul>
                <SocialLinks/>
            </div>
        </header>
                }
    }
}
