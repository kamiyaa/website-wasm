use yew::prelude::*;

#[derive(Clone, Debug)]
pub struct Contribution {
    pub name: &'static str,
    pub url: &'static str,
    pub icon_url: &'static str,
    pub description: &'static str,
    pub languages: &'static [&'static str],
    pub html: fn() -> Html,
}

pub type ContributionListType = [Contribution; 4];

pub static CONTRIBUTION_LIST: ContributionListType = [
    Contribution {
        name: "Capstone",
        url: "https://github.com/aquynh/capstone",
        icon_url: "https://github.com/kamiyaa/kamiyaa.github.io/releases/download/0.1.0/capstone_icon.png",
        description: "Multi-platform, multi-architecture disassembly framework",
        languages: &["C"],
        html: || {
            html! {
            <ul id="capstone">
                <li>{ "- Add build support for IBM platforms such as AIX and zOS" }</li>
                <li>{ "- Fix zOS instruction disassembly for instructions where base is 0 but index is not" }</li>
            </ul>
                    }
        },
    },
    Contribution {
        name: "Portage",
        url: "https://github.com/gentoo/gentoo",
        icon_url: "https://upload.wikimedia.org/wikipedia/commons/4/41/Gentoo-logo-dark.svg",
        description: "Official package manager and distribution system for Gentoo",
        languages: &["Shell"],
        html: || {
            html! {
            <ul>
                <li>{ "- Package maintainer for " }<a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://github.com/easymodo/qimgv">{ "qimgv" }</a></li>
            </ul>
                    }
        },
    },
    Contribution {
        name: "Maven",
        url: "https://github.com/carlossg/docker-maven",
        icon_url: "https://github.com/kamiyaa/kamiyaa.github.io/releases/download/0.1.0/maven_icon.png",
        description: "Maven Dockerfiles",
        languages: &["Scripting"],
        html: || {
            html! {
            <ul>
                <li>{ "- Add Dockerfile with " }<a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://www.eclipse.org/openj9/">{ "OpenJ9" }</a>{ " as Java implementation" }</li>
            </ul>
                    }
        },
    },
    Contribution {
        name: "Jenkins",
        url: "https://github.com/jenkinsci/jenkins",
        icon_url: "https://github.com/kamiyaa/kamiyaa.github.io/releases/download/0.1.0/jenkins_icon.png",
        description: "Free and open source automation server",
        languages: &["Scripting"],
        html: || {
            html! {
            <ul>
                <li>{ "- Add Dockerfile with " }<a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://www.eclipse.org/openj9/">{ "OpenJ9" }</a>{ " as Java implementation
                with " }<a
                    target="_blank"
                    rel="noopener noreferrer"
                    href="https://www.eclipse.org/openj9/docs/shrc/">{ "Shared Classes Cache" }</a>{ " enabled" }</li>
                <li>{ "- Decrease memory usage by up to 40% and decrease startup times by up to 30%" }</li>
            </ul>
                    }
        },
    },
];
