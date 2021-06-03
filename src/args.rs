use structopt::StructOpt;

/// Command line arguments to be parsed by StructOpt
#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "Starts an ACI Server")]
#[structopt(version = crate::version::BUILD_VERSION)]
#[structopt(rename_all = "kebab-case")]
pub struct Arguments
{
    #[structopt(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// Error on any non matching version
    #[structopt(short, long)]
    pub mismatch: bool,

    /// Do not use the config file for global settings (will still load the config database for user authentication)
    #[structopt(short, long)]
    pub ignore_config: bool,

    /// Allow all database versions to be loaded (may cause instability)
    #[structopt(short, long)]
    pub allow_all: bool,

    /// Run in raw socket mode
    #[structopt(short, long)]
    pub raw_socket: bool,

    /// IP address to connect the server to (overrides the config database)
    #[structopt(long)]
    pub ip: Option<String>,

    /// Port to connect the server to (overrides the config database)
    #[structopt(short, long)]
    pub port: Option<usize>,

    /// Database root directory
    #[structopt(parse(from_os_str), default_value = "test-databases/")]
    pub path: std::path::PathBuf,

    /// Config Path
    #[structopt(parse(from_os_str), default_value = "databases-dev/")]
    pub config_path: std::path::PathBuf,
}