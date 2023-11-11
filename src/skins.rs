use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{Client, Error};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkinLine {
    pub id: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Rarity {
    pub region: String,
    pub rarity: i8,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Description {
    pub region: String,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chroma {
    pub id: u32,
    pub name: String,
    #[serde(rename = "chromaPath")]
    pub chroma_path: String,
    pub colors: Vec<String>,
    pub descriptions: Vec<Description>,
    pub rarities: Vec<Rarity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skin {
    pub id: u32,
    #[serde(rename = "isBase")]
    pub is_base: bool,
    pub name: String,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "uncenteredSplashPath")]
    pub uncentered_splash_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    #[serde(rename = "loadScreenPath")]
    pub load_screen_path: String,
    #[serde(rename = "loadScreenVintagePath")]
    pub load_screen_vintage_path: Option<String>,
    #[serde(rename = "skinType")]
    pub skin_type: String,
    pub rarity: String,
    #[serde(rename = "isLegacy")]
    pub is_legacy: bool,
    #[serde(rename = "splashVideoPath")]
    pub splash_video_path: Option<String>,
    #[serde(rename = "collectionSplashVideoPath")]
    pub collection_splash_video_path: Option<String>,
    #[serde(rename = "featuresText")]
    pub features_text: Option<String>,
    #[serde(rename = "chromaPath")]
    pub chroma_path: Option<String>,
    pub chromas: Option<Vec<Chroma>>,
    #[serde(rename = "regionRarityId")]
    pub region_rarity_id: u8,
    #[serde(rename = "skinLines")]
    pub skin_lines: Option<Vec<SkinLine>>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Tier {
    pub id: i64,
    pub name: String,
    pub stage: i64,
    pub description: String,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "uncenteredSplashPath")]
    pub uncentered_splash_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    #[serde(rename = "loadScreenPath")]
    pub load_screen_path: String,
    #[serde(rename = "loadScreenVintagePath")]
    pub load_screen_vintage_path: String,
    #[serde(rename = "shortName")]
    pub short_name: String,
    #[serde(rename = "splashVideoPath")]
    pub splash_video_path: String,
    #[serde(rename = "collectionSplashVideoPath")]
    pub collection_splash_video_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct DescriptionInfo {
    pub title: String,
    pub description: String,
    #[serde(rename = "iconPath")]
    pub icon_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct QuestSkinInfo {
    pub name: String,
    #[serde(rename = "collectionDescription")]
    pub collection_description: String,
    #[serde(rename = "descriptionInfo")]
    pub description_info: Vec<DescriptionInfo>,
    #[serde(rename = "splashPath")]
    pub splash_path: String,
    #[serde(rename = "uncenteredSplashPath")]
    pub uncentered_splash_path: String,
    #[serde(rename = "tilePath")]
    pub tile_path: String,
    #[serde(rename = "collectionCardPath")]
    pub collection_card_path: String,
    pub tiers: Vec<Tier>,
}

pub async fn get(client: Client) -> Result<HashMap<u32, Skin>, Error> {
    client.get("https://raw.communitydragon.org/latest/plugins/rcp-be-lol-game-data/global/default/v1/skins.json").await
}