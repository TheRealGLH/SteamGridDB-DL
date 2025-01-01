use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct CollectionInfo {
    pub success: bool,
    pub data: Option<CollectionData>,
}

#[derive(Deserialize, Debug)]
pub struct CollectionData {
    pub grids: Vec<AssetInfo>,
    pub heroes: Vec<AssetInfo>,
    pub icons: Vec<AssetInfo>,
    pub logos: Vec<AssetInfo>,
}

#[derive(Deserialize, Debug)]
pub struct AssetInfo {
    pub id: u32,
    pub url: String,
    pub game: AssetGameInfo,
}

#[derive(Deserialize, Debug)]
pub struct AssetGameInfo {
    pub id: u32,
    pub name: String,
    pub release_date: u64,
    pub verified: bool,
}


#[derive(Deserialize, Debug)]
pub struct GameInfo {
    pub success: bool,
    pub data: Option<GameData>,
}

#[derive(Deserialize, Debug)]
pub struct GameData {
    platforms: GamePlatform,
}

#[derive(Deserialize, Debug)]
pub struct GamePlatform {
    //TODO: other platforms might have useful data too?
    steam: Option<PlatformSteam>,
}

#[derive(Deserialize, Debug)]
pub struct PlatformSteam {
    //TODO: consider if we want to use the other data too
    id: String,
}
