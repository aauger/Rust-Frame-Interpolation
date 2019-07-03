mod frame_pair;
mod block;

extern crate image;
use frame_pair::FramePair;
use std::string::String;
use std::path::Path;
use std::fs;

fn main() {
    //TODO: these ought to be user provided arguments
    //Perhaps via a conf file, for mo-vec settings later?
    let threshold: u8 = 255;
    let distance: i32 = 2;
    let block_size: i32 = 2;
    let infolder: String = String::from("fstore\\inf\\");
    let outfolder: String = String::from("fstore\\outf\\");
    let mut rframecount: u64 = 1;
    let mut oframecount: u64 = 1;

    println!("{}", std::env::current_dir().unwrap().to_str().unwrap());

    let fcount: usize = match fs::read_dir(Path::new(&infolder)) {
        Ok(dir) => dir.count(),
        _ => panic!("Directory not found")
    };
    println!("count: {}", fcount);

    for _ in 0..fcount-1 {
        if let Ok(mut fp) = FramePair::new(
            format!("{}{}.png", infolder, rframecount),
            format!("{}{}.png", infolder, rframecount+1),
            block_size
        ) {
            fp.generate_iframe(distance);

            match fp.save_aframe(Path::new(&format!("{}{}.png", outfolder, oframecount))) {
                Err(_e) => panic!("Error saving A frame"),
                _ => ()
            }

            match fp.save_iframe(Path::new(&format!("{}{}.png", outfolder, oframecount+1))) {
                Err (_e) => panic!("Error saving I frame"),
                _ => ()
            }

            println!("{}/{}", oframecount, fcount*2);
        };

        oframecount += 2;
        rframecount += 1;
    }
}
