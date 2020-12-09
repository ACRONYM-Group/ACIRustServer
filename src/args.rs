use structopt::StructOpt;

/// Command line arguments to be parsed by StructOpt
#[derive(Debug, StructOpt, Clone)]
#[structopt(about = "Starts an ACI Server")]
#[structopt(rename_all = "kebab-case")]
pub struct Arguments
{
    #[structopt(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// Error on any non matching version
    #[structopt(short, long)]
    pub mismatch: bool,

    /// Allow all database versions to be loaded (may cause instability)
    #[structopt(short, long)]
    pub allow_all: bool,

    /// Input file
    #[structopt(parse(from_os_str), default_value = "test-databases/")]
    pub path: std::path::PathBuf,

    /// Config Path
    #[structopt(parse(from_os_str), default_value = "databases-dev/")]
    pub config_path: std::path::PathBuf,
}