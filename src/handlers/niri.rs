use niri_ipc::{
    Request,
    state::{EventStreamState, EventStreamStatePart},
};
use serde::Serialize;

#[derive(Serialize)]
struct State {
    workspaces: Vec<usize>,
    workspace_id: usize,
    window_name: String,
}

impl From<&EventStreamState> for State {
    fn from(value: &EventStreamState) -> Self {
        let mut out = Self {
            workspaces: value
                .workspaces
                .workspaces
                .iter()
                .map(|ws| *ws.0 as usize)
                .collect(),
            workspace_id: *value
                .workspaces
                .workspaces
                .iter()
                .find(|ws| ws.1.is_active)
                .unwrap()
                .0 as usize,
            window_name: {
                let window = value.windows.windows.iter().find(|win| win.1.is_focused);
                if let Some(win) = window {
                    win.1.title.clone().unwrap()
                } else {
                    String::from("desktop")
                }
            },
        };
        out.workspaces.sort();
        out
    }
}

pub fn handle_niri() -> std::io::Result<()> {
    let mut socket = niri_ipc::socket::Socket::connect()?;
    let reply = socket.send(Request::EventStream)?;
    if let Err(e) = reply {
        eprintln!("{e}");
        std::process::exit(1);
    }

    let mut state = niri_ipc::state::EventStreamState::default();
    let mut read_event = socket.read_events();
    while let Ok(event) = read_event() {
        state.apply(event);
        println!("{}", serde_json::to_string(&State::from(&state))?)
    }
    Ok(())
}
