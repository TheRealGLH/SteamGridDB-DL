use std::{collections::HashMap, fs, io::Error};

use crate::{connectors::http, Asset, CollectionResponse, GameData, GameResponse};

pub fn save_files(
    collection_json: CollectionResponse,
    directory: String,
    dry_run: bool,
) -> Result<(), i32> {
    if let Err(e) = fs::create_dir_all(&directory) {
        eprintln!("Error creating directory: {e}");
        return Err(3);
    }
    match collection_json.data {
        Some(collection_data) => {
            let mut gamedata_map: HashMap<u32, GameData> = HashMap::new();
            for hero in collection_data.heroes.iter() {
                let filename = extract_path_from_asset(hero, &mut gamedata_map);
                let extension = hero.url.rsplit_once('.').unwrap_or(("", ".png")).1;
                if let Err(e) = save_image(
                    &format!("{directory}{filename}_hero.{extension}"),
                    hero.url.as_str(),
                    dry_run,
                ) {
                    eprintln!("Error saving file: {e}");
                    return Err(128);
                }
            }
            for grid in collection_data.grids.iter() {
                let filename = extract_path_from_asset(grid, &mut gamedata_map);
                let extension = grid.url.rsplit_once('.').unwrap_or(("", ".png")).1;
                if let Err(e) = save_image(
                    &format!("{directory}{filename}p.{extension}"),
                    grid.url.as_str(),
                    dry_run,
                ) {
                    eprintln!("Error saving file: {e}");
                    return Err(128);
                }
            }
            for icon in collection_data.icons.iter() {
                let filename = extract_path_from_asset(icon, &mut gamedata_map);
                let extension = icon.url.rsplit_once('.').unwrap_or(("", ".png")).1;
                if let Err(e) = save_image(
                    &format!("{directory}{filename}_icon.{extension}"),
                    icon.url.as_str(),
                    dry_run,
                ) {
                    eprintln!("Error saving file: {e}");
                    return Err(128);
                }
            }
            for logo in collection_data.logos.iter() {
                let filename = extract_path_from_asset(logo, &mut gamedata_map);
                let extension = logo.url.rsplit_once('.').unwrap_or(("", ".png")).1;
                if let Err(e) = save_image(
                    &format!("{directory}{filename}_logo.{extension}"),
                    logo.url.as_str(),
                    dry_run,
                ) {
                    eprintln!("Error saving file: {e}");
                    return Err(128);
                }
            }
        }
        None => {
            eprintln!("Somehow requesting the collection succeeded, but there was no data.");
            return Err(200);
        }
    };
    Ok(())
}

fn extract_path_from_asset(asset: &Asset, gamedata: &mut HashMap<u32, GameData>) -> String {
    match gamedata.get(&asset.game.id) {
        Some(asset_gamedata) => {
            return extract_path_item_prefix_from_gamedata(&asset_gamedata);
        }
        None => {
            //TODO: find the game data ourselves and parse it
            //Afterwards we store it in gamedata

            match http::handle_get_request(
                http::HttpRequest::game_info_request(&asset.game.id.to_string())
                    .expect("Invalid game id {} in asset id {}"),
            ) {
                Ok(g) => {
                    match g.into_json::<GameResponse>() {
                        Ok(response) => {
                            //dbg!(&response.data);
                            match response.data {
                                Some(response_gamedata) => {
                                    //dbg!(&response_gamedata);
                                    let path =
                                        extract_path_item_prefix_from_gamedata(&response_gamedata);
                                    gamedata.insert(asset.game.id, response_gamedata);
                                    return path;
                                }
                                None => todo!(),
                            }
                        }
                        Err(e) => {
                            eprintln!("{e} for game id {}", asset.game.id);
                        }
                    }
                }
                Err(e) => eprint!("{}", e),
            }
        }
    }
    return asset.game.id.to_string() + "/";
}

fn extract_path_item_prefix_from_gamedata(gamedata: &GameData) -> String {
    if let Some(platform_data) = &gamedata.platforms {
        if let Some(steam) = &platform_data.steam {
            return steam.id.clone();
        }
    }
    println!(
        "Could not find a Steam ID for {}, look for it in the {} folder",
        gamedata.game.name, gamedata.game.id
    );
    return gamedata.game.id.to_string() + "/";
}

fn save_image(path: &str, url: &str, dry_run: bool) -> Result<(), Error> {
    println!("Saving file {}", path);
    if dry_run {
        return Ok(());
    }
    let request: http::HttpRequest = http::HttpRequest {
        method: http::HttpRequestMethod::GET,
        url: url.to_string(),
        headers: vec![],
    };

    match http::handle_get_request(request) {
        Ok(r) => {
            let mut reader = r.into_reader();
            match fs::File::create(path) {
                Ok(mut f) => {
                    if let Err(e) = std::io::copy(&mut reader, &mut f){
                        return Err(e);
                    }
                   return Ok(())
                },
                Err(e) => return Err(e),
            }
        }
        Err(e) => {
            eprintln!("{e}");
            return Err(Error::new(std::io::ErrorKind::Other, e.to_string()));
        }
    };
}