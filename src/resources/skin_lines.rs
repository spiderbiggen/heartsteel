use const_format::concatcp;
use serde::{Deserialize, Serialize};

use crate::{Error, RAW_COMMUNITY_DRAGON_URL};
use crate::version::Game;

const SKIN_LINES_PATH: &str = "plugins/rcp-be-lol-game-data/global/default/v1/skinlines.json";

pub async fn get(client: &reqwest::Client) -> Result<Vec<SkinLine>, Error> {
    crate::get(client, concatcp!(RAW_COMMUNITY_DRAGON_URL, "/latest/", SKIN_LINES_PATH)).await
}

pub async fn get_for_version(client: &reqwest::Client, version: Game) -> Result<Vec<SkinLine>, Error> {
    crate::get(client, format!("{RAW_COMMUNITY_DRAGON_URL}/{version}/{SKIN_LINES_PATH}")).await
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SkinLine {
    pub id: u32,
    pub name: String,
    pub description: String,
}