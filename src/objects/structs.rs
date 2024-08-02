use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct GlobalConfiguration {
    pub print_banner: bool,
    pub client: Option<String>,
    pub organization: Option<String>,
    pub configuration_file: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ProjectConfiguation {
    pub repository_path: String,
    pub service: Vec<Service>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Service {
    pub name: String,
    #[serde(rename = "repository_url")]
    pub repository_url: String,
    pub slug: String,
    pub container: Container,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Container {
    pub name: String,
    pub image: String,
    pub restart: String,
    pub ports: Vec<Vec<String>>,
    pub volumes: Vec<Vec<String>>,
    pub build: Build,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Build {
    pub dockerfile: String,
}
