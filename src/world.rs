#[derive(Debug)]
pub struct Room {
    pub description: String,
    pub items: Vec<String>,
    pub npc: Option<String>,
}

pub struct World {
    rooms: Vec<Vec<Room>>,
}

impl World {
    pub fn new() -> Self {
        // Create a 5x5 world map
        let mut rooms = Vec::with_capacity(5);

        for x in 0..5 {
            let mut row = Vec::with_capacity(5);
            for y in 0..5 {
                row.push(Room {
                    description: format!("Empty area ({}, {})", x, y),
                    items: vec![],
                    npc: None,
                })
            }
            rooms.push(row);
        }

        // Starting area - Mysterious Forest
        rooms[0][0].description = String::from(
            "Entrance to the mysterious forest. Moonlight filters through the dense leaves, creating dappled shadows on the ground. Low growls of wild beasts can be heard in the distance.",
        );
        rooms[0][0].items.push(String::from("rusty sword"));
        rooms[0][0].items.push(String::from("small potion"));

        // Deep Forest
        rooms[0][1].description = String::from(
            "Deep in the forest. The trees are denser here, blocking almost all sunlight. You can hear birds singing and leaves rustling.",
        );
        rooms[0][1].items.push(String::from("wooden shield"));
        rooms[0][1].npc = Some(String::from("forest elf"));

        // Forest Cabin
        rooms[1][0].description = String::from(
            "A dilapidated wooden cabin sits at the edge of the forest. The door is half-open, and there are signs that someone lives here.",
        );
        rooms[1][0].items.push(String::from("old map"));
        rooms[1][0].items.push(String::from("lantern"));
        rooms[1][0].npc = Some(String::from("old hunter"));

        // Mountain Base
        rooms[1][1].description = String::from(
            "You stand at the foot of a tall mountain. The path winds upward, looking quite steep. An old man rests against a large rock.",
        );
        rooms[1][1].items.push(String::from("climbing rope"));
        rooms[1][1].npc = Some(String::from("old man"));

        // Mountain Slope
        rooms[1][2].description = String::from(
            "A platform on the mountainside offers a view of the forest below. The air is thin, making breathing somewhat difficult.",
        );
        rooms[1][2].items.push(String::from("strange crystal"));

        // Mountain Peak
        rooms[2][2].description = String::from(
            "The mountain peak is shrouded in clouds, limiting visibility. An ancient temple stands here, with strange runes at its entrance.",
        );
        rooms[2][2].items.push(String::from("ancient key"));
        rooms[2][2].npc = Some(String::from("mountain sage"));

        // Temple Interior
        rooms[2][3].description = String::from(
            "The temple interior is dimly lit, with walls covered in ancient writings and patterns. In the center is an altar with a glowing object on it.",
        );
        rooms[2][3].items.push(String::from("glowing orb"));
        rooms[2][3].items.push(String::from("ceremonial dagger"));

        // Underground Passage
        rooms[2][1].description = String::from(
            "A hidden entrance to an underground passage. Steps lead down into darkness. Several torches hang on the walls.",
        );
        rooms[2][1].items.push(String::from("torch"));

        // Underground Cave
        rooms[3][1].description = String::from(
            "A damp underground cave with water dripping from the ceiling. In the center is a small pond, its surface as still as a mirror.",
        );
        rooms[3][1].items.push(String::from("healing herb"));
        rooms[3][1].items.push(String::from("mysterious potion"));
        rooms[3][1].npc = Some(String::from("cave dweller"));

        // Treasure Room
        rooms[3][2].description = String::from(
            "A hidden treasure room! The walls are embedded with gems, and gold coins and jewels are scattered on the floor. A huge chest sits in the corner.",
        );
        rooms[3][2].items.push(String::from("golden key"));
        rooms[3][2].items.push(String::from("jeweled crown"));
        rooms[3][2].items.push(String::from("magic scroll"));

        // Riverside
        rooms[0][2].description = String::from(
            "A clear river flows down from the mountain. The water is cool, with fish swimming in it. There's a simple dock by the riverbank.",
        );
        rooms[0][2].items.push(String::from("fishing rod"));
        rooms[0][2].npc = Some(String::from("fisherman"));

        // Small Village
        rooms[0][3].description = String::from(
            "A peaceful small village. Several thatched cottages are scattered along the road, with villagers going about their daily activities. There's a well in the center.",
        );
        rooms[0][3].items.push(String::from("bread"));
        rooms[0][3].items.push(String::from("water flask"));
        rooms[0][3].npc = Some(String::from("village elder"));

        // Blacksmith's Shop
        rooms[1][3].description = String::from(
            "The village blacksmith's shop. The forge is blazing as the blacksmith crafts weapons. Various tools and finished weapons hang on the walls.",
        );
        rooms[1][3].items.push(String::from("steel sword"));
        rooms[1][3].npc = Some(String::from("blacksmith"));

        // Wasteland
        rooms[4][0].description = String::from(
            "A barren land with almost no vegetation. The ground is cracked, with tumbleweeds occasionally passing by. Something seems to be moving in the distance.",
        );
        rooms[4][0].items.push(String::from("cracked bone"));
        rooms[4][0].npc = Some(String::from("wandering merchant"));

        // Desert Oasis
        rooms[4][1].description = String::from(
            "An oasis in the desert. Palm trees surround a small pool, providing rare shade. Several camels rest nearby.",
        );
        rooms[4][1].items.push(String::from("exotic fruit"));
        rooms[4][1].items.push(String::from("waterskin"));
        rooms[4][1].npc = Some(String::from("desert nomad"));

        // Ancient Ruins
        rooms[4][2].description = String::from(
            "Ancient ruins in the desert. Half-buried stone columns and broken walls tell of past glory. Mysterious symbols mark the entrance.",
        );
        rooms[4][2].items.push(String::from("ancient tablet"));
        rooms[4][2].items.push(String::from("scarab amulet"));

        // Ruins Interior
        rooms[4][3].description = String::from(
            "The main hall of the ruins. Tall stone columns support the ceiling, and murals on the walls tell ancient stories. A sarcophagus sits in the center.",
        );
        rooms[4][3].items.push(String::from("pharaoh's mask"));
        rooms[4][3].npc = Some(String::from("guardian spirit"));

        // Mysterious Portal
        rooms[4][4].description = String::from(
            "A mysterious portal glowing with blue light stands in the deepest part of the ruins. Energy fluctuations distort the air, seemingly leading to another world.",
        );
        rooms[4][4].items.push(String::from("dimensional key"));

        World { rooms }
    }

    // get the current room based on the player's position
    pub fn current_room(&self, position: (i32, i32)) -> Option<&Room> {
        if position.0 < 0 || position.1 < 0 {
            return None;
        }
        self.rooms
            .get(position.0 as usize)
            .and_then(|row| row.get(position.1 as usize))
    }

    // get the current room based on the player's position
    pub fn current_room_mut(&mut self, position: (i32, i32)) -> Option<&mut Room> {
        if position.0 < 0 || position.1 < 0 {
            return None;
        }
        self.rooms
            .get_mut(position.0 as usize)
            .and_then(|row| row.get_mut(position.1 as usize))
    }

    // remove an item from the current room
    pub fn remove_item_from_current_room(
        &mut self,
        position: (i32, i32),
        item: &str,
    ) -> Option<String> {
        let room = self.current_room_mut(position)?;
        println!("room: {:?}", room);
        if let Some(index) = room.items.iter().position(|i| i == &item) {
            Some(room.items.remove(index))
        } else {
            None
        }
    }

    // add an item to the current room
    pub fn add_item_to_current_room(
        &mut self,
        position: (i32, i32),
        item: String,
    ) -> Result<(), String> {
        if let Some(room) = self.current_room_mut(position) {
            room.items.push(item);
            Ok(())
        } else {
            Err(format!("Room not found"))
        }
    }

    // Extended NPC dialogue system
    pub fn get_npc_response(&self, position: (i32, i32), npc: &str) -> Option<&str> {
        let room = self.current_room(position)?;

        println!(
            "Debug - Room NPC: '{:?}', Requested NPC: '{}'",
            room.npc, npc
        );

        // Use contains check instead of exact match
        match &room.npc {
            Some(name) if name.to_lowercase().contains(&npc.to_lowercase()) => {
                match name.as_str() {
                    "old man" => Some(
                        "Young one, I was once an adventurer like you, until I took an arrow to the knee... The temple at the mountain peak holds many secrets waiting to be discovered.",
                    ),
                    "old hunter" => Some(
                        "This forest has become dangerous lately. If you're going to venture out, take my lantern - you'll need it at night.",
                    ),
                    "forest elf" => Some(
                        "Welcome to the elven forest. We live in harmony with nature. If you seek power, try visiting the temple on the mountain.",
                    ),
                    "mountain sage" => Some(
                        "The temple door can only be opened with the ancient key. Legend says powerful magical items are hidden inside.",
                    ),
                    "cave dweller" => Some(
                        "Deep in the cave is a secret chamber containing treasures of an ancient civilization. But beware of the guardian...",
                    ),
                    "fisherman" => Some(
                        "The fish are particularly active today. If you have a fishing rod, you might try your luck. I've heard someone caught a treasure in the river.",
                    ),
                    "village elder" => Some(
                        "Our village is peaceful, but there have been rumors of strange creatures appearing in the wasteland. Be careful, young one.",
                    ),
                    "blacksmith" => Some(
                        "This steel sword is my masterpiece! You can easily defeat those beasts with it. If you find rare metals, I can forge even better weapons for you.",
                    ),
                    "wandering merchant" => Some(
                        "I have all sorts of curious goods from distant lands. Do you have anything to trade? I'm particularly interested in those glowing crystals.",
                    ),
                    "desert nomad" => Some(
                        "Carry plenty of water when traveling in the desert. Those ruins over there have existed for thousands of years, said to contain the pharaoh's treasure.",
                    ),
                    "guardian spirit" => Some(
                        "I have guarded this ruin for a thousand years. Only a true hero can pass the final trial and receive the pharaoh's blessing.",
                    ),
                    _ => Some("Hello, traveler. May your adventures be filled with wonder."),
                }
            }
            _ => None,
        }
    }
}
