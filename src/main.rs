extern crate image;
mod frame_pair;
use std::string::String;
use std::path::Path;

fn main() {
    let THRESHOLD: u8 = 255;
    let mut rframecount : u64 = 1;
    let mut oframecount : u64 = 1;
    println!("{:?}", std::env::current_dir().unwrap());
    let fcount = std::fs::read_dir(Path::new("fstore\\inf")).unwrap().count();
    println!("count: {}", fcount);

    for _ in 0..fcount{
        let mut fp = frame_pair::FramePair::new(
            String::from(format!("fstore\\inf\\{}.png", rframecount)),
            String::from(format!("fstore\\inf\\{}.png", rframecount+1))
        );
        fp.generate_interframe(THRESHOLD);
        fp.save_aframe(Path::new
            (&String::from(format!("fstore\\outf\\{}.png", oframecount))));
        fp.save_interframe(Path::new
            (&String::from(format!("fstore\\outf\\{}.png", oframecount+1))));
        oframecount += 2;
        rframecount += 1;
        println!("{}/{}", oframecount, fcount*2);
    }
}
