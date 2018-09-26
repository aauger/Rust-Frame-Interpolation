extern crate image;
use std::path::Path;
use image::GenericImageView;
use image::Rgba;
use std::io;

fn blend(a: u8, b: u8, thres: u8) -> u8
{
    let ia: i32 = a as i32;
    let ib: i32 = b as i32;
    let diff = (ia-ib).wrapping_abs();
    if diff >= (thres as i32) {
        b
    }
    else {
        ((ia + ib)/2) as u8
    }
}

pub struct FramePair {
    a: image::DynamicImage,
    b: image::DynamicImage,
    pub out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>
}

impl FramePair {
    pub fn new(fna: String, fnb: String) -> Self
    {
        let ima = image::open(Path::new(&fna)).unwrap(); //FIXME
        let imb = image::open(Path::new(&fnb)).unwrap(); //FIXME
        let iout = image::ImageBuffer::new(ima.width(), ima.height());

        FramePair { a: ima, b: imb, out: iout }
    }

    pub fn generate_iframe(&mut self, thres : u8)
    {
        let (w,h) = self.a.dimensions();

        for x in 0..w {
            for y in 0..h {
                let a_color: Rgba<u8> = self.a.get_pixel(x, y);
                let b_color: Rgba<u8> = self.b.get_pixel(x, y);
                let n_color: Rgba<u8> = Rgba(
                    [
                        blend(a_color.data[0], b_color.data[0], thres), //R
                        blend(a_color.data[1], b_color.data[1], thres), //G
                        blend(a_color.data[2], b_color.data[2], thres), //B
                        blend(a_color.data[3], b_color.data[3], thres)  //A
                    ]
                );
                self.out.put_pixel(x,y,n_color);
            }
        }
    }

    pub fn save_aframe(&self, path : &Path) -> io::Result<()>
    {
        self.a.save(path)
    }

    pub fn save_bframe(&self, path : &Path) -> io::Result<()>
    {
        self.b.save(path)
    }

    pub fn save_iframe(&self, path : &Path) -> io::Result<()>
    {
        self.out.save(path)
    }
}


