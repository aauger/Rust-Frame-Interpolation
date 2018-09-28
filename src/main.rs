extern crate image;
mod frame_pair;
use frame_pair::FramePair;
use std::string::String;
use std::path::Path;
use std::fs;

fn main() {
    //TODO: these ought to be user provided arguments
    //Perhaps via a conf file, for mo-vec settings later?
    let threshold: u8 = 255;
    let infolder: String = String::from("..\\fstore\\inf\\");
    let outfolder: String = String::from("..\\fstore\\outf\\");

    let mut rframecount: u64 = 1;
    let mut oframecount: u64 = 1;
    let fcount: usize = match fs::read_dir(Path::new(&infolder)) {
        Ok(dir) => dir.count(),
        _ => panic!("Directory not found")
    };
    println!("count: {}", fcount);

    for _ in 0..fcount-1 {
        if let Ok(mut fp) = FramePair::new(
            format!("{}{}.png", infolder, rframecount),
            format!("{}{}.png", infolder, rframecount+1)
        ) {
            fp.generate_iframe(threshold);

            match fp.save_aframe(Path::new(&format!("{}{}.png", outfolder, oframecount))) {
                Err(_e) => panic!("Error saving A frame"),
                _ => ()
            }

            match fp.save_iframe(Path::new(&format!("{}{}.png", outfolder, oframecount+1))) {
                Err (_e) => panic!("Error saving I frame"),
                _ => ()
            }
        };

        oframecount += 2;
        rframecount += 1;
    }
}
