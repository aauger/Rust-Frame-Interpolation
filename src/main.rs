extern crate image;
mod frame_pair;
use std::string::String;
use std::path::Path;

fn main() {
    let THRESHOLD :u8 = 255;
    let INFILE_PATH : String = String::from("..\\fstore\\inf\\");
    let OUTFILE_PATH : String = String::from("..\\fstore\\outf\\");

    let mut rframecount : u64 = 1;
    let mut oframecount : u64 = 1;
    println!("{:?}", std::env::current_dir().unwrap());
    let fcount = std::fs::read_dir(Path::new(INFILE_PATH)).unwrap().count();
    println!("count: {}", fcount);

    for _ in 0..fcount-1{
        let mut fp = frame_pair::FramePair::new(
            String::from(format!("{}{}.png", INFILE_PATH, rframecount)),
            String::from(format!("{}{}.png", INFILE_PATH, rframecount+1))
        );
        fp.generate_interframe(THRESHOLD);
        fp.save_aframe(Path::new
            (&String::from(format!("{}{}.png", OUTFILE_PATH, oframecount))));
        fp.save_interframe(Path::new
            (&String::from(format!("{}{}.png", OUTFILE_PATH, oframecount+1))));
        oframecount += 2;
        rframecount += 1;
        println!("{}/{}", oframecount, fcount*2);
    }
}
