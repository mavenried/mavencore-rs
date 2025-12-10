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
    /// Cpu usage %
    Memory,
    /// Disk usage % <path>
    Disk(PathObj),
    /// Battery % <battery path> eg. /sys/class/power_supply/BAT1
    Battery(PathObj),
    /// Battery % <battery path> eg. /sys/class/power_supply/BAT1
    BatteryIcon(PathObj),
    /// Power consumption in Watts <battery path> eg. /sys/class/power_supply/BAT1
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
