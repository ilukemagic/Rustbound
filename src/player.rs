#[derive(Debug)]
pub struct Player {
    pub name: String,
    pub position: (i32, i32),
    pub inventory: Vec<String>,
}

impl Player {
    pub fn new(name: String) -> Self {
        Player {
            name,
            position: (0, 0),
            inventory: Vec::new(),
        }
    }

    // move the player to a new position
    pub fn move_to(&mut self, direction: &str) -> Result<(), String> {
        match direction {
            "north" => self.position.1 += 1,
            "south" => self.position.1 -= 1,
            "east" => self.position.0 += 1,
            "west" => self.position.0 -= 1,
            _ => return Err(format!("Invalid direction: {}", direction)),
        }
        Ok(())
    }

    // handle item taking logic
    pub fn add_to_inventory(&mut self, item: String) {
        self.inventory.push(item);
    }

    // handle item removing logic
    pub fn remove_from_inventory(&mut self, item: String) -> bool {
        self.inventory.iter().any(|i| i == &item)
    }

    // handle item using logic
    pub fn use_item(&mut self, item: &str) -> Result<String, String> {
        if let Some(index) = self.inventory.iter().position(|i| i == item) {
            self.inventory.remove(index);
            match item {
                "potion" => Ok("Health restored!".to_string()),
                "key" => Ok("The door creaks open...".to_string()),
                _ => Err(format!("You can't use {}", item)),
            }
        } else {
            Err(format!("You don't have {}", item))
        }
    }
}
