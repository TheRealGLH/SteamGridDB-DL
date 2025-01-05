use std::{collections::HashMap, fs, io::Error};

use crate::{connectors::http, Asset, CollectionResponse, GameData, GameResponse};

pub fn save_files(
    collection_json: CollectionResponse,
    directory: String,
    dry_run: bool,
) -> Result<(), i32> {
    if let Some(collection_data) = collection_json.data {
        let mut gamedata_map: HashMap<u32, GameData> = HashMap::new();
        for hero in &collection_data.heroes {
            let filename = extract_path_from_asset(hero, &mut gamedata_map);
            let extension = hero.url.rsplit_once('.').unwrap_or(("", ".png")).1;
            if let Err(e) = save_image(
                &format!("{directory}{filename}_hero.{extension}"),
                hero.url.as_str(),
                dry_run,
            ) {
                eprintln!("Error saving file {directory}{filename}_hero.{extension}: {e}");
                return Err(128);
            }
        }
        for grid in &collection_data.grids {
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
        for icon in &collection_data.icons {
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
        for logo in &collection_data.logos {
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
    } else {
        eprintln!("Somehow requesting the collection succeeded, but there was no data.");
        return Err(200);
    };
    Ok(())
}

fn extract_path_from_asset(asset: &Asset, gamedata: &mut HashMap<u32, GameData>) -> String {
    match gamedata.get(&asset.game.id) {
        Some(asset_gamedata) => {
            return extract_path_item_prefix_from_gamedata(asset_gamedata);
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
                            if let Some(response_gamedata) = response.data {
                                //dbg!(&response_gamedata);
                                let path =
                                    extract_path_item_prefix_from_gamedata(&response_gamedata);
                                gamedata.insert(asset.game.id, response_gamedata);
                                return path;
                            }
                        }
                        Err(e) => {
                            eprintln!("{e} for game id {}", asset.game.id);
                        }
                    }
                }
                Err(e) => eprint!("{e}"),
            }
        }
    }
    asset.game.id.to_string() + "/"
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
    gamedata.game.id.to_string() + "/"
}

fn save_image(path: &str, url: &str, dry_run: bool) -> Result<(), Error> {
    //Why is this not a singular if-statement?
    //Here's why: https://github.com/rust-lang/rust/issues/53667
    if !dry_run {
        if let Some(directory) = path.rsplit_once('/') {
            if let Err(e) = fs::create_dir_all(directory.0) {
                eprintln!("Error creating directory: {e}");
                return Err(e);
            }
        }
    }
    println!("Saving file {path}");
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
                    std::io::copy(&mut reader, &mut f)?;
                    Ok(())
                }
                Err(e) => Err(e),
            }
        }
        Err(e) => {
            eprintln!("{e}");
            Err(Error::new(std::io::ErrorKind::Other, e.to_string()))
        }
    }
}
