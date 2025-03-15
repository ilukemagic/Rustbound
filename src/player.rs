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
            let result = match item {
                "small potion" => Ok("You drink the small potion and feel your energy restored!".to_string()),
                "mysterious potion" => Ok("You drink the mysterious potion and suddenly feel like you can understand animal language!".to_string()),
                "healing herb" => Ok("You use the healing herb and your wounds begin to heal.".to_string()),
                "rusty sword" => Ok("You swing the rusty sword. Though somewhat dull, it can still be used for defense.".to_string()),
                "steel sword" => Ok("You swing the sharp steel sword, gleaming in the sunlight. This sword can easily defeat most enemies.".to_string()),
                "wooden shield" => Ok("You raise the wooden shield, which provides basic protection.".to_string()),
                "lantern" => Ok("You light the lantern, illuminating the dark area. You can now see further.".to_string()),
                "torch" => Ok("The torch lights up the surrounding area, dispelling the darkness.".to_string()),
                "climbing rope" => Ok("You use the rope to climb to a hard-to-reach place.".to_string()),
                "ancient key" => Ok("This ancient key seems capable of opening an important door.".to_string()),
                "golden key" => Ok("The golden key shines in your hand, looking like it could open a valuable chest.".to_string()),
                "dimensional key" => Ok("As you hold the dimensional key, you feel energy flowing through your body. It seems capable of activating some kind of teleportation device.".to_string()),
                "glowing orb" => Ok("The glowing orb emits a soft light, and you feel a mysterious power flowing within it.".to_string()),
                "ceremonial dagger" => Ok("The ceremonial dagger doesn't seem suitable for combat, but it might be useful in certain special rituals.".to_string()),
                "strange crystal" => Ok("When you touch the strange crystal, it begins to emit pulsating light, and strange images flash through your mind.".to_string()),
                "ancient tablet" => Ok("You study the ancient tablet, and the symbols seem to tell the story of an ancient civilization.".to_string()),
                "scarab amulet" => Ok("When you wear the scarab amulet, you feel protected by some kind of force.".to_string()),
                "pharaoh's mask" => Ok("The pharaoh's mask exudes an aura of majesty. Wearing it makes you feel like you possess the power of a king.".to_string()),
                "jeweled crown" => Ok("The jewel-encrusted crown sparkles in the sunlight. It must be worth a fortune.".to_string()),
                "magic scroll" => Ok("You open the magic scroll, and the text begins to glow and float in the air before disappearing. You feel like you've learned something new.".to_string()),
                "fishing rod" => Ok("You use the fishing rod by the river, waiting patiently. Soon, you catch a fish!".to_string()),
                "bread" => Ok("You eat the bread and feel your hunger subside.".to_string()),
                "water flask" => Ok("You take a sip of water and feel refreshed.".to_string()),
                "exotic fruit" => Ok("You taste the exotic fruit, which is both sweet and sour, and very delicious. You feel energized.".to_string()),
                "waterskin" => Ok("You take a sip from the waterskin, a valuable resource in the desert.".to_string()),
                "old map" => Ok("You carefully study the old map and discover some location markers you hadn't noticed before. This might help you find hidden treasures.".to_string()),
                _ => Err(format!("You don't know how to use {}", item)),
            };

            // Only remove the item from inventory after successful use
            if result.is_ok() {
                // Some items shouldn't disappear after use
                match item {
                    "rusty sword" | "steel sword" | "wooden shield" | "lantern" | "torch"
                    | "climbing rope" | "ancient key" | "golden key" | "dimensional key"
                    | "glowing orb" | "ceremonial dagger" | "strange crystal"
                    | "ancient tablet" | "scarab amulet" | "pharaoh's mask" | "jeweled crown"
                    | "fishing rod" | "water flask" | "waterskin" | "old map" => {}
                    _ => {
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
