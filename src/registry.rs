use std::fmt::Display;

pub enum Registry {
    DockerElasticCo,
    DockerIo,
    GcrIo,
    GhcrIo,
    Zalando,
    Others(String),
}

impl From<String> for Registry {
    fn from(s: String) -> Self {
        match s.as_ref() {
            "docker.elastic.co" => Self::DockerElasticCo,
            "docker.io" => Self::DockerIo,
            "gcr.io" => Self::GcrIo,
            "ghcr.io" => Self::GhcrIo,
            "registry.opensource.zalan.do" => Self::Zalando,
            _ => Self::Others(s),
        }
    }
}

impl Display for Registry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Registry::DockerElasticCo => write!(f, "docker.elastic.co"),
            Registry::DockerIo => write!(f, "registry-1.docker.io"),
            Registry::GcrIo => write!(f, "gcr.io"),
            Registry::GhcrIo => write!(f, "ghcr.io"),
            Registry::Zalando => write!(f, "registry.opensource.zalan.do"),
            Registry::Others(s) => write!(f, "{s}"),
        }
    }
}
