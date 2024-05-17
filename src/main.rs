use std::collections::BTreeMap;
use std::str::FromStr;

use zellij_tile::prelude::*;

#[derive(Default)]
struct State {
    active_tab: usize,
}

register_plugin!(State);

impl ZellijPlugin for State {
    #[allow(unused_variables)]
    fn load(&mut self, configuration: BTreeMap<String, String>) {
        // we need the ReadApplicationState permission to receive the TabUpdate events
        request_permission(&[
            PermissionType::ReadApplicationState,
            PermissionType::ChangeApplicationState,
        ]);
        subscribe(&[EventType::TabUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        if let Event::TabUpdate(tab_info) = event {
            self.active_tab = tab_info
                .iter()
                .filter(|t| t.active)
                .next()
                .expect("No active tab found")
                .position;

            let mode = if self.active_tab == 0 {
                Mode::from_str("Locked")
            } else {
                Mode::from_str("Normal")
            };

            switch_to_input_mode(&mode.unwrap_or(Mode(InputMode::Normal)).0);
        };

        false
    }
}

struct Mode(InputMode);

impl FromStr for Mode {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s {
            "Locked" | "locked" => Ok(Mode(InputMode::Locked)),
            "Normal" | "normal" => Ok(Mode(InputMode::Normal)),
            _ => Ok(Mode(InputMode::Normal)),
        }
    }
}
