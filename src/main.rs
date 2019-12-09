use std::io::{self, Write};
use std::process::Stdio;

use structopt::StructOpt;

//use base64::decode;

const FFMPEG: &str = "ffmpeg";
const ARGS: &str = " ffmpeg -i rtmp://localhost/live/tabvn -c:a aac -b:a 32k -c:v libx264 -b:v 128K -hls_time 1 out.m3u8";

fn main() {
    let args: Cli = Cli::from_args();
    let encoded_data = args.input;
    decode_input(encoded_data);
    //    let data = decode(&encoded_data).unwrap();
//    let result = String::from_utf8_lossy(&data);
//    let string = result.to_string();
//    use std::process::Command;
//
//    let child = Command::new(FFMPEG)
//        .arg("-i")
//        .arg("rtmp://localhost/live/tabvn")
//        .arg("-c:a")
//        .arg("aac")
//        .arg("-b:a")
//        .arg("32k")
//        .arg("-c:v")
//        .arg("libx264")
//        .arg("-b:v")
//        .arg("128K")
//        .arg("-hls_time")
//        .arg("1")
//        .arg("out.m3u8")
////        .output()
//        .spawn()
//        .expect("ls command failed to start");
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

fn decode_input(string: String) {
    use percent_encoding::percent_decode;
    let decoded = percent_decode(string.as_bytes()).decode_utf8().unwrap();
    println!("{}", decoded);
}


pub struct VideoVariant {
    width: u32,
    height: u32,
    display_name: &'static str,
    resolution: &'static str,
    bitrate_low_motion: u32,
    bitrate_high_motion: u32,
    audio_bitrate: u32,
}

impl VideoVariant {
    const QCIF: VideVariant = VideoVariant {
        width: 256,
        height: 144,
        display_name: "144P",
        resolution: "256x144",
        bitrate_low_motion: 400_000,
        bitrate_high_motion: 450_000,
        audio_bitrate: 32_000,
    };
    const CIF: VideVariant = VideoVariant {
        width: 426,
        height: 240,
        display_name: "240P",
        resolution: "426x240",
        bitrate_low_motion: 600_000,
        bitrate_high_motion: 650_000,
        audio_bitrate: 64_000,
    };
    const QSD: VideVariant = VideoVariant {
        width: 640,
        height: 360,
        display_name: "360P",
        resolution: "640x360",
        bitrate_low_motion: 800_000,
        bitrate_high_motion: 856_000,
        audio_bitrate: 96_000,
    };
    const SD: VideVariant = VideoVariant {
        width: 854,
        height: 480,
        display_name: "480P",
        resolution: "854x480",
        bitrate_low_motion: 1_400_000,
        bitrate_high_motion: 1_498_000,
        audio_bitrate: 128_000,
    };
    const HD: VideVariant = VideoVariant {
        width: 1280,
        height: 720,
        display_name: "720P",
        resolution: "1280x720",
        bitrate_low_motion: 2_800_000,
        bitrate_high_motion: 2_996_000,
        audio_bitrate: 128_000,
    };
    const FHD: VideVariant = VideoVariant {
        width: 1920,
        height: 1080,
        display_name: "1080P",
        resolution: "1920x1080",
        bitrate_low_motion: 5_000_000,
        bitrate_high_motion: 5_350_000,
        audio_bitrate: 194_000,
    };
    const UHD: VideVariant = VideoVariant {
        width: 3840,
        height: 2160,
        display_name: "2160P",
        resolution: "3840x2160",
        bitrate_low_motion: 7_000_000,
        bitrate_high_motion: 7_350_000,
        audio_bitrate: 194_000,
    };
}

#[derive(StructOpt)]
struct Cli {
    input: String,

}
