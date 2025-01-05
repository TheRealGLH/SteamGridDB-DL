pub mod connectors;
mod files;

use connectors::api_responses::*;
use connectors::http;
use files::*;
use std::env;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;

#[derive(Debug)]
pub struct Configuration {
    pub override_directory: Option<String>,
    pub dry_run: bool,
    pub grid_id: Option<String>,
    pub print_help: bool,
}

impl Configuration {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        //the first argument passed is the executable name. we don't need it
        if args.next().is_none() {
            return Err("No arguments passed. Somehow");
        }
        let mut print_help: bool = false;
        let mut dry_run = false;
        let mut override_directory: Option<String> = None;
        let grid_id: Option<String> = match args.next() {
            Some(arg) => {
                if arg == "-h" {
                    print_help = true;
                    None
                } else {
                    Some(arg)
                }
            }
            None => {
                print_help = true;
                None
            }
        };

        while let Some(arg) = args.next() {
            match arg.as_str() {
                "-h" => print_help = true,
                "-n" => dry_run = true,
                _ => {
                    if arg.starts_with("--directory=") {
                        match arg.split_once('=') {
                            Some(a) => {
                                override_directory = Some(a.1.to_string());
                            }
                            None => return Err(
                                "Something went horribly wrong trying to parse the argument {arg}",
                            ),
                        }
                    }
                }
            }
        }

        Ok(Configuration {
            override_directory,
            dry_run,
            grid_id,
            print_help,
        })
    }
}
pub fn run(config: Configuration) -> Result<(), i32> {
    dbg!(&config);
    if config.print_help {
        print_help();
        return Ok(());
    }
    let mut directory = config.override_directory.unwrap_or(guess_steam_directory());
    match config.grid_id {
        Some(id) => {
            let request = http::HttpRequest::collection_info_request(&id);
            match request {
                Ok(r) => {
                    match http::handle_get_request(r) {
                        Ok(r) => match r.into_json::<CollectionResponse>() {
                            Ok(collection_response) => {
                                if !directory.ends_with('/') {
                                    directory = directory + "/"
                                }
                                return save_files(collection_response, directory, config.dry_run);
                            }
                            Err(e) => {
                                eprintln!("JSON format error: {e}");
                                return Err(3);
                            }
                        },
                        Err(e) => {
                            eprintln!("{}: {}", e.kind(), e.to_string());
                            return Err(3);
                        }
                    };
                }
                Err(e) => {
                    eprintln!("Couldn't form collection request for id: {id}: {e}");
                    return Err(3);
                }
            };
        }
        None => {
            print_help();
            eprintln!("Please supply a collection ID, as seen in the webpage URL: https://www.steamgriddb.com/collection/<id>");
            return Err(10);
        }
    };
}

pub fn print_help() {
    println!("usage: steamgriddb-dl <grid id> [--directory=<path>]");
}

//TODO: This whole function is completely awful and needs a top to bottom rewrite.
//at the very least it should return a Result<String, Err> instead of just a String
fn guess_steam_directory() -> String {
    let mut base_dir = String::new();
    match env::consts::OS {
        "linux" => {
            base_dir = env::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("/home/"))
                .to_str()
                .unwrap()
                .to_string()
                + "/.steam/steam/userdata";
        }
        "macos" => {
            base_dir = env::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("/home/"))
                .to_str()
                .unwrap()
                .to_string()
                + "/Library/Application Support/Steam/userdata";
        }

        "windows" => base_dir += "C:\\Program Files (x86)\\Steam\\userdata",
        _ => return String::from("/tmp/steamgriddb/"),
    }
    if let Ok(read_dir) = fs::read_dir(&base_dir) {
        let mut previous_dir: Option<DirEntry> = None;
        for dir_result in read_dir {
            if let Ok(dir_entry) = dir_result {
                if let Ok(file_type) = dir_entry.file_type() {
                    if (file_type.is_dir()) {
                        previous_dir = Some(dir_entry);
                    } else {
                        break;
                    };
                }
            }
        }
        if let Some(selected_dir) = previous_dir {
            base_dir += &(selected_dir.path().to_str().unwrap().to_string() + "/config/grid");
        }
    } else {
        eprintln!("Couldn't locate the userdata folder, which is normally located at <wherever you installed steam>/userdata/<user number>/config/grid. The items will be downloaded, but to a fallback directory.\n Try rerunning the program with the --directory flag as shown in the instructions to manually set it.");
    }

    return base_dir;
}
