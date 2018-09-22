extern crate image;
use std::path::Path;
use image::GenericImageView;
use image::DynamicImage;
use image::Rgba;

fn blend(a: u8, b: u8, thres: u8) -> u8
{
    let ia : i32 = a as i32;
    let ib : i32 = b as i32;
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
        let ima = image::open(Path::new(&fna)).unwrap();
        let imb = image::open(Path::new(&fnb)).unwrap();
        let iout = image::ImageBuffer::new(ima.width(), ima.height());

        FramePair { a: ima, b: imb, out: iout }
    }

    pub fn generate_interframe(&mut self, thres : u8)
    {
        for x in 0..self.a.width() {
            for y in 0..self.a.height(){
                let acolor: Rgba<u8> = self.a.get_pixel(x, y);
                let bcolor: Rgba<u8> = self.b.get_pixel(x, y);
                let ncolor: Rgba<u8> = Rgba(
                    [
                        blend(acolor.data[0], bcolor.data[0], thres),
                        blend(acolor.data[1], bcolor.data[1], thres),
                        blend(acolor.data[2], bcolor.data[2], thres),
                        blend(acolor.data[3], bcolor.data[3], thres)
                    ]
                );
                self.out.put_pixel(x,y,ncolor);
            }
        }
    }

    pub fn save_aframe(&self, path : &Path)
    {
        self.a.save(path);
    }

    pub fn save_bframe(&self, path : &Path)
    {
        self.b.save(path);
    }

    pub fn save_interframe(&self, path : &Path)
    {
        self.out.save(path);
    }
}


