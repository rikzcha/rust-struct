use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    pub first_name: String,
    pub last_name: String,
}
// Interface
pub trait FullName {
    fn full_name(&self) -> String;
}
// Implementing interface
impl FullName for Player {
    fn full_name(&self) -> String {
        format!("{}, {}", self.first_name, self.last_name)
    }
}

impl Default for Player {
    fn default() -> Self {
        Player {
            first_name: "".to_string(),
            last_name: "".to_string(),
        }
    }
}

impl Player {
    pub fn new(first_name: String, last_name: String) -> Player {
        Player {
            first_name, // first_name: first_name,
            last_name, // last_name: last_name,
        }
    }

    pub fn update(player: Player) -> Self {
        Player {
            .. player
        }
    }

    pub fn update_first_name(first_name: String, player: Player) -> Self {
        Player {
            first_name,
            .. player
        }
    }

    pub fn update_last_name(last_name: String, player: Player) -> Self {
        Player {
            last_name,
            .. player
        }
    }
}

