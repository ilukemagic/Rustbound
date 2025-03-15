#[derive(Debug, Clone, PartialEq)]
pub enum Npc {
    OldMan,
    OldHunter,
    ForestElf,
    MountainSage,
    CaveDweller,
    Fisherman,
    VillageElder,
    Blacksmith,
    WanderingMerchant,
    DesertNomad,
    GuardianSpirit,
}

impl Npc {
    // from string to enum
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "old man" => Some(Npc::OldMan),
            "old hunter" => Some(Npc::OldHunter),
            "forest elf" => Some(Npc::ForestElf),
            "mountain sage" => Some(Npc::MountainSage),
            "cave dweller" => Some(Npc::CaveDweller),
            "fisherman" => Some(Npc::Fisherman),
            "village elder" => Some(Npc::VillageElder),
            "blacksmith" => Some(Npc::Blacksmith),
            "wandering merchant" => Some(Npc::WanderingMerchant),
            "desert nomad" => Some(Npc::DesertNomad),
            "guardian spirit" => Some(Npc::GuardianSpirit),
            _ => None,
        }
    }

    // from enum to string
    pub fn to_string(&self) -> String {
        match self {
            Npc::OldMan => "old man".to_string(),
            Npc::OldHunter => "old hunter".to_string(),
            Npc::ForestElf => "forest elf".to_string(),
            Npc::MountainSage => "mountain sage".to_string(),
            Npc::CaveDweller => "cave dweller".to_string(),
            Npc::Fisherman => "fisherman".to_string(),
            Npc::VillageElder => "village elder".to_string(),
            Npc::Blacksmith => "blacksmith".to_string(),
            Npc::WanderingMerchant => "wandering merchant".to_string(),
            Npc::DesertNomad => "desert nomad".to_string(),
            Npc::GuardianSpirit => "guardian spirit".to_string(),
        }
    }

    // get npc dialogue
    pub fn get_dialogue(&self) -> &'static str {
        match self {
            Npc::OldMan => {
                "Young one, I was once an adventurer like you, until I took an arrow to the knee... The temple at the mountain peak holds many secrets waiting to be discovered."
            }
            Npc::OldHunter => {
                "This forest has become dangerous lately. If you're going to venture out, take my lantern - you'll need it at night."
            }
            Npc::ForestElf => {
                "Welcome to the elven forest. We live in harmony with nature. If you seek power, try visiting the temple on the mountain."
            }
            Npc::MountainSage => {
                "The temple door can only be opened with the ancient key. Legend says powerful magical items are hidden inside."
            }
            Npc::CaveDweller => {
                "Deep in the cave is a secret chamber containing treasures of an ancient civilization. But beware of the guardian..."
            }
            Npc::Fisherman => {
                "The fish are particularly active today. If you have a fishing rod, you might try your luck. I've heard someone caught a treasure in the river."
            }
            Npc::VillageElder => {
                "Our village is peaceful, but there have been rumors of strange creatures appearing in the wasteland. Be careful, young one."
            }
            Npc::Blacksmith => {
                "This steel sword is my masterpiece! You can easily defeat those beasts with it. If you find rare metals, I can forge even better weapons for you."
            }
            Npc::WanderingMerchant => {
                "I have all sorts of curious goods from distant lands. Do you have anything to trade? I'm particularly interested in those glowing crystals."
            }
            Npc::DesertNomad => {
                "Carry plenty of water when traveling in the desert. Those ruins over there have existed for thousands of years, said to contain the pharaoh's treasure."
            }
            Npc::GuardianSpirit => {
                "I have guarded this ruin for a thousand years. Only a true hero can pass the final trial and receive the pharaoh's blessing."
            }
        }
    }
}
