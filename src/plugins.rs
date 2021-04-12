use curl::easy::Easy;

pub static FORGE_API_URL: &str = "https://addons-ecs.forgesvc.net/api/v2/";
// https://addons-ecs.forgesvc.net/api/v2/addon/search?gameId=432&sort=create&sectionId=null
pub fn search_plugins(keyword: String){
    println!(":: Making CurseForge API request...");
    let mut easy = Easy::new();
    easy.url(&format!("{}addon/search?gameId=432&sort={}&sectionId=null", FORGE_API_URL, keyword)).unwrap();
    let mut data = Vec::new();
    {
        let mut transfer = easy.transfer();
        transfer.write_function(|dat| {
            data.extend_from_slice(dat);
            Ok(dat.len())
        }).unwrap();
        transfer.perform().unwrap();
    }
    let body = String::from_utf8(data).expect("body is not valid UTF8!");
    println!(":: got response. parsing response data...");
}