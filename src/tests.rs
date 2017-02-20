extern crate hyper;
extern crate rustc_serialize;


#[derive(RustcDecodable, RustcEncodable)]
struct LegacyJsonRequest {
    jsonrpc: String,
    method: String,
    params: String,
    id: i32,
    auth: String,
}

#[test]
fn apiinfo_rest_tests(){
    let client = Client::new();

    let url = "http://localhost:6767/api/apiinfo";
    
    let mut http_reader = header::Headers::new();
    http_reader.set_raw("Content-Type",vec![b"application/json".to_vec()]);

    let mut response = client.get(url)
                             .send().unwrap();

    assert_eq!(response.status, hyper::Ok);

    let mut data = String::new();
    response.read_to_string(&mut data);

    let encoded = rustc_serialize::json::Json::from_str(&data).unwrap();
    assert!(encoded.is_object());

    let obj = encoded.as_object().unwrap();
    let version = obj.get("version").unwrap();

    assert!(version.is_string());
    assert_eq!(version.to_string(),String::from("3.2.3"));
    
}

#[test]
fn apiinfo_jsonrpc_tests(){
    let client = Client::new();

    let url = "http://localhost:6767/api_jsonrpc.php";

    let mut http_reader = header::Headers::new();
    http_reader.set_raw("Content-Type",vec![b"application/json".to_vec()]);
    
    //TODO: How to use a struct and 'export' it to a raw string literal???
/*    let request_data = LegacyJsonRequest { jsonrpc: "2.0".to_string(), 
                                            method: "apiinfo.version".to_string(),
                                            params: "[]".to_string(),
                                            auth: "[]".to_string(),
                                            id: 1 };

    let encoded_request = rustc_serialize::json::encode(&request_data).unwrap();*/

    let request_data = r#"{"jsonrpc":"2.0", "method": "apiinfo.version", "params": {}, "auth": {}, "id": "1"}"#;

    let mut response = client.post(url).body(request_data)
                             .send().unwrap();

    assert_eq!(response.status, hyper::Ok);

    let mut response_data = String::new();
    response.read_to_string(&mut response_data);

    let encoded = rustc_serialize::json::Json::from_str(&response_data).unwrap();

    assert!(encoded.is_object());

    let obj = encoded.as_object().unwrap();
    let result = obj.get("result").unwrap();

    assert_eq!(result.to_string(),String::from("3.2.3"));

}

