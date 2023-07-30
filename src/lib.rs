use std::fmt::Debug;

pub mod cli;
pub mod config;
pub mod upstream;

#[derive(Debug)]
pub enum RunningMode {
    Server,
    Client,
}

impl Default for RunningMode {
    fn default() -> Self {
        Self::Client
    }
}

#[derive(Debug)]
pub struct App {
    pub mode: RunningMode,
    pub config: config::Config,
    pub upstreams: Vec<upstream::Upstream>,
}

impl App {
    pub fn new(path_str: Option<&str>) -> Self {
        let config = config::Config::from_path(path_str);
        let upstreams: Vec<upstream::Upstream> = config
            .upstreams
            .iter()
            .map(|upstream| upstream::Upstream::new(&upstream.address))
            .collect();

        Self {
            config,
            upstreams,
            mode: Default::default(),
        }
    }
}
