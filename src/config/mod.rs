use serde::{Deserialize, Serialize};
use std::{fmt::Debug, fs, path::PathBuf, str::FromStr};

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct ServerArgs {
    listen: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Server {
    #[serde(rename = "type")]
    server_type: String,
    args: ServerArgs,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct UpstreamInConfig {
    pub address: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Default)]
pub struct Config {
    pub servers: Vec<Server>,
    pub upstreams: Vec<UpstreamInConfig>,
}

impl Config {
    pub fn from_path(path_str: Option<&str>) -> Self {
        let path = PathBuf::from_str(path_str.unwrap_or("config.yaml".into())).unwrap();
        let metadata = path.metadata().unwrap();
        let path = if metadata.is_symlink() {
            fs::read_link(path).unwrap()
        } else {
            path
        };
        let f = fs::File::open(path).unwrap();
        let config: Self = serde_yaml::from_reader(f).unwrap();
        config
    }
}
