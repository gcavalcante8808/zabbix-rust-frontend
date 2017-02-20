extern crate rustc_serialize;
extern crate hyper;

#[macro_use] extern crate nickel;
//use std::collections::HashMap;
use nickel::{Nickel, HttpRouter};

mod base;
mod apiinfo;
mod tests;

fn main(){
    let mut server = Nickel::new();

/*    server.get("/", middleware! { |_, response|
        let mut data = HashMap::new();
        data.insert("name", "user");
        return response.render("examples/assets/template.tpl", &data);
    });*/
    // There is no support for named parameters at this moment. The boolean passed here defines if the result should be returned in the LegacyMode (JSON-RPC 2.0 style) or Not (Rest Mode).
    server.get("/api/:method", middleware!(apiinfo::get_apiinfo_version(false)));
    server.get("/api_jsonrpc.php", middleware!(apiinfo::get_apiinfo_version(true)));
    server.post("/api_jsonrpc.php", middleware!(apiinfo::get_apiinfo_version(true)));

    server.listen("127.0.0.1:6767").unwrap();
}
