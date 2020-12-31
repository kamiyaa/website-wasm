use yew::prelude::*;

use crate::components::{Footer, HomeHeader};
use crate::data::LOGO_NAME;
use crate::data::{CONTRIBUTION_LIST, EXPERIENCE_LIST, PROJECT_LIST};
use crate::routes::{AppRoute, Link};

use crate::components::{ContributionList, ExperienceList, ProjectList};

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
        <div class="home">
            <HomeHeader/>
            <div class="home-section">
                <div style="width: 100%;">
                    <h2 id="experiences">{"Experiences"}</h2>
                    <div>
                    <ExperienceList list={&EXPERIENCE_LIST}/>
                    </div>
                </div>
            </div>
            <div class="home-section">
                <div style="width: 100%;">
                    <h2 id="projects">{"Projects"}</h2>
                    <div>
                    <ProjectList list={&PROJECT_LIST}/>
                    </div>
                </div>
            </div>
            <div class="home-section">
                <div style="width: 100%;">
                    <h2 id="contributions">{"Contributions"}</h2>
                    <div>
                    <ContributionList list={&CONTRIBUTION_LIST}/>
                    </div>
                </div>
            </div>
            <br/>
            <br/>
            <Footer/>
        </div>
                }
    }
}
