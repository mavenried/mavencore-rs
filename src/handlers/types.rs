use serde::Serialize;

#[derive(Serialize, Default)]
pub struct State {
    pub workspaces: Vec<usize>,
    pub workspace_id: usize,
    pub window_name: String,
}
