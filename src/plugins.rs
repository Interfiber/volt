use serde_json::{Value};
use isahc::ReadResponseExt;

pub static FORGE_API_URL: &str = "https://addons-ecs.forgesvc.net/api/v2/addon";
// https://addons-ecs.forgesvc.net/api/v2/addon/60089/files
pub fn get_latest_plugin(proj_id: String) -> String {
    println!(":: making curseforge API request...");
    let mut response = isahc::get(&format!("{}/{}/files", FORGE_API_URL, proj_id)).expect("Failed to make API request");
    let body_text = response.text().expect("Failed to get response text");
    let body: Value = serde_json::from_str(&body_text).expect("Failed to parse response body");
    println!(":: getting latest download URL...");
    // NOTE: The last index of the returned json array is the latest one added to the array(Like a stack)
    // NOTE: So we need to extract the last object from the array, then get the "downloadUrl"
    
    // Get the last index of the array
    let last = body.as_array().unwrap().last().unwrap();
    println!("{}", last);
    return last["downloadUrl"].to_string();
}