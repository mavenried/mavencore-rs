use clap::Parser;
mod args;
mod handlers;

use args::{Action, ProjArgs};
use handlers::*;

fn main() {
    let args = ProjArgs::parse();

    match args.action {
        Action::Listen(listen) => match listen.compositor.as_str() {
            "niri" => {
                if let Err(e) = handle_niri() {
                    eprintln!("{e}")
                }
            }
            _ => eprint!("Not Supported!"),
        },
        Action::Cpu => handle_cpu(),
        Action::Memory => handle_memory(),
        Action::Disk(disk) => handle_disk(disk),
        Action::Battery(battery) => handle_battery(battery),
        Action::BatteryIcon(battery) => handle_battery_icon(battery),
        Action::Power(battery) => handle_power(battery),
    }
}
