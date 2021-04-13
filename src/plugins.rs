use serde_json::{Value};
use crate::http;

pub static FORGE_API_URL: &str = "https://addons-ecs.forgesvc.net/api/v2/addon";
// https://addons-ecs.forgesvc.net/api/v2/addon/60089/files
pub fn get_latest_plugin(proj_id: String) -> String {
    println!(":: making curseforge API request...");
    let body_text = http::get(format!("{}/{}/files", FORGE_API_URL, proj_id));
    println!(":: checking for 404...");
    let is_err = http::is_err(format!("{}/{}/files", FORGE_API_URL, proj_id));
    if is_err {
        return "PLUGIN_NOT_FOUND".to_string();
    }else {
        let body: Value = serde_json::from_str(&body_text).expect("Failed to parse response body");
        println!(":: getting latest download URL...");
        // NOTE: The last index of the returned json array is the latest one added to the array(Like a stack)
        // NOTE: So we need to extract the last object from the array, then get the "downloadUrl"
        
        // Get the last index of the array
        let last = body.as_array().unwrap().last().unwrap();
        println!("{}", last);
        return last["downloadUrl"].to_string().replace("\"", "");
    }
}
pub fn get_plugin_name(proj_id: String) -> String {
    let body_text = http::get(format!("{}/{}", FORGE_API_URL, proj_id));
    let body: Value = serde_json::from_str(&body_text).unwrap();
    println!(":: getting mod name...");

    // Get the display name on minecraft forge
    let forge_name = body["name"].to_string().replace("\"", "");

    // Make it all lower case, and replace - with _, and spaces with _
    let safe_name = forge_name.replace("-", "_").replace(" ", "_");
    // Return the safe name
    return safe_name;
}
pub fn download(url: String) {
    http::save_get(url, String::from("/tmp/mod.jar"));
}
pub fn install_mod(name: String){
    let mc_folder = crate::minecraft::get_minecraft_folder();
    std::fs::copy("/tmp/mod.jar", &format!("{}/mods/{}_latest_volt.jar", mc_folder, name)).expect("Failed to copy mod into install folder");
    println!(":: removing mod.jar");
    match std::fs::remove_file("/tmp/mod.jar"){
        Ok(_) => print!(""),
        Err(err) => {
            println!(":: failed to remove /tmp/mod.jar!");
            println!(":: {}", err);
        }
    }
}