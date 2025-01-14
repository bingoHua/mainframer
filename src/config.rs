use sync::PullMode;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Config {
    pub remote: Remote,
    pub push: Push,
    pub pull: Pull,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Remote {
    pub host: String,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Push {
    pub compression: u8,
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Pull {
    pub compression: u8,
    pub mode: PullMode,
}
