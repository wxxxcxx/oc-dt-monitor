use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Start {
    /// Interval of check （second）
    #[structopt(env = "OC_INTERVAL", short, long, default_value = "3600")]
    pub interval: u64,
}

#[derive(Debug, StructOpt)]
pub enum Command {
    /// Start the monitor
    #[structopt(name = "start")]
    Start(Start),
}

#[derive(Debug, StructOpt)]
pub enum StopMethod {
    Soft,
    Hard,
}

/// A basic example
#[derive(StructOpt, Debug)]
#[structopt(name = "oc-dt-monitor",about = "An oracle cloud data transfer usage monitor", global_settings = &[ structopt::clap::AppSettings::DisableVersion])]
pub struct Opt {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// The oci executable path
    #[structopt(env = "OCDTM_EXECUTABLE", short, long, default_value = "oci")]
    pub path: PathBuf,

    /// The oci config path
    #[structopt(env = "OCDTM_CONFIG", short, long, default_value = "~/.oci/config")]
    pub config: PathBuf,

    /// Oracle Cloud tenancy id
    #[structopt(env = "OCDTM_TENANT_ID", short, long)]
    pub tenant_id: String,

    /// Stop instance(s) when the data transfer reaches the threshold
    #[structopt(short, long)]
    pub auto_stop: bool,

    /// Instance ids that need to be stopped, if not specified, all instances will be stopped by default
    #[structopt(env = "OCDTM_STOP_INSTANCES", long, value_delimiter = ",")]
    pub instances: Option<Vec<String>>,

    /// The stop threshold of data transfer in GB
    #[structopt(env = "OCDTM_THRESHOLD", long, default_value = "1000")]
    pub threshold: u32,

    /// Use soft stop to stop instance ( soft` or `hard` )
    #[structopt(env = "OCDTM_STOP_METHOD", long, default_value = "soft")]
    pub stop_method: String,

    /// Use clean output (Only output the used data transfer)
    #[structopt(long)]
    pub clean: bool,

    /// Run command
    #[structopt(subcommand)]
    pub command: Option<Command>,
}
