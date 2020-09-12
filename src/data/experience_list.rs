use yew::prelude::*;

use super::experiences::{ibm, ops, spiretrading, uoft};

#[derive(Clone, Debug)]
pub struct Experience {
    pub title: &'static str,
    pub company: &'static str,
    pub company_url: &'static str,
    pub start_date: &'static str,
    pub end_date: &'static str,
    pub preview_url: &'static str,
    pub preview_thumbnail_url: &'static str,
    pub html: fn() -> Html,
    pub tags: &'static [&'static str],
}

pub static EXPERIENCE_LIST: [Experience; 3] = [
    Experience {
        title: "Open Source Runtime Developer",
        company: "IBM",
        company_url: "https://www.ibm.com/",
        start_date: "May 2019",
        end_date: "April 2020",
        preview_url: "https://github.com/kamiyaa/kamiyaa.github.io/raw/master/img/IBM.jpg",
        preview_thumbnail_url:
            "https://github.com/kamiyaa/kamiyaa.github.io/raw/master/img/IBM.jpg",
        html: ibm::component,
        tags: &[
            "C", "Java", "Docker", "Jenkins", "Appsody", "Maven", "Capstone", "Linux", "AIX",
            "z/OS", "PowerPC", "Systemz",
        ],
    },
    Experience {
        title: "Teaching Assistant",
        company: "University of Toronto",
        company_url: "https://www.utoronto.ca/",
        start_date: "September 2017",
        end_date: "December 2019",
        preview_url: "https://github.com/kamiyaa/kamiyaa.github.io/raw/master/img/UofT_002.jpg",
        preview_thumbnail_url:
            "https://github.com/kamiyaa/kamiyaa.github.io/raw/master/img/UofT_002.jpg",
        html: uoft::component,
        tags: &[
            "C",
            "Python",
            "Java",
            "Shell",
            "SVN",
            "Design Patterns",
            "OOP/SOLID design",
            "Pipes/Sockets",
        ],
    },
    Experience {
        title: "Application Programmer",
        company: "Ontario Public Service",
        company_url: "https://www.ontario.ca/",
        start_date: "May 2018",
        end_date: "August 2018",
        preview_url: "https://github.com/kamiyaa/kamiyaa.github.io/raw/master/img/OPS_002.png",
        preview_thumbnail_url:
            "https://github.com/kamiyaa/kamiyaa.github.io/raw/master/img/OPS_002.png",
        html: ops::component,
        tags: &[
            "Java",
            "JasperReports",
            "Vbscript",
            "HP Quality Center",
            "HP UFT",
        ],
    },
];
