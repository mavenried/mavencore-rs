use super::types::DesktopEntry;
use configparser::ini::Ini;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

fn parse_desktop(path: &Path, desktops: &[String]) -> Option<DesktopEntry> {
    let mut ini = Ini::new();
    ini.load(path).ok()?;

    let section = "Desktop Entry";

    let name = ini.get(section, "Name")?;

    // Hidden / NoDisplay
    if ini
        .getbool(section, "Hidden")
        .unwrap_or(Some(false))
        .is_some()
    {
        return None;
    }
    if ini
        .getbool(section, "NoDisplay")
        .unwrap_or(Some(false))
        .is_some()
    {
        return None;
    }

    // OnlyShowIn
    if let Some(only) = ini.get(section, "OnlyShowIn")
        && !only
            .split(';')
            .filter(|s| !s.is_empty())
            .any(|d| desktops.contains(&d.to_string()))
    {
        return None;
    }

    // NotShowIn
    if let Some(not) = ini.get(section, "NotShowIn")
        && not
            .split(';')
            .filter(|s| !s.is_empty())
            .any(|d| desktops.contains(&d.to_string()))
    {
        return None;
    }

    Some(DesktopEntry {
        name,
        icon: ini.get(section, "Icon"),
        path: path.to_string_lossy().to_string(),
    })
}

pub fn handle_apps_list() {
    let desktops: Vec<String> = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .split(':')
        .map(|s| s.to_string())
        .collect();

    let dirs: Vec<PathBuf> = std::env::var("XDG_DATA_DIRS")
        .unwrap_or_default()
        .split(':')
        .map(PathBuf::from)
        .map(|p| p.join("applications"))
        .collect();

    for dir in dirs {
        for entry in WalkDir::new(dir)
            .follow_links(true)
            .into_iter()
            .filter_map(Result::ok)
            .filter(|e| e.path().extension().and_then(|s| s.to_str()) == Some("desktop"))
        {
            if let Some(de) = parse_desktop(entry.path(), &desktops) {
                println!("{}", serde_json::to_string(&de).unwrap());
            }
        }
    }
}
