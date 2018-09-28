extern crate image;

use std::path::Path;
use image::GenericImageView;
use image::ImageError;
use image::Rgba;
use std::io;

fn invert(a: Rgba<u8>) -> Rgba<u8> {
    Rgba([
        255 - a.data[0],
        255 - a.data[1],
        255 - a.data[2],
        255 - a.data[3]
    ])
}

fn distance(a: Rgba<u8>, b: Rgba<u8>) -> u32 {
    let rdist = ((a.data[0] as i32) - (b.data[0] as i32)).wrapping_abs();
    let gdist = ((a.data[1] as i32) - (b.data[1] as i32)).wrapping_abs();
    let bdist = ((a.data[2] as i32) - (b.data[2] as i32)).wrapping_abs();
    let adist = ((a.data[3] as i32) - (b.data[3] as i32)).wrapping_abs();
    (rdist + gdist + bdist) as u32
}

fn blend(a: u8, b: u8, thres: u8) -> u8 {
    let ia: i32 = a as i32;
    let ib: i32 = b as i32;
    let diff = (ia - ib).wrapping_abs();
    if diff >= (thres as i32) {
        b
    } else {
        ((ia + ib) / 2) as u8
    }
}

pub struct FramePair {
    a: image::DynamicImage,
    b: image::DynamicImage,
    pub out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl FramePair {
    pub fn new(fna: String, fnb: String) -> Result<Self, ImageError> {
        let ima = image::open(Path::new(&fna))?;
        let imb = image::open(Path::new(&fnb))?;
        let iout = image::ImageBuffer::new(ima.width(), ima.height());

        Ok(FramePair { a: ima, b: imb, out: iout })
    }

    pub fn generate_iframe(&mut self, thres: u8, dist: i32) {
        let (w, h) = self.a.dimensions();

        for x in 0..w {
            for y in 0..h {
                let a_color: Rgba<u8> = self.a.get_pixel(x, y);
                let mut nearest: Rgba<u8> = self.b.get_pixel(x,y);
                self.out.put_pixel(x,y,nearest);
                let mut xloc: u32 = x;
                let mut yloc: u32 = y;

                for xoff in -dist..dist {
                    for yoff in -dist..dist {
                        let xf = (x as i32) + xoff;
                        let yf = (y as i32) + yoff;
                        //OOB WIDTH
                        if (xf < 0 || xf > (w as i32) - 1) { continue; }
                        //OOB HEIGHT
                        if (yf < 0 || yf > (h as i32) - 1) { continue; }
                        //ORIGIN
                        if (xoff == 0 && yoff == 0) { continue; }

                        let b_color = self.b.get_pixel(xf as u32, yf as u32);

                        if (distance(a_color, b_color) < distance(a_color, nearest)) {
                            nearest = b_color;
                            xloc = xf as u32;
                            yloc = yf as u32;
                        }
                    }
                }

                let mpx = (x + xloc)/2;
                let mpy = (y + yloc)/2;

                self.out.put_pixel(mpx, mpy,
                Rgba([
                    blend(a_color.data[0], nearest.data[0], thres),
                    blend(a_color.data[1], nearest.data[1], thres),
                    blend(a_color.data[2], nearest.data[2], thres),
                    blend(a_color.data[3], nearest.data[3], thres)
                ]));
            }
        }
    }

    pub fn save_aframe(&self, path: &Path) -> io::Result<()>
    {
        self.a.save(path)
    }

    pub fn save_bframe(&self, path: &Path) -> io::Result<()>
    {
        self.b.save(path)
    }

    pub fn save_iframe(&self, path: &Path) -> io::Result<()>
    {
        self.out.save(path)
    }
}


