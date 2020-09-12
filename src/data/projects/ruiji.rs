use yew::prelude::*;

pub fn component() -> Html {
    html! {
    <div>
    <p>{ "This project was initially for my own personal use, but I thought some people might find it useful as well." }
    </p>
    <br/>
    <h4>{ "Motivation" }</h4>
    <p>{ "Many people like myself often have tons of pictures saved on our computers. And very often, they are low resolution images saved off of social media. This program allows users to process large collection of images and look online for similar images, usually of higher resolution." }
    <br/>
    { "From this project, I was able to apply the C knowledge I learned in school and also learn about build systems used by the industry. I implemented:" }
    </p>

    <ul>
    <li>{ "linked lists for storing results" }</li>
    <li>{ "resizable arrays to hold html content until the entire transaction is over" }</li>
    </ul>

    <p>{ "I utilized " }
        <a href="https://curl.haxx.se/">{ "libcurl" }</a>
    { " to make network requests to " }
        <a href="http://iqdb.org/">{ "iqdb.org" }</a>
    {". The returned HTML is parsed as a string to find all the necessary information to populate results for the user to pick from. Once the user has chosen a source to download from, site-specific code is run to find the source link on that website and the image is downloaded." }
    <br/>
    { "For building the project, I used both Makefiles as well as the new build system " }
    <a href="https://mesonbuild.com/">{ "Meson" }</a>{ "." }
    </p>
    </div>
        }
}
