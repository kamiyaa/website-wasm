pub mod experiences;
pub mod projects;

mod contribution_list;
mod experience_list;
mod extracurricular_list;
mod header_links;
mod project_list;

pub use contribution_list::{Contribution, ContributionListType, CONTRIBUTION_LIST};
pub use experience_list::{Experience, ExperienceListType, EXPERIENCE_LIST};
pub use extracurricular_list::{ExtracurricularListType, EXTRACURRICULAR_LIST};
pub use header_links::{URLRoute, HEADER_LINKS};
pub use project_list::{GithubMetrics, Project, ProjectListType, GITHUB_METRICS, PROJECT_LIST};

pub const LOGO_NAME: &str = "Jeff Zhao";
