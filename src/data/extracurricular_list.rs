use yew::prelude::*;

#[derive(Clone, Debug)]
pub struct Extracurricular {
    pub name: &'static str,
    pub title: &'static str,
    pub url: &'static str,
    pub date: &'static str,
    pub html: fn() -> Html,
}

pub type ExtracurricularListType = [Extracurricular; 4];

pub static EXTRACURRICULAR_LIST: ExtracurricularListType = [
    Extracurricular {
        name: "Developer Student Club",
        title: "VP Marketing",
        url: "https://utsc.developerstudentclubs.ca/",
        date: "August 2019 - May 2020",
        html: || {
            html! {
            <div>
            <p>{ "Developer Student Clubs are university based community groups for students interested in Google developer technologies." }
            <br/>
            { "As the VP of Marketing, I was responsible for writing and promoting events. Promoting includes writing emails, Facebook posts and other forms of copy." }
            </p>
            </div>
                    }
        },
    },
    Extracurricular {
        name: "UofT Hacks",
        title: "Participant",
        url: "https://uofthacks.com/",
        date: "January 2018",
        html: || {
            html! {
            <p>{ "Participated in a group of two with a grad student. We created a Django web app that aggregates large, existing open datasets and tries to do predictions on illnesses and diseases." }
            </p>
                    }
        },
    },
    Extracurricular {
        name: "Microsoft msft Coding Challenge",
        title: "1st Place",
        url: "https://www.microsoft.com/en-ca/",
        date: "January 2017",
        html: || {
            html! {
            <p>{ "Working in a team of three with two other first years, we tackled a series of coding problems presented by Microsoft." }
            </p>
                    }
        },
    },
    Extracurricular {
        name: "Chinese Lion Dancing",
        title: "Performer and student",
        url: "http://www.sammychengtorontoliondance.com/",
        date: "2012 - Present",
        html: || {
            html! {
            <div>
            <p>{ "Worked together in teams to deliver spectacular performances for many events such as parades, weddings, grand openings, new years, etc." }
            <br/>
            { "Some of the roles I've performed are lion head/tail, dragon head, and various martial arts." }
            <br/>
            </p>
            </div>
                    }
        },
    },
];
