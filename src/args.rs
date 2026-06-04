use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct Cli {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Listen for events on the active compositor's socket
    Listen,
    /// Cpu usage %
    Cpu,
    /// Memory usage %
    Memory,
    /// Uptime in dd:hh:mm:ss
    Uptime,
    /// Disk usage %, takes <path> eg. /
    Disk(PathArg),
    /// Battery %, takes <battery path> eg. /sys/class/power_supply/BAT1
    Battery(PathArg),
    /// Battery Icon, takes <battery path> eg. /sys/class/power_supply/BAT1
    BatteryIcon(PathArg),
    /// Power consumption in Watts, takes <battery path> eg. /sys/class/power_supply/BAT1
    Power(PathArg),
    /// List installed applications as JSON
    AppsList,
    /// Check network reachability, optional <host:port> (default: 1.1.1.1:80)
    Online(OnlineArgs),
}

#[derive(Debug, Args)]
pub struct OnlineArgs {
    /// Host and port to probe, e.g. 1.1.1.1:80
    pub host: Option<String>,
}

#[derive(Debug, Args)]
pub struct PathArg {
    pub path: String,
}
