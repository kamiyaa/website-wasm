use yew::prelude::*;

pub fn summary() -> Html {
    html! {
    <div>
    <br/>
    </div>
    }
}

pub fn content() -> Html {
    html! {
    <div>
    <p>{ "At Ontario Public Service (OPS), I was working on the Case and Grant Management Systems team. This team is responsible for managing investigation cases by the government as well as grants for public services." }
    <br/>
    { "I worked on updating their test suites to correspond with new releases." }
    </p>
    <br/>
    <h5>{ "JasperReports Proof-of-Concept" }</h5>
    <p>{ "One of the key components of the grants management system is the dynamic generation of PDF documents. Currently, OPS uses Adobe LiveCycle to create workflows for generating contracts, reports and other important documents." }
    <br/>{ "I was tasked to explore alternatives for replacing this component. I created a JasperReports proof-of-concept to replace Adobe LiveCycle's PDF generation." }
    </p>
    </div>
        }
}
