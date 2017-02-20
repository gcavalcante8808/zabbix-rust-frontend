extern crate rustc_serialize;

use std;
use rustc_serialize::json;
use base::ToLegacy;

#[derive(RustcDecodable, RustcEncodable,Default)]
struct ApiInfo {
    version: String,
}

impl std::fmt::Display for ApiInfo {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "(value a: {})", self.version)
    }
}

impl ToLegacy for ApiInfo{ }


pub fn get_apiinfo_version(legacy_mode: bool) -> String {
    let apiinfo = ApiInfo { version: "3.2.3".to_string() };
    let result;

    if legacy_mode == true{
        let data = String::from("3.2.3");
        result = ApiInfo::legacy_response(data);
    } else {
        result = json::encode(&apiinfo).unwrap();
    }

    return result;
}

