#[derive(Debug)]
pub enum Command {
    Go(String),
    Take(String),
    Talk(String),
    Use(String),
    Inventory,
    Drop(String),
    Name(String),
    Quit,
    Invalid,
}

impl Command {
    pub fn parse(input: &str) -> Self {
        let lower = input.trim().to_lowercase();
        let mut parts = lower.split_whitespace();

        match parts.next() {
            Some("go") => {
                if let Some(direction) = parts.next() {
                    Command::Go(direction.to_string())
                } else {
                    Command::Invalid
                }
            }
            Some("take") => {
                let item_name = parts.collect::<Vec<&str>>().join(" ");
                if !item_name.is_empty() {
                    Command::Take(item_name)
                } else {
                    Command::Invalid
                }
            }
            Some("talk") => {
                if parts.next() == Some("to") {
                    let npc_name = parts.collect::<Vec<&str>>().join(" ");
                    if !npc_name.is_empty() {
                        Command::Talk(npc_name)
                    } else {
                        Command::Invalid
                    }
                } else {
                    Command::Invalid
                }
            }
            Some("inventory") => Command::Inventory,
            Some("drop") => {
                let item_name = parts.collect::<Vec<&str>>().join(" ");
                if !item_name.is_empty() {
                    Command::Drop(item_name)
                } else {
                    Command::Invalid
                }
            }
            Some("name") => {
                let name = parts.collect::<Vec<&str>>().join(" ");
                if !name.is_empty() {
                    Command::Name(name)
                } else {
                    Command::Invalid
                }
            }
            Some("quit") => Command::Quit,
            Some("use") => {
                let item_name = parts.collect::<Vec<&str>>().join(" ");
                if !item_name.is_empty() {
                    Command::Use(item_name)
                } else {
                    Command::Invalid
                }
            }
            _ => Command::Invalid,
        }
    }
}
