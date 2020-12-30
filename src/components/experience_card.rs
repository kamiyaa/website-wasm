use yew::prelude::*;

use crate::data::{AppRoute, Link, Experience};

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
        <div class="flex_card">
            <Link route={self.props.route.clone()}>
                <div class="flex_card_img_container">
                         <img alt={experience.company} src={experience.preview_thumbnail_url}/>
                </div>
                <div class="flex_card_info">
                    <h4>{experience.title}</h4>
                    <h6>{experience.company}</h6>
                    <p>{experience.start_date}{" - "}{experience.end_date}</p>
                </div>
            </Link>
        </div>
                }
    }
}
