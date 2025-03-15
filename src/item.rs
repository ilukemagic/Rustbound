#[derive(Debug, Clone, PartialEq)]
pub enum Item {
    // potion item
    SmallPotion,
    MysteriousPotion,
    HealingHerb,

    // weapon item
    RustySword,
    SteelSword,
    WoodenShield,
    CeremonialDagger,

    // lighting item
    Lantern,
    Torch,

    // tool item
    ClimbingRope,
    FishingRod,

    // key item
    AncientKey,
    GoldenKey,
    DimensionalKey,

    // magic item
    GlowingOrb,
    StrangeCrystal,
    MagicScroll,

    // ancient item
    AncientTablet,
    ScarabAmulet,
    PharaohsMask,
    JeweledCrown,

    // food and drink
    Bread,
    WaterFlask,
    ExoticFruit,
    Waterskin,

    // other
    OldMap,
    CrackedBone,
}

impl Item {
    // from string to enum
    pub fn from_string(s: &str) -> Option<Self> {
        match s {
            "small potion" => Some(Item::SmallPotion),
            "mysterious potion" => Some(Item::MysteriousPotion),
            "healing herb" => Some(Item::HealingHerb),
            "rusty sword" => Some(Item::RustySword),
            "steel sword" => Some(Item::SteelSword),
            "wooden shield" => Some(Item::WoodenShield),
            "lantern" => Some(Item::Lantern),
            "torch" => Some(Item::Torch),
            "climbing rope" => Some(Item::ClimbingRope),
            "ancient key" => Some(Item::AncientKey),
            "golden key" => Some(Item::GoldenKey),
            "dimensional key" => Some(Item::DimensionalKey),
            "glowing orb" => Some(Item::GlowingOrb),
            "ceremonial dagger" => Some(Item::CeremonialDagger),
            "strange crystal" => Some(Item::StrangeCrystal),
            "ancient tablet" => Some(Item::AncientTablet),
            "scarab amulet" => Some(Item::ScarabAmulet),
            "pharaoh's mask" => Some(Item::PharaohsMask),
            "jeweled crown" => Some(Item::JeweledCrown),
            "magic scroll" => Some(Item::MagicScroll),
            "fishing rod" => Some(Item::FishingRod),
            "bread" => Some(Item::Bread),
            "water flask" => Some(Item::WaterFlask),
            "exotic fruit" => Some(Item::ExoticFruit),
            "waterskin" => Some(Item::Waterskin),
            "old map" => Some(Item::OldMap),
            "cracked bone" => Some(Item::CrackedBone),
            _ => None,
        }
    }

    // from enum to string
    pub fn to_string(&self) -> String {
        match self {
            Item::SmallPotion => "small potion".to_string(),
            Item::MysteriousPotion => "mysterious potion".to_string(),
            Item::HealingHerb => "healing herb".to_string(),
            Item::RustySword => "rusty sword".to_string(),
            Item::SteelSword => "steel sword".to_string(),
            Item::WoodenShield => "wooden shield".to_string(),
            Item::Lantern => "lantern".to_string(),
            Item::Torch => "torch".to_string(),
            Item::ClimbingRope => "climbing rope".to_string(),
            Item::AncientKey => "ancient key".to_string(),
            Item::GoldenKey => "golden key".to_string(),
            Item::DimensionalKey => "dimensional key".to_string(),
            Item::GlowingOrb => "glowing orb".to_string(),
            Item::CeremonialDagger => "ceremonial dagger".to_string(),
            Item::StrangeCrystal => "strange crystal".to_string(),
            Item::AncientTablet => "ancient tablet".to_string(),
            Item::ScarabAmulet => "scarab amulet".to_string(),
            Item::PharaohsMask => "pharaoh's mask".to_string(),
            Item::JeweledCrown => "jeweled crown".to_string(),
            Item::MagicScroll => "magic scroll".to_string(),
            Item::FishingRod => "fishing rod".to_string(),
            Item::Bread => "bread".to_string(),
            Item::WaterFlask => "water flask".to_string(),
            Item::ExoticFruit => "exotic fruit".to_string(),
            Item::Waterskin => "waterskin".to_string(),
            Item::OldMap => "old map".to_string(),
            Item::CrackedBone => "cracked bone".to_string(),
        }
    }

    // use item description
    pub fn use_description(&self) -> String {
        match self {
            Item::SmallPotion => "You drink the small potion and feel your energy restored!".to_string(),
            Item::MysteriousPotion => "You drink the mysterious potion and suddenly feel like you can understand animal language!".to_string(),
            Item::HealingHerb => "You use the healing herb and your wounds begin to heal.".to_string(),
            Item::RustySword => "You swing the rusty sword. Though somewhat dull, it can still be used for defense.".to_string(),
            Item::SteelSword => "You swing the sharp steel sword, gleaming in the sunlight. This sword can easily defeat most enemies.".to_string(),
            Item::WoodenShield => "You raise the wooden shield, which provides basic protection.".to_string(),
            Item::Lantern => "You light the lantern, illuminating the dark area. You can now see further.".to_string(),
            Item::Torch => "The torch lights up the surrounding area, dispelling the darkness.".to_string(),
            Item::ClimbingRope => "You use the rope to climb to a hard-to-reach place.".to_string(),
            Item::AncientKey => "This ancient key seems capable of opening an important door.".to_string(),
            Item::GoldenKey => "The golden key shines in your hand, looking like it could open a valuable chest.".to_string(),
            Item::DimensionalKey => "As you hold the dimensional key, you feel energy flowing through your body. It seems capable of activating some kind of teleportation device.".to_string(),
            Item::GlowingOrb => "The glowing orb emits a soft light, and you feel a mysterious power flowing within it.".to_string(),
            Item::CeremonialDagger => "The ceremonial dagger doesn't seem suitable for combat, but it might be useful in certain special rituals.".to_string(),
            Item::StrangeCrystal => "When you touch the strange crystal, it begins to emit pulsating light, and strange images flash through your mind.".to_string(),
            Item::AncientTablet => "You study the ancient tablet, and the symbols seem to tell the story of an ancient civilization.".to_string(),
            Item::ScarabAmulet => "When you wear the scarab amulet, you feel protected by some kind of force.".to_string(),
            Item::PharaohsMask => "The pharaoh's mask exudes an aura of majesty. Wearing it makes you feel like you possess the power of a king.".to_string(),
            Item::JeweledCrown => "The jewel-encrusted crown sparkles in the sunlight. It must be worth a fortune.".to_string(),
            Item::MagicScroll => "You open the magic scroll, and the text begins to glow and float in the air before disappearing. You feel like you've learned something new.".to_string(),
            Item::FishingRod => "You use the fishing rod by the river, waiting patiently. Soon, you catch a fish!".to_string(),
            Item::Bread => "You eat the bread and feel your hunger subside.".to_string(),
            Item::WaterFlask => "You take a sip of water and feel refreshed.".to_string(),
            Item::ExoticFruit => "You taste the exotic fruit, which is both sweet and sour, and very delicious. You feel energized.".to_string(),
            Item::Waterskin => "You take a sip from the waterskin, a valuable resource in the desert.".to_string(),
            Item::OldMap => "You carefully study the old map and discover some location markers you hadn't noticed before. This might help you find hidden treasures.".to_string(),
            Item::CrackedBone => "You examine the cracked bone, wondering what creature it might have belonged to.".to_string(),
        }
    }

    // check if item is consumable
    pub fn is_consumable(&self) -> bool {
        match self {
            Item::SmallPotion
            | Item::MysteriousPotion
            | Item::HealingHerb
            | Item::Bread
            | Item::ExoticFruit => true,
            _ => false,
        }
    }
}
