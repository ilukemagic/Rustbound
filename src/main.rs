mod player;
mod world;

use player::Player;
use std::io;
use world::World;

fn main() {
    // initialize the world
    let mut plyaer = Player::new("Adventurer".to_string());
    let world = World::new();

    // game loop
    loop {
        if let Some(room) = world.current_room(plyaer.position) {
            println!("{}", room.description);
            if !room.items.is_empty() {
                println!("You see: {}", room.items.join(", "));
            }
            if let Some(npc) = &room.npc {
                println!("You see: {} here", npc);
            }

            // get player input
            let mut input = String::new();
            println!("\nWhat do you want to do?");
            // todo: handle errors
            io::stdin().read_line(&mut input).unwrap();

            if input.trim() == "quit" {
                println!("Goodbye!");
                break;
            }
        }
    }
}
