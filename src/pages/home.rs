use yew::prelude::*;

use crate::components::{ContributionList, ExperienceList, Footer, HomeHeader, ProjectList};
use crate::data::{CONTRIBUTION_LIST, EXPERIENCE_LIST, PROJECT_LIST};

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
        html! {
        <div class="base">
            <HomeHeader/>
            <div class="home-section">
                <div style="width: 100%;">
                    <a class="link" href="/#experiences">
                        <h2 id="experiences">{"Experiences"}</h2>
                    </a>
                    <div>
                    <ExperienceList list={&EXPERIENCE_LIST}/>
                    </div>
                </div>
            </div>
            <div class="home-section">
                <div style="width: 100%;">
                    <a class="link" href="/#projects">
                        <h2 id="projects">{"Projects"}</h2>
                    </a>
                    <div>
                    <ProjectList list={&PROJECT_LIST}/>
                    </div>
                </div>
            </div>
            <div class="home-section">
                <div style="width: 100%;">
                    <a class="link" href="/#contributions">
                        <h2 id="contributions">{"Contributions"}</h2>
                    </a>
                    <div>
                    <ContributionList list={&CONTRIBUTION_LIST}/>
                    </div>
                </div>
            </div>
            <Footer/>
        </div>
                }
    }
}
