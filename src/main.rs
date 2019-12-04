use base64::decode;

use structopt::StructOpt;
use std::io::{self, Write};
use std::process::Stdio;

const FFMPEG: &str = "ffmpeg";
const ARGS: &str = " ffmpeg -i rtmp://localhost/live/tabvn -c:a aac -b:a 32k -c:v libx264 -b:v 128K -hls_time 1 out.m3u8";

fn main() {
//        let args: Cli = Cli::from_args();
//    let encoded_data = args.input;
//    let data = decode(&encoded_data).unwrap();
//    let result = String::from_utf8_lossy(&data);
//    let string = result.to_string();
    use std::process::Command;

    let child = Command::new(FFMPEG)
        .arg("-i")
        .arg("rtmp://localhost/live/tabvn")
        .arg("-c:a")
        .arg("aac")
        .arg("-b:a")
        .arg("32k")
        .arg("-c:v")
        .arg("libx264")
        .arg("-b:v")
        .arg("128K")
        .arg("-hls_time")
        .arg("1")
        .arg("out.m3u8")
//        .output()
        .spawn()
        .expect("ls command failed to start");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());

//    use std::process::{Command, Stdio};
//
//    let child = Command::new("ls")
//        .arg("-la")
//        .stdout(Stdio::piped())
//        .spawn()
//        .expect("failed to execute child");
//
//    let output = child
//        .wait_with_output()
//        .expect("failed to wait on child");
//    println!("status: {}", output.status);
//    io::stdout().write_all(&output.stdout).unwrap();
//    io::stderr().write_all(&output.stderr).unwrap();
//
//    assert!(output.status.success());
}


#[derive(StructOpt)]
struct Cli {
    input: String,

}
