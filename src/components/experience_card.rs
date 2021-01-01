use yew::prelude::*;

use crate::data::Experience;
use crate::routes::{AppRoute, Link};

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub experience: &'static Experience,
    pub route: AppRoute,
}

#[derive(Clone, Debug)]
pub struct ExperienceCard {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ExperienceCard {
    type Message = ();
    type Properties = Props;
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let experience = &self.props.experience;

        html! {
        <div class="horizontal_card">
            <div class="horizontal_card_icon">
                <div class="horizontal_card_img_cover">
                    <img alt={experience.company} src={experience.icon_url}/>
                </div>
            </div>
            <div class="horizontal_card_info">
                <h4>{experience.title}</h4>
                <a target="_blank" rel="noopener noreferrer"
                    href={experience.company_url}>
                    <h6>{experience.company}</h6>
                </a>
                <p>{experience.start_date}{" - "}{experience.end_date}</p>
                {(experience.summary)()}
                <Link route={self.props.route.clone()}>{"Read more"}</Link>
            </div>
        </div>
                }
    }
}
