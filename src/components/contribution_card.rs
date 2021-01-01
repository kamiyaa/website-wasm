use yew::prelude::*;

use crate::data::Contribution;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub contribution: &'static Contribution,
}

#[derive(Clone, Debug)]
pub struct ContributionCard {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for ContributionCard {
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
        let contrib = &self.props.contribution;
        html! {
        <div class="horizontal_card">
            <div class="horizontal_card_container">
                <div class="horizontal_card_header">
                    <div class="horizontal_card_icon">
                        <a target="_blank" rel="noopener noreferrer" href={contrib.url}>
                            <div class="horizontal_card_img_cover">
                                <img class="horizontal_card_icon" alt={contrib.name} src={contrib.icon_url}/>
                            </div>
                        </a>
                    </div>
                    <div class="horizontal_card_info">
                        <h3 id={contrib.name}>{contrib.name}</h3>
                        <h5>{contrib.languages.join(", ")}</h5>
                        <p>{contrib.description}</p>
                    </div>
                </div>
                <div class="horizontal_card_info">
                    {(contrib.html)()}
                </div>
            </div>
        </div>
                }
    }
}
