use structopt::StructOpt;

/// Command line arguments to be parsed by StructOpt
#[derive(Debug, StructOpt)]
#[structopt(about = "Starts an ACI Server")]
pub struct Arguments
{
    #[structopt(flatten)]
    pub verbose: clap_verbosity_flag::Verbosity,

    /// Input file
    #[structopt(parse(from_os_str))]
    input: std::path::PathBuf,
}