use yew::prelude::*;

const GITHUB_LINK: &str = "https://github.com/kamiyaa/";
const LINKEDIN_LINK: &str = "https://www.linkedin.com/in/jiayii-zhao/";
const YOUTUBE_LINK: &str = "https://www.youtube.com/channel/UC6NPLaq5poP-CoLo3DPCC8w";
const EMAIL_LINK: &str = "mailto:jeff.no.zhao@gmail.com";
const RESUME_LINK: &str =
    "https://github.com/kamiyaa/kamiyaa.github.io/releases/download/0.1.0/Resume-Jeff-Zhao.pdf";

#[derive(Clone, Debug)]
pub struct SocialLinks {
    link: ComponentLink<Self>,
}

impl Component for SocialLinks {
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
        <div class="social_links">
        <ul>
            <li>
            <a
                class="home-icon" target="_blank" rel="noopener noreferrer"
                href={GITHUB_LINK}><i class="fab fa-github fa-lg"/>
            </a>
            </li>

            <li>
            <a
                class="home-icon" target="_blank" rel="noopener noreferrer"
                href={LINKEDIN_LINK}><i class="fab fa-linkedin fa-lg"/>
            </a>
            </li>

            <li>
            <a
                class="home-icon" target="_blank" rel="noopener noreferrer"
                href={YOUTUBE_LINK}><i class="fab fa-youtube fa-lg"></i>
            </a>
            </li>

            <li>
            <a
                class="home-icon"
                href={EMAIL_LINK}><i class="fa fa-envelope fa-lg"/>
            </a>
            </li>

            <li>
            <a
                class="home-icon"
                href={RESUME_LINK}><i class="fa fa-file fa-lg"/>
            </a>
            </li>
        </ul>
        </div>
                }
    }
}
