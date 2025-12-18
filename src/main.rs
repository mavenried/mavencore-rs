use clap::Parser;
mod args;
mod handlers;

use args::{Action, ProjArgs};
use handlers::*;

fn main() {
    let args = ProjArgs::parse();

    match args.action {
        Action::Listen => match std::env::var("XDG_CURRENT_DESKTOP")
            .expect("Failed to load $XDG_CURRENT_DESKTOP")
            .as_str()
        {
            "niri" => {
                if let Err(e) = handle_niri() {
                    eprintln!("{e}")
                }
            }
            "hyprland" => {
                if let Err(e) = handle_hyprland() {
                    eprintln!("{e}")
                }
            }
            compositor => eprintln!("{compositor} is not supported yet! :("),
        },
        Action::Cpu => handle_cpu(),
        Action::Memory => handle_memory(),
        Action::Uptime => handle_uptime(),

        Action::Disk(disk) => handle_disk(disk),
        Action::Battery(battery) => handle_battery(battery),
        Action::BatteryIcon(battery) => handle_battery_icon(battery),
        Action::Power(battery) => handle_power(battery),

        Action::AppsList => handle_apps_list(),
    }
}
