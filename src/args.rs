use clap::{Args, Parser, Subcommand};

#[derive(Debug, Parser)]
pub struct ProjArgs {
    #[clap(subcommand)]
    pub action: Action,
}

#[derive(Debug, Subcommand)]
pub enum Action {
    /// Listen for events on a compositor's socket
    Listen(Listen),
    /// Cpu usage %
    Cpu,
    /// Memory usage %
    Memory,
    /// Uptime in dd:hh:mm:ss
    Uptime,
    /// Disk usage %, takes <path> eg. /
    Disk(PathObj),
    /// Battery %, takes <battery path> eg. /sys/class/power_supply/BAT1
    Battery(PathObj),
    /// Battery Icon, takes <battery path> eg. /sys/class/power_supply/BAT1
    BatteryIcon(PathObj),
    /// Power consumption in Watts, takes <battery path> eg. /sys/class/power_supply/BAT1
    Power(PathObj),
}

#[derive(Debug, Args)]
pub struct PathObj {
    pub path: String,
}

#[derive(Debug, Args)]
pub struct Listen {
    /// Niri
    pub compositor: String,
}
