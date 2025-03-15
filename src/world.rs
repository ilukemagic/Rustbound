use crate::item::Item;
use crate::location::Location;
use crate::npc::Npc;

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

        // 使用Location枚举初始化房间
        let locations = [
            Location::MysteriousForest,
            Location::DeepForest,
            Location::ForestCabin,
            Location::MountainBase,
            Location::MountainSlope,
            Location::MountainPeak,
            Location::TempleInterior,
            Location::UndergroundPassage,
            Location::UndergroundCave,
            Location::TreasureRoom,
            Location::Riverside,
            Location::SmallVillage,
            Location::BlacksmithShop,
            Location::Wasteland,
            Location::DesertOasis,
            Location::AncientRuins,
            Location::RuinsInterior,
            Location::MysteriousPortal,
        ];

        // 初始化每个位置
        for location in locations.iter() {
            let (x, y) = location.get_coordinates();
            if x >= 0 && y >= 0 {
                rooms[x as usize][y as usize].description = location.get_description().to_string();
            }
        }

        // 添加物品和NPC
        World::add_items_and_npcs(&mut rooms);

        World { rooms }
    }

    // 添加物品和NPC的辅助函数
    fn add_items_and_npcs(rooms: &mut Vec<Vec<Room>>) {
        // 神秘森林入口
        let (x, y) = Location::MysteriousForest.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::RustySword.to_string());
        rooms[x as usize][y as usize]
            .items
            .push(Item::SmallPotion.to_string());

        // 深林
        let (x, y) = Location::DeepForest.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::WoodenShield.to_string());
        rooms[x as usize][y as usize].npc = Some(Npc::ForestElf.to_string());

        // 森林小屋
        let (x, y) = Location::ForestCabin.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::OldMap.to_string());
        rooms[x as usize][y as usize]
            .items
            .push(Item::Lantern.to_string());
        rooms[x as usize][y as usize].npc = Some(Npc::OldHunter.to_string());

        // 山脚
        let (x, y) = Location::MountainBase.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::ClimbingRope.to_string());
        rooms[x as usize][y as usize].npc = Some(Npc::OldMan.to_string());

        // 山坡
        let (x, y) = Location::MountainSlope.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::StrangeCrystal.to_string());

        // 山顶
        let (x, y) = Location::MountainPeak.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::AncientKey.to_string());
        rooms[x as usize][y as usize].npc = Some(Npc::MountainSage.to_string());

        // 神庙内部
        let (x, y) = Location::TempleInterior.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::GlowingOrb.to_string());
        rooms[x as usize][y as usize]
            .items
            .push(Item::CeremonialDagger.to_string());

        // 地下通道
        let (x, y) = Location::UndergroundPassage.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::Torch.to_string());

        // 地下洞穴
        let (x, y) = Location::UndergroundCave.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::HealingHerb.to_string());
        rooms[x as usize][y as usize]
            .items
            .push(Item::MysteriousPotion.to_string());
        rooms[x as usize][y as usize].npc = Some(Npc::CaveDweller.to_string());

        // 宝藏室
        let (x, y) = Location::TreasureRoom.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::GoldenKey.to_string());
        rooms[x as usize][y as usize]
            .items
            .push(Item::JeweledCrown.to_string());
        rooms[x as usize][y as usize]
            .items
            .push(Item::MagicScroll.to_string());

        // 河边
        let (x, y) = Location::Riverside.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::FishingRod.to_string());
        rooms[x as usize][y as usize].npc = Some(Npc::Fisherman.to_string());

        // 小村庄
        let (x, y) = Location::SmallVillage.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::Bread.to_string());
        rooms[x as usize][y as usize]
            .items
            .push(Item::WaterFlask.to_string());
        rooms[x as usize][y as usize].npc = Some(Npc::VillageElder.to_string());

        // 铁匠铺
        let (x, y) = Location::BlacksmithShop.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::SteelSword.to_string());
        rooms[x as usize][y as usize].npc = Some(Npc::Blacksmith.to_string());

        // 荒地
        let (x, y) = Location::Wasteland.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::CrackedBone.to_string());
        rooms[x as usize][y as usize].npc = Some(Npc::WanderingMerchant.to_string());

        // 沙漠绿洲
        let (x, y) = Location::DesertOasis.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::ExoticFruit.to_string());
        rooms[x as usize][y as usize]
            .items
            .push(Item::Waterskin.to_string());
        rooms[x as usize][y as usize].npc = Some(Npc::DesertNomad.to_string());

        // 古代遗迹
        let (x, y) = Location::AncientRuins.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::AncientTablet.to_string());
        rooms[x as usize][y as usize]
            .items
            .push(Item::ScarabAmulet.to_string());

        // 遗迹内部
        let (x, y) = Location::RuinsInterior.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::PharaohsMask.to_string());
        rooms[x as usize][y as usize].npc = Some(Npc::GuardianSpirit.to_string());

        // 神秘传送门
        let (x, y) = Location::MysteriousPortal.get_coordinates();
        rooms[x as usize][y as usize]
            .items
            .push(Item::DimensionalKey.to_string());
    }

    // get the current room based on the player's position
    pub fn current_room(&self, position: (i32, i32)) -> Option<&Room> {
        // 使用from_coordinates检查位置是否有效
        let location = Location::from_coordinates(position.0, position.1);

        // 如果是EmptyArea，表示无效位置
        if matches!(location, Location::EmptyArea) {
            return None;
        }

        // 否则，返回对应的房间
        if position.0 < 0 || position.1 < 0 {
            return None;
        }
        self.rooms
            .get(position.0 as usize)
            .and_then(|row| row.get(position.1 as usize))
    }

    // get the current room based on the player's position
    pub fn current_room_mut(&mut self, position: (i32, i32)) -> Option<&mut Room> {
        // 使用from_coordinates检查位置是否有效
        let location = Location::from_coordinates(position.0, position.1);

        // 如果是EmptyArea，表示无效位置
        if matches!(location, Location::EmptyArea) {
            return None;
        }

        // 否则，返回对应的房间
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

    // 获取NPC对话
    pub fn get_npc_response(&self, position: (i32, i32), npc_name: &str) -> Option<&str> {
        let room = self.current_room(position)?;

        println!(
            "Debug - Room NPC: '{:?}', Requested NPC: '{}'",
            room.npc, npc_name
        );

        // 使用contains检查而不是精确匹配
        match &room.npc {
            Some(name) if name.to_lowercase().contains(&npc_name.to_lowercase()) => {
                // 尝试将字符串转换为Npc枚举
                if let Some(npc) = Npc::from_string(name) {
                    Some(npc.get_dialogue())
                } else {
                    Some("Hello, traveler. May your adventures be filled with wonder.")
                }
            }
            _ => None,
        }
    }
}
