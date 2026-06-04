use std::{
    io::{BufRead, BufReader, Read, Write},
    os::unix::net::UnixStream,
    path::PathBuf,
};

use super::State;

fn get_hyprland_socket_path() -> (PathBuf, PathBuf) {
    let runtime_dir = PathBuf::from(std::env::var("XDG_RUNTIME_DIR").unwrap_or_else(
        |err| match err {
            std::env::VarError::NotPresent => {
                eprintln!("Could not find $XDG_RUNTIME_DIR! What!How?");
                std::process::exit(1)
            }
            std::env::VarError::NotUnicode(data) => {
                eprintln!("$XDG_RUNTIME_DIR does not contain valid unicode: {data:?}");
                std::process::exit(1)
            }
        },
    ));
    let his = std::env::var("HYPRLAND_INSTANCE_SIGNATURE").unwrap_or_else(|err| match err {
        std::env::VarError::NotPresent => {
            eprintln!("Could not find $HYPRLAND_INSTANCE_SIGNATURE!");
            std::process::exit(1)
        }
        std::env::VarError::NotUnicode(data) => {
            eprintln!("$HYPRLAND_INSTANCE_SIGNATURE does not contain valid unicode: {data:?}");
            std::process::exit(1)
        }
    });
    let hyprland_dir = runtime_dir.join("hypr").join(his);
    (
        hyprland_dir.join(".socket.sock"),
        hyprland_dir.join(".socket2.sock"),
    )
}

pub fn handle_hyprland() -> std::io::Result<()> {
    let (sock, sock2) = get_hyprland_socket_path();
    let socket = UnixStream::connect(sock2)?;

    let mut reader = BufReader::new(socket);
    let mut state = State::default();

    {
        let mut init = UnixStream::connect(sock.clone())?;
        let mut buf = String::new();

        init.write_all(b"j/activeworkspace")?;
        init.read_to_string(&mut buf)?;

        if let Some(ws) = buf
            .lines()
            .find_map(|l| l.trim().strip_prefix("\"id\": "))
            .map(|s| s.chars().filter(|c| c.is_numeric()).collect::<String>())
            .and_then(|s| s.parse::<usize>().ok())
        {
            state.workspace_id = ws;
            state.workspaces.push(ws);
            println!("{}", serde_json::to_string(&state).unwrap())
        }
    }

    {
        let mut init = UnixStream::connect(sock)?;
        let mut buf = String::new();

        init.write_all(b"activewindow")?;
        init.read_to_string(&mut buf)?;

        if let Some(t) = buf.lines().find_map(|l| l.trim().strip_prefix("title: ")) {
            state.window_name = t.to_string();

            if state.window_name.trim().is_empty() {
                state.window_name = "desktop".to_string();
            }

            println!("{}", serde_json::to_string(&state).unwrap())
        }
    }

    loop {
        let mut line = String::new();
        reader.read_line(&mut line)?;
        let mut split = line.split(">>");
        let event = split.next().unwrap_or("");
        let data = split.next().unwrap_or("");

        match event {
            "activewindow" => {
                state.window_name = {
                    let wn = data.split(",").nth(1).unwrap_or("").trim();
                    if wn.is_empty() {
                        "desktop".into()
                    } else {
                        wn.into()
                    }
                };
                println!("{}", serde_json::to_string(&state).unwrap())
            }
            "workspace" => {
                let active_ws = data.trim().parse::<usize>().unwrap();
                state.workspace_id = active_ws;
                if !state.workspaces.contains(&active_ws) {
                    state.workspaces.push(active_ws);
                    state.workspaces.sort();
                }
                println!("{}", serde_json::to_string(&state).unwrap())
            }
            _ => (),
        }
    }
}
