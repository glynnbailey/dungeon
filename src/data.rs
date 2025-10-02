use serde::{Deserialize, Serialize};

pub struct GameData {
    pub actors: Vec<ActorType>,
    pub items: Vec<ItemType>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum DataType {
    #[serde(rename = "actor")]
    Actor(ActorType),
    #[serde(rename = "item")]
    Item(ItemType),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActorType {
    id: String,
    name: String,
    glyph: char,
    color: (u8, u8, u8),
    base_health: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemType {
    id: String,
    name: String,
}

pub fn load_data(path: &str) -> GameData {
    let mut actors = Vec::new();
    let mut items = Vec::new();

    for file in std::fs::read_dir(path).unwrap() {
        let file = file.unwrap();
        let path = file.path();
        if path.extension().and_then(|s| s.to_str()) == Some("yaml") {
            let content = std::fs::read_to_string(&path).unwrap();
            let data_types: Vec<DataType> = serde_yaml::from_str(&content).unwrap();
            for data_type in data_types {
                match data_type {
                    DataType::Actor(actor) => actors.push(actor),
                    DataType::Item(item) => items.push(item),
                }
            }
        }
    }

    GameData {
        actors,
        items,
    }
}
