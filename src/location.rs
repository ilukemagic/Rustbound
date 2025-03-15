#[derive(Debug, Clone, PartialEq)]
pub enum Location {
    MysteriousForest,
    DeepForest,
    ForestCabin,
    MountainBase,
    MountainSlope,
    MountainPeak,
    TempleInterior,
    UndergroundPassage,
    UndergroundCave,
    TreasureRoom,
    Riverside,
    SmallVillage,
    BlacksmithShop,
    Wasteland,
    DesertOasis,
    AncientRuins,
    RuinsInterior,
    MysteriousPortal,
    EmptyArea,
}

impl Location {
    // get location coordinates
    pub fn get_coordinates(&self) -> (i32, i32) {
        match self {
            Location::MysteriousForest => (0, 0),
            Location::DeepForest => (0, 1),
            Location::ForestCabin => (1, 0),
            Location::MountainBase => (1, 1),
            Location::MountainSlope => (1, 2),
            Location::MountainPeak => (2, 2),
            Location::TempleInterior => (2, 3),
            Location::UndergroundPassage => (2, 1),
            Location::UndergroundCave => (3, 1),
            Location::TreasureRoom => (3, 2),
            Location::Riverside => (0, 2),
            Location::SmallVillage => (0, 3),
            Location::BlacksmithShop => (1, 3),
            Location::Wasteland => (4, 0),
            Location::DesertOasis => (4, 1),
            Location::AncientRuins => (4, 2),
            Location::RuinsInterior => (4, 3),
            Location::MysteriousPortal => (4, 4),
            Location::EmptyArea => (-1, -1), // invalid position
        }
    }

    // from coordinates to location
    pub fn from_coordinates(x: i32, y: i32) -> Self {
        match (x, y) {
            (0, 0) => Location::MysteriousForest,
            (0, 1) => Location::DeepForest,
            (1, 0) => Location::ForestCabin,
            (1, 1) => Location::MountainBase,
            (1, 2) => Location::MountainSlope,
            (2, 2) => Location::MountainPeak,
            (2, 3) => Location::TempleInterior,
            (2, 1) => Location::UndergroundPassage,
            (3, 1) => Location::UndergroundCave,
            (3, 2) => Location::TreasureRoom,
            (0, 2) => Location::Riverside,
            (0, 3) => Location::SmallVillage,
            (1, 3) => Location::BlacksmithShop,
            (4, 0) => Location::Wasteland,
            (4, 1) => Location::DesertOasis,
            (4, 2) => Location::AncientRuins,
            (4, 3) => Location::RuinsInterior,
            (4, 4) => Location::MysteriousPortal,
            _ => Location::EmptyArea,
        }
    }

    // get location description
    pub fn get_description(&self) -> &'static str {
        match self {
            Location::MysteriousForest => {
                "Entrance to the mysterious forest. Moonlight filters through the dense leaves, creating dappled shadows on the ground. Low growls of wild beasts can be heard in the distance."
            }
            Location::DeepForest => {
                "Deep in the forest. The trees are denser here, blocking almost all sunlight. You can hear birds singing and leaves rustling."
            }
            Location::ForestCabin => {
                "A dilapidated wooden cabin sits at the edge of the forest. The door is half-open, and there are signs that someone lives here."
            }
            Location::MountainBase => {
                "You stand at the foot of a tall mountain. The path winds upward, looking quite steep. An old man rests against a large rock."
            }
            Location::MountainSlope => {
                "A platform on the mountainside offers a view of the forest below. The air is thin, making breathing somewhat difficult."
            }
            Location::MountainPeak => {
                "The mountain peak is shrouded in clouds, limiting visibility. An ancient temple stands here, with strange runes at its entrance."
            }
            Location::TempleInterior => {
                "The temple interior is dimly lit, with walls covered in ancient writings and patterns. In the center is an altar with a glowing object on it."
            }
            Location::UndergroundPassage => {
                "A hidden entrance to an underground passage. Steps lead down into darkness. Several torches hang on the walls."
            }
            Location::UndergroundCave => {
                "A damp underground cave with water dripping from the ceiling. In the center is a small pond, its surface as still as a mirror."
            }
            Location::TreasureRoom => {
                "A hidden treasure room! The walls are embedded with gems, and gold coins and jewels are scattered on the floor. A huge chest sits in the corner."
            }
            Location::Riverside => {
                "A clear river flows down from the mountain. The water is cool, with fish swimming in it. There's a simple dock by the riverbank."
            }
            Location::SmallVillage => {
                "A peaceful small village. Several thatched cottages are scattered along the road, with villagers going about their daily activities. There's a well in the center."
            }
            Location::BlacksmithShop => {
                "The village blacksmith's shop. The forge is blazing as the blacksmith crafts weapons. Various tools and finished weapons hang on the walls."
            }
            Location::Wasteland => {
                "A barren land with almost no vegetation. The ground is cracked, with tumbleweeds occasionally passing by. Something seems to be moving in the distance."
            }
            Location::DesertOasis => {
                "An oasis in the desert. Palm trees surround a small pool, providing rare shade. Several camels rest nearby."
            }
            Location::AncientRuins => {
                "Ancient ruins in the desert. Half-buried stone columns and broken walls tell of past glory. Mysterious symbols mark the entrance."
            }
            Location::RuinsInterior => {
                "The main hall of the ruins. Tall stone columns support the ceiling, and murals on the walls tell ancient stories. A sarcophagus sits in the center."
            }
            Location::MysteriousPortal => {
                "A mysterious portal glowing with blue light stands in the deepest part of the ruins. Energy fluctuations distort the air, seemingly leading to another world."
            }
            Location::EmptyArea => "An empty area with nothing of interest.",
        }
    }

    // 将Location转换为字符串
    pub fn to_string(&self) -> String {
        match self {
            Location::MysteriousForest => "Mysterious Forest".to_string(),
            Location::DeepForest => "Deep Forest".to_string(),
            Location::ForestCabin => "Forest Cabin".to_string(),
            Location::MountainBase => "Mountain Base".to_string(),
            Location::MountainSlope => "Mountain Slope".to_string(),
            Location::MountainPeak => "Mountain Peak".to_string(),
            Location::TempleInterior => "Temple Interior".to_string(),
            Location::UndergroundPassage => "Underground Passage".to_string(),
            Location::UndergroundCave => "Underground Cave".to_string(),
            Location::TreasureRoom => "Treasure Room".to_string(),
            Location::Riverside => "Riverside".to_string(),
            Location::SmallVillage => "Small Village".to_string(),
            Location::BlacksmithShop => "Blacksmith's Shop".to_string(),
            Location::Wasteland => "Wasteland".to_string(),
            Location::DesertOasis => "Desert Oasis".to_string(),
            Location::AncientRuins => "Ancient Ruins".to_string(),
            Location::RuinsInterior => "Ruins Interior".to_string(),
            Location::MysteriousPortal => "Mysterious Portal".to_string(),
            Location::EmptyArea => "Unknown Area".to_string(),
        }
    }
}
