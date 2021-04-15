use std::path::Path;
use envmnt::get_or;
use std::io::Read;
use std::fs::File;
pub fn config_exists() -> bool {
    // Config files are located at: ~/.config/volt/volt.json  
    let home = get_or("HOME", "HOME_USR_NOT_FOUND_ERR");
    if home == "HOME_USR_NOT_FOUND_ERR" {
        return false;
    }else {
        let config_location = &format!("{}/.config/volt/volt.json", home);
        println!(":: default config location {}", config_location);
        if !Path::new(config_location).exists(){
            println!(":: config not found!");
            return false;
        }else {
            println!(":: config found.");
            return true;
        }
    }
}
pub fn get_config() -> String {
    let home = get_or("HOME", "HOME_USR_NOT_FOUND_ERR");
    let config_location = &format!("{}/.config/volt/volt.json", home);
    println!(":: default config location {}", config_location);
    let mut configf = File::open(config_location).expect("Failed to read config.");
    let mut config = String::new();
    match configf.read_to_string(&mut config){
        Ok(_) => print!(""),
        Err(err) => {
            println!("PANIC!!! ");
            println!("Failed to read config file, make sure to run the config_exists()");
            println!("function before this one. Error below");
            println!("{}", err);
            println!("Exit Code: 0");
            std::process::exit(0);
        }
    }
    return config.to_string();
}