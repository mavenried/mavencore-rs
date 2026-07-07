use clap::Parser;
mod args;
mod handlers;

use args::{Action, Cli};
use handlers::*;

fn main() {
    let args = Cli::parse();

    match args.action {
        Action::Listen => {
            let desktop = std::env::var("XDG_CURRENT_DESKTOP").unwrap_or_else(|_| {
                eprintln!("mavencore: $XDG_CURRENT_DESKTOP is not set");
                std::process::exit(1);
            });
            match desktop.to_lowercase().as_str() {
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
            }
        }
        Action::Cpu => handle_cpu(),
        Action::Memory => handle_memory(),
        Action::Uptime => handle_uptime(),

        Action::Disk(disk) => handle_disk(disk),
        Action::Battery(battery) => handle_battery(battery),
        Action::BatteryIcon(battery) => handle_battery_icon(battery),
        Action::Power(battery) => handle_power(battery),

        Action::AppsList => handle_apps_list(),
        Action::Online(args) => handle_online(args.host.as_deref()),
    }
}
