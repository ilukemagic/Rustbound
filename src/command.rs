#[derive(Debug)]
pub enum Command {
    Go(String),
    Take(String),
    Talk(String),
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
                if let Some(item) = parts.next() {
                    Command::Take(item.to_string())
                } else {
                    Command::Invalid
                }
            }
            Some("talk") => {
                if parts.next() == Some("to") {
                    if let Some(npc) = parts.next() {
                        Command::Talk(npc.to_string())
                    } else {
                        Command::Invalid
                    }
                } else {
                    Command::Invalid
                }
            }
            Some("quit") => Command::Quit,
            _ => Command::Invalid,
        }
    }
}
