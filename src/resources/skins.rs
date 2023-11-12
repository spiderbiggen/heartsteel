use std::collections::BTreeMap;

use const_format::concatcp;
use serde::{Deserialize, Serialize};

use crate::Error;
use crate::RAW_COMMUNITY_DRAGON_URL;
use crate::version::Game;

const SKINS_PATH: &str = "plugins/rcp-be-lol-game-data/global/default/v1/skins.json";

pub async fn get(client: &reqwest::Client) -> Result<BTreeMap<u32, Skin>, Error> {
    crate::get(client, concatcp!(RAW_COMMUNITY_DRAGON_URL, "/latest/", SKINS_PATH)).await
}

pub async fn get_for_version(client: &reqwest::Client, version: Game) -> Result<BTreeMap<u32, Skin>, Error> {
    crate::get(client, format!("{RAW_COMMUNITY_DRAGON_URL}/{version}/{SKINS_PATH}")).await
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SkinLine {
    pub id: u32,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ChromaRarity {
    /// Currently known variants are:
    /// - ""
    /// - "riot"
    /// - "TENCENT"/"tencent" - *China*?
    /// - "VN" - *Vietnam*?
    /// - "ID" - *Indonesia*?
    /// - "TW" - *Taiwan*?
    pub region: String,
    /// Currently known variants are:
    /// -1..3
    pub rarity: i8,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Description {
    pub region: String,
    pub description: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Chroma {
    pub id: u32,
    pub name: String,
    pub chroma_path: String,
    pub colors: Vec<String>,
    pub descriptions: Vec<Description>,
    pub rarities: Vec<ChromaRarity>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Images {
    pub splash_path: String,
    pub tile_path: String,
    pub uncentered_splash_path: String,
    pub load_screen_path: String,
    pub load_screen_vintage_path: Option<String>,
    pub splash_video_path: Option<String>,
    pub collection_splash_video_path: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Skin {
    pub id: u32,
    pub is_base: bool,
    pub name: String,
    #[serde(flatten)]
    pub images: Images,
    pub skin_type: String,
    pub rarity: String,
    pub is_legacy: bool,
    pub features_text: Option<String>,
    pub chroma_path: Option<String>,
    pub chromas: Option<Vec<Chroma>>,
    pub region_rarity_id: u8,
    pub skin_lines: Option<Vec<SkinLine>>,
    pub description: Option<String>,
}

/// **K/DA ALL OUT Seraphine** is currently the only quest skin
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuestSkinInfo {
    pub name: String,
    pub splash_path: String,
    pub uncentered_splash_path: String,
    pub tile_path: String,
    pub collection_description: String,
    pub description_info: Vec<DescriptionInfo>,
    pub collection_card_path: String,
    pub tiers: Vec<Tier>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DescriptionInfo {
    pub title: String,
    pub description: String,
    pub icon_path: String,
}

/// This type is currently only used for K/DA ALL OUT Seraphine
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Tier {
    pub id: u32,
    pub name: String,
    pub stage: u32,
    pub description: String,
    pub short_name: String,
    #[serde(flatten)]
    pub images: Images
}
