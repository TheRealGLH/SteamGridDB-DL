pub mod connectors;
mod files;

use connectors::api_responses::*;
use connectors::http;
use files::*;

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
    let mut directory = config.override_directory.unwrap_or(get_steam_directory());
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
                                return save_files(
                                    collection_response,
                                    directory,
                                    config.dry_run,
                                );
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

fn get_steam_directory() -> String{
    String::from("/tmp/steamgriddb/")
}
