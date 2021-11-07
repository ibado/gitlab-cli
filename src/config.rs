use std::io::{Write, BufRead, Error};
use std::fs::{OpenOptions, remove_file};

pub fn load_config() {
    let file = OpenOptions::new()
        .read(true)
        .open(config_file_name());

    if file.is_err() { return (); }

    let lines = std::io::BufReader::new(file.unwrap()).lines();
    for line in lines {
        if let Ok(config) = line {
            let words: Vec<_> = config.split("=").collect();
            std::env::set_var(words[0], words[1])
        }
    }
}

pub fn write_config(key: String, value: String) -> Result<(), Error> {
    let file = std::fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(config_file_name())
        .unwrap();
    return writeln!(&file, "{}={}", key, value);
}

pub fn clear_config() -> Result<(), Error> {
    let result = remove_file(config_file_name());
    if result.is_err() { /* config file is not created */ }
    Ok(())
}

fn config_file_name() -> String {
    let home = std::env::var("HOME").unwrap();
    home + "/.glab"
}
