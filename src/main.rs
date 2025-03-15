mod command;
mod player;
mod world;

use command::Command;
use player::Player;
use std::io;
use world::World;

fn main() -> io::Result<()> {
    // initialize the world
    let mut player = Player::new("Adventurer".to_string());
    let mut world = World::new();

    // game loop
    loop {
        if let Some(room) = world.current_room(player.position) {
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
            io::stdin().read_line(&mut input)?;

            match Command::parse(&input) {
                Command::Go(direction) => {
                    if let Err(e) = player.move_to(&direction) {
                        println!("{}", e);
                    }
                }
                Command::Take(item) => {
                    match world.remove_item_from_current_room(player.position, &item) {
                        Some(item) => {
                            player.add_to_inventory(item.clone());
                            println!("You took the {}", item);
                        }
                        None => println!("There's no {} here", item),
                    }
                }
                Command::Talk(npc) => match world.get_npc_response(player.position, &npc) {
                    Some(response) => println!("{}: {}", npc, response),
                    None => println!("There's no one here to talk to"),
                },
                Command::Use(item) => match player.use_item(&item) {
                    Ok(message) => println!("{}", message),
                    Err(message) => println!("Error: {}", message),
                },
                Command::Quit => {
                    println!("Goodbye!");
                    break;
                }
                Command::Invalid => {
                    println!("Available commands:");
                    println!(
                        "go [direction], take [item], use [item], talk to [npc], inventory, quit"
                    );
                }
            }

            if input.trim() == "quit" {
                println!("Goodbye!");
                break;
            }
        }
    }
    Ok(())
}
