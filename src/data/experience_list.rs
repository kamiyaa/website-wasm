use yew::prelude::*;

use super::experiences::{ibm, ops, spiretrading, uoft};

#[derive(Clone, Debug)]
pub struct Experience {
    pub title: &'static str,
    pub company: &'static str,
    pub company_url: &'static str,
    pub start_date: &'static str,
    pub end_date: &'static str,
    pub icon_url: &'static str,
    pub summary: fn() -> Html,
    pub content: fn() -> Html,
    pub tags: &'static [&'static str],
}

pub type ExperienceListType = [Experience; 4];

pub static EXPERIENCE_LIST: ExperienceListType = [
    Experience {
        title: "Quantitative Software Developer",
        company: "SpireTrading",
        company_url: "https://spiretrading.com/",
        start_date: "May 2020",
        end_date: "Aug 2020",
        icon_url: "https://github.com/kamiyaa/kamiyaa.github.io/releases/download/0.1.0/spiretrading_icon.png",
        summary: spiretrading::summary,
        content: spiretrading::content,
        tags: &[
            "C", "Java", "Docker", "Jenkins", "Appsody", "Maven", "Capstone", "Linux", "AIX",
            "z/OS", "PowerPC", "Systemz",
        ],
    },
    Experience {
        title: "Open Source Runtime Developer",
        company: "IBM",
        company_url: "https://www.ibm.com/",
        start_date: "May 2019",
        end_date: "Apr 2020",
        icon_url: "https://github.com/kamiyaa/kamiyaa.github.io/releases/download/0.1.0/ibm_icon.png",
        summary: ibm::summary,
        content: ibm::content,
        tags: &[
            "C", "Java", "Docker", "Jenkins", "Appsody", "Maven", "Capstone", "Linux", "AIX",
            "z/OS", "PowerPC", "Systemz",
        ],
    },
    Experience {
        title: "Teaching Assistant",
        company: "University of Toronto",
        company_url: "https://www.utoronto.ca/",
        start_date: "Sept 2017",
        end_date: "Dec 2019",
        icon_url: "https://github.com/kamiyaa/kamiyaa.github.io/releases/download/0.1.0/uoft_icon.png",
        summary: uoft::summary,
        content: uoft::content,
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
        end_date: "Aug 2018",
        icon_url: "https://github.com/kamiyaa/kamiyaa.github.io/releases/download/0.1.0/ontario_icon.jpg",
        summary: ops::summary,
        content: ops::content,
        tags: &[
            "Java",
            "JasperReports",
            "Vbscript",
            "HP Quality Center",
            "HP UFT",
        ],
    },
];
