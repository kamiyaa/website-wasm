use yew::prelude::*;

pub fn summary() -> Html {
    html! {
    <ul>
    <li>{ "- Gathered, processed and analyzed 10GB+ of historical data on "}
    <a target="_blank" href="https://en.wikipedia.org/wiki/Bitcoin">{ "Bitcoin" }</a>
    { ", resulting in the discovery of several arbitrage opportunities" }</li>
    <li>{ "- Leveraged Python and FSA to develop arbitrage trading algorithms between BTC and QBTC.U, producing up to a $10 profit per trade" }</li>
    <li>{ "- Backtested algorithms and produced data visualization for performance analysis and iterative improvements" }</li>
    </ul>
    }
}

pub fn content() -> Html {
    html! {
    <div>
    <p>{ "Normally I would write a page on what I did, but this time I decided to make a video instead." }
    <br/>
    { "I hope you find it entertaining and educational!" }
    <br/>
    </p>
    <div>
        <iframe width="853" height="480" src="https://www.youtube.com/embed/c1iEh6TlQeQ" frameborder="0" allow="accelerometer; encrypted-media; gyroscope; picture-in-picture; fullscreen"></iframe>
    </div>
    </div>
        }
}
