use std::io::{self, Write};
use std::process::{Stdio, Command};

use structopt::StructOpt;
use std::borrow::Cow;
use std::str::Split;
use std::fs;
use std::path::Path;

//use base64::decode;


/// folder name: KwLzl/processed/video/5036160f-8aed-49bb-b2da-3375ba113f55
/// folder name: KwLzl/processed/video/5036160f-8aed-49bb-b2da-3375ba113f55/480P/012.ts

const FFMPEG: &str = "ffmpeg";
const ARGS: &str = " ffmpeg -i rtmp://localhost/live/tabvn -c:a aac -b:a 32k -c:v libx264 -b:v 128K -hls_time 1 out.m3u8";

fn main() {
    let args: Cli = Cli::from_args();
    let encoded_data = args.input;
    let input = decode_input(encoded_data);

    let id_result = read_input_id(&input).unwrap();

    let source = format!("{}{}", "rtmp://localhost/live/", id_result);
    let mut args = vec![
//        FFMPEG,
"-y",//Overwrite output files without asking.
"-i",// Input
source.as_str(),
    ];
    let hd_variant = VideoVariants::HD.get_variant();


    let hd_args = hd_variant.get_args();

    args.extend_from_slice(&hd_args);
//    let directory = format!("{}", &input);
    let relative_path = format!("./{}", &input);
    let quality_path = format!("{}/{}", &input, &hd_variant.display_name);
    let output_path = format!("{}/manifest.m3u8", &quality_path);
    let relative_quality_path = format!("./{}", &quality_path);
    dbg!(&input);
    dbg!(&output_path);
    dbg!(&relative_path);
    dbg!(&quality_path);
    dbg!(&relative_quality_path);

    let segment_filename = format!("{}/%03d.ts", quality_path);
    args.push(segment_filename.as_str());
    args.push(output_path.as_str());
    dbg!(&args);

    fs::create_dir_all(&relative_path).unwrap();
    fs::create_dir_all(&relative_quality_path).unwrap();


    //    let data = decode(&encoded_data).unwrap();
//    let result = String::from_utf8_lossy(&data);
//    let string = result.to_string();
//    use std::process::Command;
//
    let child = Command::new(FFMPEG)
        .args(&args)
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
}

fn decode_input(string: String) -> String {
    use percent_encoding::percent_decode;
    let decoded = percent_decode(string.as_bytes()).decode_utf8().unwrap();
    println!("{}", decoded);
    return decoded.to_string();
}


fn add_transcode_variants(args: &mut Vec<&str>) {}

fn read_input_id(input: &String) -> Result<String, ()> {
//    let in_str = input.as_str();
    let id = input.split("/").last();
    match id {
        Some(id) => Ok(id.to_string()),
        None => Err(())
    }
}

struct VideoVariant {
    width: &'static str,
    height: &'static str,
    display_name: &'static str,
    resolution: &'static str,
    bitrate_low_motion: &'static str,
    bitrate_high_motion: &'static str,
    audio_bitrate: &'static str,
}

impl VideoVariant {
    fn get_args(&self) -> Vec<&str> {
        return vec!["-c:a", "aac", "-b:a", self.audio_bitrate, "-c:v", "libx264", "-b:v",
                    self.bitrate_low_motion, "-hls_time", "1", "-hls_segment_filename"];
    }
}

enum VideoVariants {
    QCIF,
    CIF,
    QSD,
    SD,
    HD,
    FHD,
    UHD,
}

impl VideoVariants {
    fn get_variant(self) -> VideoVariant {
        return match self {
            VideoVariants::QCIF => VideoVariant {
                width: "256",
                height: "144",
                display_name: "144P",
                resolution: "256x144",
                bitrate_low_motion: "400k",
                bitrate_high_motion: "450k",
                audio_bitrate: "32k",
            },
            VideoVariants::CIF => VideoVariant {
                width: "426",
                height: "240",
                display_name: "240P",
                resolution: "426x240",
                bitrate_low_motion: "600k",
                bitrate_high_motion: "650k",
                audio_bitrate: "64k",
            },
            VideoVariants::QSD => VideoVariant {
                width: "640",
                height: "360",
                display_name: "360P",
                resolution: "640x360",
                bitrate_low_motion: "800k",
                bitrate_high_motion: "856k",
                audio_bitrate: "96k",
            },
            VideoVariants::SD => VideoVariant {
                width: "854",
                height: "480",
                display_name: "480P",
                resolution: "854x480",
                bitrate_low_motion: "1400k",
                bitrate_high_motion: "1498k",
                audio_bitrate: "128k",
            },
            VideoVariants::HD => VideoVariant {
                width: "1280",
                height: "720",
                display_name: "720P",
                resolution: "1280x720",
                bitrate_low_motion: "2800k",
                bitrate_high_motion: "2996k",
                audio_bitrate: "128k",
            },
            VideoVariants::FHD => VideoVariant {
                width: "1920",
                height: "1080",
                display_name: "1080P",
                resolution: "1920x1080",
                bitrate_low_motion: "5000k",
                bitrate_high_motion: "5350k",
                audio_bitrate: "194k",
            },
            VideoVariants::UHD => VideoVariant {
                width: "3840",
                height: "2160",
                display_name: "2160P",
                resolution: "3840x2160",
                bitrate_low_motion: "7000k",
                bitrate_high_motion: "7350k",
                audio_bitrate: "194k",
            },
        };
    }
    //    const QCIF: VideVariant = VideoVariant {
//        width: 256,
//        height: 144,
//        display_name: "144P",
//        resolution: "256x144",
//        bitrate_low_motion: 400_000,
//        bitrate_high_motion: 450_000,
//        audio_bitrate: 32_000,
//    };
//    const CIF: VideVariant = VideoVariant {
//        width: 426,
//        height: 240,
//        display_name: "240P",
//        resolution: "426x240",
//        bitrate_low_motion: 600_000,
//        bitrate_high_motion: 650_000,
//        audio_bitrate: 64_000,
//    };
//    const QSD: VideVariant = VideoVariant {
//        width: 640,
//        height: 360,
//        display_name: "360P",
//        resolution: "640x360",
//        bitrate_low_motion: 800_000,
//        bitrate_high_motion: 856_000,
//        audio_bitrate: 96_000,
//    };
//    const SD: VideVariant = VideoVariant {
//        width: 854,
//        height: 480,
//        display_name: "480P",
//        resolution: "854x480",
//        bitrate_low_motion: 1_400_000,
//        bitrate_high_motion: 1_498_000,
//        audio_bitrate: 128_000,
//    };
//    const HD: VideVariant = VideoVariant {
//        width: 1280,
//        height: 720,
//        display_name: "720P",
//        resolution: "1280x720",
//        bitrate_low_motion: 2_800_000,
//        bitrate_high_motion: 2_996_000,
//        audio_bitrate: 128_000,
//    };
//    const FHD: VideVariant = VideoVariant {
//        width: 1920,
//        height: 1080,
//        display_name: "1080P",
//        resolution: "1920x1080",
//        bitrate_low_motion: 5_000_000,
//        bitrate_high_motion: 5_350_000,
//        audio_bitrate: 194_000,
//    };
//    const UHD: VideVariant = VideoVariant {
//        width: 3840,
//        height: 2160,
//        display_name: "2160P",
//        resolution: "3840x2160",
//        bitrate_low_motion: 7_000_000,
//        bitrate_high_motion: 7_350_000,
//        audio_bitrate: 194_000,
//    };
}

#[derive(StructOpt)]
struct Cli {
    input: String,

}
