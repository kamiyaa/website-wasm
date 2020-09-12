use yew::prelude::*;

#[derive(Clone, Debug, Properties)]
pub struct Props {
    pub children: Children,
}

#[derive(Clone, Debug)]
pub struct MenuStrip {
    props: Props,
    link: ComponentLink<Self>,
}

impl Component for MenuStrip {
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
        html! {
            <div class="article-topbar">
                <div style="display: flex;">
                { self.props.children.clone() }
                </div>
            </div>
        }
    }
}
