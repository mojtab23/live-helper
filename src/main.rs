use std::io::{self, Write};
use std::process::Command;

use structopt::StructOpt;
use std::fs;


const FFMPEG: &str = "ffmpeg";
const PLAYLIST_HEADER: &str = "#EXTM3U\n#EXT-X-VERSION:3\n";

fn main() {
    let cli: Cli = Cli::from_args();
    dbg!(&cli);

    let mut playlist = String::from(PLAYLIST_HEADER);
    let any_output = check_playlist_outputs(&mut playlist, &cli);
    if !any_output {
        println!("There is no output type, see --help");
        return;
    };

    let relative_root_path = cli.root_path.as_ref().map_or("./", String::as_str);
//    let root_path = cli.root_path.as_ref().map_or("", String::as_str);
    let root_path = &cli.root_path.as_ref();
    dbg!(&root_path);


    let encoded_data = &cli.input;
    let input = decode_input(encoded_data);

    let id_result = read_input_id(&input).unwrap();

    let source = format!("{}{}", "rtmp://localhost/live/", id_result);
    let mut args = vec![
        "-y",//Overwrite output files without asking.
        "-i",// Input
        source.as_str(),
    ];

    let qcif_variant = VideoVariants::QCIF.get_variant();
    let cif_variant = VideoVariants::CIF.get_variant();
    let qsd_variant = VideoVariants::QSD.get_variant();
    let sd_variant = VideoVariants::SD.get_variant();
    let hd_variant = VideoVariants::HD.get_variant();
    let fhd_variant = VideoVariants::FHD.get_variant();
    let uhd_variant = VideoVariants::UHD.get_variant();


//    let directory = format!("{}", &input);
    let relative_path = format!("{}{}", relative_root_path, &input);
//    let quality_path = format!("{}/{}", &input, &hd_variant.display_name);
//    let output_path = format!("{}/manifest.m3u8", &quality_path);
//    let relative_quality_path = format!("./{}", &quality_path);
    let playlist_path = format!("{}/playlist.m3u8", &relative_path);

    dbg!(&input);
//    dbg!(&output_path);
    dbg!(&relative_path);
//    dbg!(&quality_path);
//    dbg!(&relative_quality_path);

//    let segment_filename = format!("{}/%03d.ts", quality_path);

    if cli.uhd {
        let uhd_args = uhd_variant.get_args(input.as_str(), root_path);
        args.extend_from_slice(&uhd_args);
    };
    if cli.fhd {
        let fhd_args = fhd_variant.get_args(input.as_str(), root_path);
        args.extend_from_slice(&fhd_args);
    };
    if cli.hd {
        let hd_args = hd_variant.get_args(input.as_str(), root_path);
        args.extend_from_slice(&hd_args);
    };
    if cli.sd {
        let sd_args = sd_variant.get_args(input.as_str(), root_path);
        args.extend_from_slice(&sd_args);
    };
    if cli.qsd {
        let qsd_args = qsd_variant.get_args(input.as_str(), root_path);
        args.extend_from_slice(&qsd_args);
    };
    if cli.cif {
        let cif_args = cif_variant.get_args(input.as_str(), root_path);
        args.extend_from_slice(&cif_args);
    };
    if cli.qcif {
        let qcif_args = qcif_variant.get_args(input.as_str(), root_path);
        args.extend_from_slice(&qcif_args);
    };

    dbg!(&args);

    dbg!(&playlist_path);
    fs::write(&playlist_path, &playlist).unwrap();


    let child = Command::new(FFMPEG)
        .args(&args)
        .spawn()
        .expect("Command failed to start");

    let output = child
        .wait_with_output()
        .expect("failed to wait on child");
    println!("status: {}", output.status);
    io::stdout().write_all(&output.stdout).unwrap();
    io::stderr().write_all(&output.stderr).unwrap();

    assert!(output.status.success());
}

fn decode_input(string: &String) -> String {
    use percent_encoding::percent_decode;
    let decoded = percent_decode(string.as_bytes()).decode_utf8().unwrap();
    println!("{}", decoded);
    return decoded.to_string();
}


fn check_playlist_outputs(playlist: &mut String, cli: &Cli) -> bool {
    let mut any_output = false;
    // order of adding matters!
    if cli.uhd {
        playlist.push_str("#EXT-X-STREAM-INF:BANDWIDTH=7000000,RESOLUTION=3840x2160\n2160P/manifest.m3u8\n");
        any_output = true;
    };
    if cli.fhd {
        playlist.push_str("#EXT-X-STREAM-INF:BANDWIDTH=5000000,RESOLUTION=1920x1080\n1080P/manifest.m3u8\n");
        any_output = true;
    };
    if cli.hd {
        playlist.push_str("#EXT-X-STREAM-INF:BANDWIDTH=2800000,RESOLUTION=1280x720\n720P/manifest.m3u8\n");
        any_output = true;
    };
    if cli.sd {
        playlist.push_str("#EXT-X-STREAM-INF:BANDWIDTH=1400000,RESOLUTION=842x480\n480P/manifest.m3u8\n");
        any_output = true;
    };
    if cli.qsd {
        playlist.push_str("#EXT-X-STREAM-INF:BANDWIDTH=800000,RESOLUTION=640x360\n360P/manifest.m3u8\n");
        any_output = true;
    };
    if cli.cif {
        playlist.push_str("#EXT-X-STREAM-INF:BANDWIDTH=600000,RESOLUTION=426x240\n240P/manifest.m3u8\n");
        any_output = true;
    };
    if cli.qcif {
        playlist.push_str("#EXT-X-STREAM-INF:BANDWIDTH=400000,RESOLUTION=256x144\n144P/manifest.m3u8\n");
        any_output = true;
    };
    return any_output;
}


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
    fn get_args(&self, input: &str, root_path: &Option<&String>) -> Vec<&str> {
        let relative_root_path = root_path.map_or("./", String::as_str);
        let root_path_str = root_path.map_or("", String::as_str);

        let quality_path = format!("{}{}/{}", root_path_str, input, &self.display_name);
        let relative_quality_path = format!("{}{}/{}", relative_root_path, input, &self.display_name);
        dbg!(&relative_quality_path);
        fs::create_dir_all(&relative_quality_path).unwrap();


//        let video: String = format!("scale=w={}:h={}:force_original_aspect_ratio=decrease", &self.width, &self.height);
        let video: String = format!("scale=w={}:h={}", &self.width, &self.height);
        let segment_file = format!("{}/%03d.ts", &quality_path);
        let output = format!("{}/manifest.m3u8", &quality_path);

        let video_filter = string_to_static_str(video);
        let segment_filename = string_to_static_str(segment_file);
        let output_file = string_to_static_str(output);

        return vec!["-vf", video_filter, "-c:a", "aac", "-b:a", self.audio_bitrate, "-c:v", "libx264",
                    "-b:v", self.bitrate_low_motion, "-maxrate", self.bitrate_high_motion,
                    "-sc_threshold", "0", "-g", "48", "-keyint_min", "48", "-hls_time", "6",
                    "-hls_playlist_type", "event", "-hls_flags", "append_list", "-hls_segment_filename", segment_filename, output_file];
    }
}

/// Magic!
fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
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
}

#[derive(Debug, StructOpt)]
/// EXAMPLE USAGE: ./live-helper-service --hd --sd KwLzl%2Fprocessed%2Fvideo%2F5036160f-8aed-49bb-b2da-3375ba113f55
///
/// example folder name: KwLzl/processed/video/5036160f-8aed-49bb-b2da-3375ba113f55
///
/// example file name: KwLzl/processed/video/5036160f-8aed-49bb-b2da-3375ba113f55/480P/012.ts
struct Cli {
    /// Percent decoded file path that ends with video id.
    input: String,
    /// Root path for all files and subdirectories that come from input argument.
    /// default is relative path './'; Example '/home/user'
    root_path: Option<String>,
    /// Enables 144P output
    #[structopt(long)]
    qcif: bool,
    /// Enables 240P output
    #[structopt(long)]
    cif: bool,
    /// Enables 360P output
    #[structopt(long)]
    qsd: bool,
    /// Enables 480P output
    #[structopt(long)]
    sd: bool,
    /// Enables 720P output
    #[structopt(long)]
    hd: bool,
    /// Enables 1080P output
    #[structopt(long)]
    fhd: bool,
    /// Enables 2160P output
    #[structopt(long)]
    uhd: bool,

}
