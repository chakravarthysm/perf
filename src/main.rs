extern crate hyper;

use hyper::client;
fn main(){
let client = client::Client::new();

let res = client.get("http://example.domain").send().unwrap();
assert_eq!(res.status, hyper::Ok);

}