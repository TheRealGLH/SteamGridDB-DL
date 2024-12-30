#[cfg(test)]
mod tests {
    use steamgriddb_dl::*;

    #[test]
    fn build_config_errors_if_no_args() {
        let args: Vec<String> = Vec::new();
        let config = Configuration::build(args.into_iter());
        assert!(config.is_err(), "Configuration should not exist.");
    }

    #[test]
    fn build_returns_config_if_args_exist() {
        let mut args: Vec<String> = Vec::new();
        args.push("--".to_string());
        let config = Configuration::build(args.into_iter());
        assert!(!config.is_err(), "Configuration should exist.");
    }

    #[test]
    fn help_enabled_when_only_one_arg() {
        let mut args: Vec<String> = Vec::new();
        args.push("--".to_string());
        let config = Configuration::build(args.into_iter());
        match config {
            Ok(c) => {
                assert!(c.print_help, "Print help is not enabled.");
            }
            Err(_) => {
                panic!("Config does not exist while it should.")
            }
        };
    }

    #[test]
    fn help_enabled_if_flag_set() {
        let mut args: Vec<String> = Vec::new();
        args.push("--".to_string());
        args.push("-h".to_string());

        let config = Configuration::build(args.into_iter());
        match config {
            Ok(c) => {
                assert!(c.print_help, "Print help is not enabled.");
            }
            Err(_) => {
                panic!("Config does not exist while it should.")
            }
        };
    }

    #[test]
    fn help_enabled_if_no_id() {
        let mut args: Vec<String> = Vec::new();
        args.push("--".to_string());

        let config = Configuration::build(args.into_iter());
        match config {
            Ok(c) => {
                assert!(c.print_help, "Print help is not enabled.");
            }
            Err(_) => {
                panic!("Config does not exist while it should.")
            }
        };
    }

    #[test]
    fn override_none_if_not_passed() {
        let mut args: Vec<String> = Vec::new();
        args.push("--".to_string());
        args.push("123".to_string());

        let config = Configuration::build(args.into_iter());
        match config {
            Ok(c) => {
                assert!(c.override_directory.is_none(), "Override should not be set");
            }
            Err(_) => {
                panic!("Config does not exist while it should.")
            }
        };
    }

    #[test]
    fn override_set_if_passed() {
        let mut args: Vec<String> = Vec::new();
        let directory = "/home/user/pictures";
        args.push("--".to_string());
        args.push("123".to_string());
        args.push("--directory=".to_string() + &directory);

        let config = Configuration::build(args.into_iter());
        match config {
            Ok(c) => {
                match c.override_directory {
                    Some(config_directory) => assert_eq!(config_directory, directory),
                    None => panic!("Override directory not set."),
                };
            }
            Err(_) => {
                panic!("Config does not exist while it should.")
            }
        };
    }

    #[test]
    fn dry_run_false_if_not_passed() {
        let mut args: Vec<String> = Vec::new();
        args.push("--".to_string());
        args.push("123".to_string());

        let config = Configuration::build(args.into_iter());
        match config {
            Ok(c) => {
                assert!(!c.dry_run, "Dry run is enabled.");
            }
            Err(_) => {
                panic!("Config does not exist while it should.")
            }
        };
    }
    #[test]
    fn dry_run_false_if_nothing_passed() {
        let mut args: Vec<String> = Vec::new();
        args.push("--".to_string());

        let config = Configuration::build(args.into_iter());
        match config {
            Ok(c) => {
                assert!(!c.dry_run, "Dry run is enabled.");
            }
            Err(_) => {
                panic!("Config does not exist while it should.")
            }
        };
    }

    #[test]
    fn dry_run_true_if_passed() {
        let mut args: Vec<String> = Vec::new();
        args.push("--".to_string());
        args.push("123".to_string());
        args.push("-n".to_string());

        let config = Configuration::build(args.into_iter());
        match config {
            Ok(c) => {
                assert!(c.dry_run, "Dry run is not enabled.");
            }
            Err(_) => {
                panic!("Config does not exist while it should.")
            }
        };
    }

    #[test]
    fn id_none_if_not_passed() {
        let mut args: Vec<String> = Vec::new();
        args.push("--".to_string());

        let config = Configuration::build(args.into_iter());
        match config {
            Ok(c) => {
                assert!(c.grid_id.is_none(), "Grid ID is set.");
            }
            Err(_) => {
                panic!("Config does not exist while it should.")
            }
        };
    }

    #[test]
    fn id_set_if_passed() {
        let mut args: Vec<String> = Vec::new();
        let id = "123";
        args.push("--".to_string());
        args.push(id.to_string());

        let config = Configuration::build(args.into_iter());
        match config {
            Ok(c) => {
                match c.grid_id {
                    Some(config_id) => assert_eq!(id, config_id),
                    None => panic!("ID is not set"),
                };
            }
            Err(_) => {
                panic!("Config does not exist while it should.")
            }
        };
    }
}
