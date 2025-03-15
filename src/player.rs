use crate::item::Item;
use crate::location::Location;

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
        let old_position = self.position;

        match direction {
            "north" => self.position.1 += 1,
            "south" => self.position.1 -= 1,
            "east" => self.position.0 += 1,
            "west" => self.position.0 -= 1,
            _ => return Err(format!("Invalid direction: {}", direction)),
        }

        // 使用from_coordinates检查新位置是否有效
        let new_location = Location::from_coordinates(self.position.0, self.position.1);

        // 如果是EmptyArea，表示无效位置，恢复原位置并返回错误
        if matches!(new_location, Location::EmptyArea) {
            self.position = old_position;
            return Err(format!("You cannot go {} from here.", direction));
        }

        Ok(())
    }

    // 获取当前位置的Location枚举
    pub fn current_location(&self) -> Location {
        Location::from_coordinates(self.position.0, self.position.1)
    }

    // handle item taking logic
    pub fn add_to_inventory(&mut self, item: String) {
        self.inventory.push(item);
    }

    // handle item removing logic
    pub fn remove_from_inventory(&mut self, item: &str) -> Option<String> {
        if let Some(index) = self.inventory.iter().position(|i| i == item) {
            Some(self.inventory.remove(index))
        } else {
            None
        }
    }

    // handle item using logic
    pub fn use_item(&mut self, item: &str) -> Result<String, String> {
        if let Some(index) = self.inventory.iter().position(|i| i == item) {
            // try to convert string to Item enum
            let result = if let Some(item_enum) = Item::from_string(item) {
                Ok(item_enum.use_description())
            } else {
                Err(format!("You don't know how to use {}", item))
            };

            // only remove item from inventory if it is successfully used
            if result.is_ok() {
                // check if item is consumable
                if let Some(item_enum) = Item::from_string(item) {
                    if item_enum.is_consumable() {
                        self.inventory.remove(index);
                    }
                }
            }

            result
        } else {
            Err(format!("You don't have {}", item))
        }
    }

    // handle inventory display logic
    pub fn display_inventory(&self) -> String {
        if self.inventory.is_empty() {
            "You have no items in your inventory.".to_string()
        } else {
            let mut result = String::from("Inventory:\n");
            for (index, item) in self.inventory.iter().enumerate() {
                result.push_str(&format!("{}: {}\n", index + 1, item));
            }
            result
        }
    }

    // handle player name logic
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }

    // handle player name getter
    pub fn get_name(&self) -> &str {
        &self.name
    }
}
