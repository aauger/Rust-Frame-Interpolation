extern crate image;
use std::path::Path;
use image::GenericImageView;
use image::ImageError;
use image::Rgba;
use std::io;
use block::Block;
use std::collections::HashMap;

fn invert(a: Rgba<u8>) -> Rgba<u8> {
    Rgba([
        255 - a.data[0],
        255 - a.data[1],
        255 - a.data[2],
        /*255 - a.data[3]*/
        255
    ])
}

fn similarity(a: Rgba<u8>, b: Rgba<u8>) -> i32 {
    765 - (distance(a,b) as i32)
}

fn distance(a: Rgba<u8>, b: Rgba<u8>) -> i32 {

    let rdist = ((a.data[0] as i32) - (b.data[0] as i32)) *
        ((a.data[0] as i32) - (b.data[0] as i32));
    let gdist = ((a.data[1] as i32) - (b.data[1] as i32)) *
        ((a.data[1] as i32) - (b.data[1] as i32));
    let bdist = ((a.data[2] as i32) - (b.data[2] as i32)) *
        ((a.data[2] as i32) - (b.data[2] as i32));
    //let adist = ((a.data[3] as i32) - (b.data[3] as i32)).wrapping_abs();
    (rdist + gdist + bdist)
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
    blkSize: i32,
    pub out: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl FramePair {
    pub fn new(fna: String, fnb: String, block_size: i32) -> Result<Self, ImageError> {
        let ima = image::open(Path::new(&fna))?;
        let imb = image::open(Path::new(&fnb))?;
        let iout = image::ImageBuffer::new(ima.width(), ima.height());
        let (w,h) = ima.dimensions();

        Ok(FramePair { a: ima, b: imb, blkSize: block_size, out: iout })
    }

    pub fn generate_iframe(&mut self, dist: i32) {
        let (width,height) = (self.a.width() as i32, self.a.height() as i32);
        let mut blocks : Vec<Block>  = Vec::new();

//        for x in 0..self.b.width() {
//            for y in 0..self.b.height() {
//                self.out.put_pixel(x,y,self.b.get_pixel(x,y));
//            }
//        }

        for x in 0..(self.a.width()/(self.blkSize as u32)) {
            for y in 0..(self.a.height()/(self.blkSize as u32)) {
                let xi : i32 = x as i32;
                let yi : i32 = y as i32;
                blocks.push(Block::new(xi*self.blkSize, yi*self.blkSize, self.blkSize, self.blkSize));
            }
        }

        for b in blocks {
            let (ox,oy) = b.origin();
            let mut best_neighbor : Option<(i32,i32)> = None;
            let mut smallest_difference : Option<i32> = None;

            for x_off in -dist..dist {
                for y_off in -dist..dist {
                    //let mut num_sampled = 0;
                    let mut difference = 0;
                    let x_origin = b.x1;
                    let y_origin = b.y1;

                    for x in x_origin..x_origin + b.width() {
                        for y in y_origin..y_origin + b.height() {
                            //OOB width
                            if (x < 0 || x > width - 1) { difference += b.max_difference(); continue; }
                            if (x + x_off < 0 || x + x_off > width - 1) { difference += b.max_difference(); continue; }
                            //OOB height
                            if (y < 0 || y > height - 1) { difference += b.max_difference(); continue; }
                            if (y + y_off < 0 || y + y_off > height - 1) { difference += b.max_difference(); continue; }

                            let ux = x as u32;
                            let uy = y as u32;
                            let fx = (x + x_off) as u32;
                            let fy = (y + y_off) as u32;
                            
                            //println!("{} {} {} {}", ux, uy, fx, fy);

                            let colA : Rgba<u8> = self.a.get_pixel(ux, uy);
                            let colB : Rgba<u8> = self.b.get_pixel(fx, fy);

                            difference += distance(colA, colB);
                        }
                    }

                    if (smallest_difference == None || difference < smallest_difference.unwrap()) {
                        smallest_difference = Some(difference);
                        best_neighbor = Some((b.x1+x_off,b.y1+y_off));
                    }
                }
            }

            let (bx,by) = best_neighbor.unwrap();

            let nx = (((ox as f32) + (bx as f32)) / 2.0f32).round() as i32;
            let ny = (((oy as f32) + (by as f32)) / 2.0f32).round() as i32;

            for x in 0..b.width() {
                for y in 0..b.height() {

                    if (ox + x < 0 || ox + x > width-1) { continue; }
                    if (nx + x < 0 || nx + x > width-1) { continue; }

                    if (oy + y < 0 || oy + y > height-1) { continue; }
                    if (ny + y < 0 || ny + y > height-1) { continue; }

                    //if (nx_origin + x < 0 || nx_origin + x > width-1) { continue; }
                    //if (ny_origin + y < 0 || ny_origin + y > height-1) { continue; }

                    //let colA : Rgba<u8> = self.a.get_pixel( ((ox + x) as u32), ((oy + y) as u32) );
                    let colB : Rgba<u8> = self.b.get_pixel( ((nx + x) as u32), ((ny + y) as u32) );


                    self.out.put_pixel(
                        (ox+x) as u32,
                        (oy+y) as u32,
//                        Rgba([
//                            blend(colA.data[0], colB.data[0], 255),
//                            blend(colA.data[1], colB.data[1], 255),
//                            blend(colA.data[2], colB.data[2], 255),
//                            blend(colA.data[3], colB.data[3], 255)
//                        ]));
                        colB);
                }
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


