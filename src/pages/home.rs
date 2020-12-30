use yew::prelude::*;

use crate::components::SocialLinks;
use crate::data::{AppRoute, Link, HEADER_LINKS, LOGO_NAME};

pub struct Home;

impl Component for Home {
    type Message = ();
    type Properties = ();
    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
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
                    <li><div class="home-link">
                        <Link route={v.route.clone()}>
                            {"> "}{v.name}
                        </Link>
                    </div></li>
                }
            })
            .collect::<Html>();
        html! {
        <div class="home-content">
            <div class="home-content-section">
            <img
                class="home-profile"
                alt="profile"
                src="https://raw.githubusercontent.com/kamiyaa/kamiyaa.github.io/master/img/profile.jpg"/>
            </div>
            <div class="home-content-section">
                <Link classes="home-logo-link" route={AppRoute::Index}>{LOGO_NAME}</Link>
                <br/>
                <SocialLinks/>
                <ul>{header_links}</ul>
                {about_content()}
            </div>
        </div>
                }
    }
}

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
