use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CollectionResponse {
    pub success: bool,
    pub data: Option<CollectionData>,
}

#[derive(Deserialize, Debug)]
pub struct CollectionData {
    pub grids: Vec<Asset>,
    pub heroes: Vec<Asset>,
    pub icons: Vec<Asset>,
    pub logos: Vec<Asset>,
}

#[derive(Deserialize, Debug)]
pub struct Asset {
    pub id: u32,
    pub url: String,
    pub game: AssetGameInfo,
}

#[derive(Deserialize, Debug)]
pub struct AssetGameInfo {
    pub id: u32,
    pub name: String,
    pub verified: bool,
}


#[derive(Deserialize, Debug)]
pub struct GameResponse {
    pub success: bool,
    pub data: Option<GameData>,
}

#[derive(Deserialize, Debug)]
pub struct GameData {
    pub platforms: Option<GamePlatform>,
    pub game: AssetGameInfo,
}

#[derive(Deserialize, Debug)]
pub struct GamePlatform {
    //TODO: other platforms might have useful data too?
    pub steam: Option<PlatformSteam>,
}

#[derive(Deserialize, Debug)]
pub struct PlatformSteam {
    //TODO: consider if we want to use the other data too
    pub id: String,
}
