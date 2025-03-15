mod command;
mod item;
mod location;
mod npc;
mod player;
mod world;

use command::Command;
use location::Location;
use player::Player;
use std::io;
use world::World;

fn main() -> io::Result<()> {
    // initialize the world
    let mut player = Player::new("Adventurer".to_string());
    let mut world = World::new();

    println!("Welcome to Rustbound, {}!", player.get_name());
    println!("Type 'help' for a list of commands.");

    // game loop
    loop {
        if let Some(room) = world.current_room(player.position) {
            // 获取当前位置的枚举
            let current_location = player.current_location();

            // 显示位置名称
            println!("\n[{}]", location_name(&current_location));
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
                    } else {
                        // 显示移动成功的信息
                        let new_location = player.current_location();
                        println!(
                            "You moved {} to {}.",
                            direction,
                            location_name(&new_location)
                        );
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
                Command::Inventory => println!("{}", player.display_inventory()),
                Command::Drop(item) => match player.remove_from_inventory(&item) {
                    Some(dropped_item) => {
                        if let Err(e) =
                            world.add_item_to_current_room(player.position, dropped_item.clone())
                        {
                            println!("Error: {}", e);
                            player.add_to_inventory(dropped_item);
                        } else {
                            println!("You dropped the {}", dropped_item);
                        }
                    }
                    None => println!("You don't have {}", item),
                },
                Command::Name(new_name) => {
                    let old_name = player.get_name().to_string();
                    player.set_name(new_name.clone());
                    println!(
                        "Your name has been changed from {} to {}",
                        old_name, new_name
                    );
                }
                Command::Help => {
                    display_help();
                }
                Command::Quit => {
                    println!("Goodbye!");
                    break;
                }
                Command::Invalid => {
                    println!("Available commands:");
                    println!(
                        "go [direction], take [item], drop [item], use [item], talk to [npc], inventory, name [new name], quit"
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

// 辅助函数：根据Location枚举返回位置名称
fn location_name(location: &Location) -> String {
    location.to_string()
}

fn display_help() {
    println!("\n===== RUSTBOUND - TEXT ADVENTURE GAME =====");
    println!(
        "Welcome to Rustbound! Explore the world, collect items, and interact with characters."
    );
    println!("\nAVAILABLE COMMANDS:");
    println!("  go [direction]    - Move in a direction (north, south, east, west)");
    println!("  take [item]       - Pick up an item from the current room");
    println!("  drop [item]       - Drop an item from your inventory");
    println!("  use [item]        - Use an item from your inventory");
    println!("  talk to [npc]     - Talk to a character in the current room");
    println!("  inventory         - Check your inventory (shortcut: 'i')");
    println!("  name [new name]   - Change your character's name");
    println!("  help              - Display this help message (shortcut: 'h')");
    println!("  quit              - Exit the game");
    println!("\nTIPS:");
    println!("- Explore different rooms by using 'go' in various directions");
    println!("- Collect items with 'take' and use them with 'use'");
    println!("- Talk to characters to get information and quests");
    println!("- Some items have special effects when used in certain locations");
    println!("=======================================\n");
}
