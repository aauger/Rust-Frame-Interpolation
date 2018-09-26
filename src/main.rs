extern crate image;
mod frame_pair;
use std::string::String;
use std::path::Path;

fn main() {

    let threshold: u8 = 255;
    let infolder: String = String::from("..\\fstore\\inf\\");
    let outfolder: String = String::from("..\\fstore\\outf\\");

    let mut rframecount: u64 = 1;
    let mut oframecount: u64 = 1;
    println!("{:?}", std::env::current_dir().unwrap()); //FIXME
    let fcount = std::fs::read_dir(Path::new(&infolder)).unwrap().count(); //FIXME
    println!("count: {}", fcount);

    for _ in 0..fcount-1{
        let mut fp = frame_pair::FramePair::new(
            String::from(format!("{}{}.png", infolder, rframecount)),
            String::from(format!("{}{}.png", infolder, rframecount+1))
        );
        fp.generate_iframe(threshold);
        fp.save_aframe(Path::new
            (&String::from(format!("{}{}.png", outfolder, oframecount))));
        fp.save_iframe(Path::new
            (&String::from(format!("{}{}.png", outfolder, oframecount+1))));
        oframecount += 2;
        rframecount += 1;
        println!("{}/{}", oframecount, fcount*2);
    }
}
