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
        // create a new world with 3 rooms
        let mut rooms = Vec::with_capacity(3);

        for x in 0..3 {
            let mut row = Vec::with_capacity(3);
            for y in 0..3 {
                row.push(Room {
                    description: format!("You are at ({}, {})", x, y),
                    items: vec![],
                    npc: None,
                })
            }
            rooms.push(row);
        }

        rooms[0][0].description =
            String::from("A dark forest. Moonlight filters through the trees.");
        rooms[0][0].items.push(String::from("rusty sword"));
        rooms[1][1].npc = Some(String::from("old man"));

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
}
