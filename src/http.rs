// Include libraries
use isahc::ReadResponseExt;

pub fn get(url: String) -> String{
    println!(":: GET {}", url);
    let resp = isahc::get(&url).expect("Failed to make HTTP request!").text().expect("Failed to get response text");
    println!(":: Request success");
    return resp;
}
pub fn save_get(url: String, output: String) {
    println!(":: curl -L {} -o {}", url, output);
    subprocess::Exec::shell(&format!("curl -L {} -o {}", url, output)).join().expect("Failed to run command!");
}
pub fn is_err(url: String) -> bool {
    let resp = isahc::get(&url).expect("Failed to make HTTP request!").status().as_u16();
    println!(":: status: {}", resp);
    if resp == 404 {
        return true;
    }else {
        return false;
    }
}