use std::error::Error;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hitokoto {
    pub id: i64,
    pub uuid: String,
    pub hitokoto: Option<String>,
    #[serde(rename = "type")]
    pub type_field: Option<String>,
    pub from: Option<String>,
    #[serde(rename = "from_who")]
    pub from_who: Option<String>,
    pub creator: Option<String>,
    #[serde(rename = "creator_uid")]
    pub creator_uid: i64,
    pub reviewer: i64,
    #[serde(rename = "commit_from")]
    pub commit_from: Option<String>,
    pub created_at: Option<String>,
    pub length: i64,
}

const HITOKOTO_API: &str = "https://v1.hitokoto.cn";

pub async fn get_hitokoto() -> Result<Hitokoto, Box<dyn Error>> {
    let response = reqwest::get(format!("{}", HITOKOTO_API)).await?;
    let hitokoto: Hitokoto = response.json().await?;
    Ok(hitokoto)
}

pub async fn get_hitokoto_by_id(id: i64) -> Result<Hitokoto, Box<dyn Error>> {
    let response = reqwest::get(format!("{}/{}", HITOKOTO_API, id)).await?;
    let hitokoto: Hitokoto = response.json().await?;
    Ok(hitokoto)
}
