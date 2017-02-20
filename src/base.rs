extern crate rustc_serialize;

use rustc_serialize::json;

#[derive(Default,RustcDecodable, RustcEncodable)]
pub struct LegacyJsonResponse {
    jsonrpc: String,
    result: String,
    id: i32,
    auth: String,
}

pub trait ToLegacy {
/*    fn new(jsonrpc: String, result: String, id: i32, auth: String ) -> Self;*/

    fn legacy_response(data: String) -> String {
        let legacy_struct = LegacyJsonResponse { jsonrpc: "2.0".to_string(), id: 1, result: data, ..Default::default() };
        let result = json::encode(&legacy_struct).unwrap();
        return result;
//       println!("{}",result);
    }
}

impl ToLegacy for LegacyJsonResponse {
}
