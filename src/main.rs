use std::collections::BTreeMap;
use zellij_tile::prelude::*;

#[derive(Default)]
struct Zessman;

register_plugin!(Zessman);

impl ZellijPlugin for Zessman {
    fn load(&mut self, _: BTreeMap<String, String>) {
        request_permission(&[
            PermissionType::RunCommands,
            PermissionType::ChangeApplicationState,
        ]);
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if let Some(session_name) = pipe_message.payload {
            if session_name.is_empty() {
                eprintln!("No session name provided");
                return false;
            }
            eprintln!("Switching to session: {}", session_name);
            switch_session(Some(session_name.as_str()));
        }
        false
    }
}
