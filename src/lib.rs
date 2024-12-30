mod connectors;

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
pub fn run(config: Configuration) {
    dbg!(&config);
    if config.print_help {
        print_help();
        return;
    }
}

pub fn print_help() {
    println!("usage: steamgriddb-dl <grid id> [--directory=<path>]");
}
