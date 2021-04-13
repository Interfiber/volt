// Use Path
use std::path::Path;

pub fn get_minecraft_folder() -> String{
    if cfg!(windows){
        println!(":: minecraft.rs has detected that the current OS is windows");
        return "OS_NO_SUPPORT".to_string();
    }else {
        println!(":: minecraft.rs has detected that the current OS is UNIX like");
        println!(":: checking for ~/.minecraft");
        let home = std::env::var("HOME").unwrap();
        if Path::new(&format!("{}/.minecraft", home)).exists(){
            return format!("{}/.minecraft", home);
        }else if Path::new(&format!("{}/Library/Application Support/minecraft", home)).exists() {
            return format!("{}/Library/Application Support/minecraft", home);
        }else {
            return "FAIL".to_string();
        }
    }
}