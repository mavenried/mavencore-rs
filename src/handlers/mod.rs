use crate::args::PathArg;
use std::fs::read_to_string;
use std::{fs, thread, time::Duration};

// <-Modules-------------------->
mod apps_list;
mod hyprland;
mod niri;
mod types;
pub use apps_list::handle_apps_list;
pub use hyprland::handle_hyprland;
pub use niri::handle_niri;
pub use types::State;

// <-Helpers-------------------->
fn read_cpu() -> (u64, u64) {
    let stat = fs::read_to_string("/proc/stat").expect("/proc/stat unreadable");
    let line = stat.lines().next().expect("empty /proc/stat");
    let parts: Vec<&str> = line.split_whitespace().collect();

    let user: u64 = parts[1].parse().unwrap();
    let nice: u64 = parts[2].parse().unwrap();
    let system: u64 = parts[3].parse().unwrap();
    let idle: u64 = parts[4].parse().unwrap();

    let busy = user + nice + system;
    let total = busy + idle;

    (busy, total)
}

fn get_battery(battery: &PathArg) -> f64 {
    let energy_now = fs::read_to_string(format!("{}/energy_now", battery.path));
    let energy_full = fs::read_to_string(format!("{}/energy_full", battery.path));

    if let (Ok(now), Ok(full)) = (energy_now, energy_full) {
        let now: f64 = now.trim().parse().unwrap_or(0.0);
        let full: f64 = full.trim().parse().unwrap_or(0.0);

        if full > 0.0 {
            let pct = (now / full) * 100.0;
            return pct;
        }
    }
    -1f64 // should be fine?
}

fn is_charging(battery: &PathArg) -> bool {
    if let Ok(status) = fs::read_to_string(format!("{}/status", battery.path)) {
        return status.trim() == "Charging";
    }
    true // showing charging when not is more likely to get noticed.
}

// <-Handlers------------------->
pub fn handle_cpu() {
    let (b1, t1) = read_cpu();
    thread::sleep(Duration::from_millis(1000));
    let (b2, t2) = read_cpu();

    let db = b2 - b1;
    let dt = t2 - t1;

    let usage = if dt == 0 {
        0.0
    } else {
        db as f64 / dt as f64 * 100.0
    };
    println!("{:.0}", usage);
}

pub fn handle_memory() {
    let meminfo = fs::read_to_string("/proc/meminfo").expect("/proc/meminfo unreadable");

    let mut total = 0u64;
    let mut available = 0u64;

    for line in meminfo.lines() {
        if line.starts_with("MemTotal:") {
            total = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        } else if line.starts_with("MemAvailable:") {
            available = line.split_whitespace().nth(1).unwrap().parse().unwrap();
        }
    }

    if total == 0 {
        println!("0%");
        return;
    }

    let used = total - available;
    let pct = used as f64 / total as f64 * 100.0;

    println!("{:.0}", pct);
}

pub fn handle_disk(disk: PathArg) {
    use std::ffi::CString;
    let path = CString::new(disk.path.as_bytes()).expect("path contains null byte");
    let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(path.as_ptr(), &mut stat) } != 0 {
        println!("?");
        return;
    }
    if stat.f_blocks == 0 {
        println!("?");
        return;
    }
    let used = stat.f_blocks - stat.f_bfree;
    println!("{:.0}", used as f64 / stat.f_blocks as f64 * 100.0);
}

pub fn handle_battery(battery: PathArg) {
    println!("{:.0}", get_battery(&battery));
}

pub fn handle_battery_icon(battery: PathArg) {
    let icons = ['\u{F007B}', '\u{F007B}', '\u{F007C}', '\u{F007D}', '\u{F007E}', '\u{F007F}', '\u{F0080}', '\u{F0080}', '\u{F0082}', '\u{F0079}'];
    let icon: char = {
        let pct = get_battery(&battery);
        if pct < 0.0 {
            '?'
        } else if is_charging(&battery) {
            '\u{F0084}'
        } else {
            let idx = (pct as usize / 10).min(9);
            icons[idx]
        }
    };

    println!("{}", icon);
}
pub fn handle_power(battery: PathArg) {
    let p = fs::read_to_string(format!("{}/power_now", battery.path));
    match p {
        Ok(val) => {
            let val: f64 = val.trim().parse().unwrap_or(0.0);

            println!("{:.0}", val / 1000000.0)
        }
        Err(_) => println!("?"),
    }
}

pub fn handle_online(host: Option<&str>) {
    use std::net::TcpStream;
    let host = host.unwrap_or("1.1.1.1:80");
    let reachable = TcpStream::connect_timeout(
        &host.parse().unwrap_or("1.1.1.1:80".parse().unwrap()),
        Duration::from_secs(1),
    )
    .is_ok();
    if reachable {
        println!("\u{F0928} "); // 󰤨
    } else {
        println!("\u{F092D} "); // 󰤭
    }
}

pub fn handle_uptime() {
    let content = read_to_string("/proc/uptime").expect("/proc/uptime unreadable");
    let uptime_seconds = content
        .split_ascii_whitespace()
        .next()
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(0.0) as usize;
    let uptime_hours = (uptime_seconds / 3600) % 24;
    let uptime_days = uptime_seconds / 86400;
    let uptime_min = (uptime_seconds / 60) % 60;
    let uptime_seconds = uptime_seconds % 60;

    println!("{uptime_days:0>2}:{uptime_hours:0>2}:{uptime_min:0>2}:{uptime_seconds:0>2}");
}
