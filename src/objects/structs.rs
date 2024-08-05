use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalConfiguration {
    #[serde(rename(deserialize = "banner"))]
    pub print_banner: bool,
    pub client: Option<String>,
    pub organization: Option<String>,
    pub configuration_file: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectConfiguation {
    #[serde(rename(deserialize = "path"))]
    pub repository_path: String,
    pub service: Vec<Service>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    #[serde(rename(deserialize = "application_name"))]
    pub name: String,
    #[serde(rename(deserialize = "url"))]
    pub repository_url: String,
    #[serde(rename(deserialize = "directory_name"))]
    pub slug: String,
    pub container: Container,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Container {
    #[serde(rename(serialize = "container_name", deserialize = "name"))]
    pub name: String,
    pub image: String,
    #[serde(rename(deserialize = "restart_policy"))]
    pub restart: String,
    #[serde(rename(deserialize = "docker_user_group_id"))]
    pub user: Option<String>,
    #[serde(rename(deserialize = "standard_in"))]
    pub std_in: Option<bool>,
    #[serde(rename(deserialize = "interactive"))]
    pub tty: Option<bool>,
    pub ports: Vec<Vec<String>>,
    pub volumes: Vec<Vec<String>>,
    pub build: Build,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Build {
    pub context: String,
    pub dockerfile: String,
}
