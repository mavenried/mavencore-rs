use crate::args::PathObj;
use std::fs::read_to_string;
use std::process::Command;
use std::{fs, thread, time::Duration};

mod niri;
pub use niri::handle_niri;
// <-Helpers-------------------->
fn read_cpu() -> (u64, u64) {
    let stat = fs::read_to_string("/proc/stat").unwrap();
    let line = stat.lines().next().unwrap();
    let parts: Vec<&str> = line.split_whitespace().collect();

    let user: u64 = parts[1].parse().unwrap();
    let nice: u64 = parts[2].parse().unwrap();
    let system: u64 = parts[3].parse().unwrap();
    let idle: u64 = parts[4].parse().unwrap();

    let busy = user + nice + system;
    let total = busy + idle;

    (busy, total)
}

fn get_battery(battery: &PathObj) -> f64 {
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

fn is_charging(battery: &PathObj) -> bool {
    if let Ok(status) = fs::read_to_string(format!("{}/status", battery.path)) {
        return status == "Charging";
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
    let meminfo = fs::read_to_string("/proc/meminfo").unwrap();

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

pub fn handle_disk(disk: PathObj) {
    let output = Command::new("df")
        .arg(disk.path)
        .output()
        .expect("failed to run df");

    let stdout = String::from_utf8_lossy(&output.stdout);

    if let Some(line) = stdout.lines().nth(1) {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 5 {
            let pct = parts[4].trim_end_matches('%');
            println!("{}", pct);
            return;
        }
    }

    println!("?");
}

pub fn handle_battery(battery: PathObj) {
    println!("{:.0}", get_battery(&battery));
}

pub fn handle_battery_icon(battery: PathObj) {
    let icons = ['󰁻', '󰁻', '󰁼', '󰁽', '󰁾', '󰁿', '󰂀', '󰂀', '󰂂', '󰁹'];
    let icon: char = {
        let pct = get_battery(&battery);
        if pct < 0.0 {
            '?'
        } else if is_charging(&battery) {
            '󰂄'
        } else {
            let idx = ((pct - 0.1) as usize / 10) % 10;
            icons[idx]
        }
    };

    println!("{}", icon);
}
pub fn handle_power(battery: PathObj) {
    let p = fs::read_to_string(format!("{}/power_now", battery.path));
    match p {
        Ok(val) => {
            let val: f64 = val.trim().parse().unwrap_or(0.0);

            println!("{:.0}", val / 1000000.0)
        }
        Err(_) => println!("?"),
    }
}

pub fn handle_uptime() {
    let uptime_seconds: usize = read_to_string("/proc/uptime")
        .unwrap()
        .split(" ")
        .next()
        .unwrap()
        .parse::<f32>()
        .unwrap() as usize;
    let uptime_hours = (uptime_seconds / 3600) % 24;
    let uptime_days = uptime_seconds / 86400;
    let uptime_min = (uptime_seconds / 60) % 60;
    let uptime_seconds = uptime_seconds % 60;

    println!("{uptime_days:0>2}:{uptime_hours:0>2}:{uptime_min:0>2}:{uptime_seconds:0>2}");
}
