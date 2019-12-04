use base64::{decode, encode};
use duct::cmd;

fn main() {
    let encoded_data = "VGhpcyBpcyBNb2p0YWJh";
    let data = decode(&encoded_data).unwrap();
    let result = String::from_utf8_lossy(&data);
    let string = result.to_string();
    cmd!("echo","test").run().unwrap();
}
