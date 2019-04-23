pub struct Block {
    pub x1 : i32,
    pub x2 : i32,
    pub y1 : i32,
    pub y2 : i32,
}

impl Block {
    pub fn new(x: i32, y: i32, width: i32, height: i32) -> Self
    {
        Block {x1: x, x2: x + width, y1: y, y2: y + height}
    }

    pub fn origin(&self) -> (i32, i32) {
        (self.x1, self.y1)
    }

    pub fn width(&self) -> i32 {
        self.x2-self.x1
    }

    pub fn height(&self) -> i32 {
        self.y2-self.y1
    }

    pub fn max_difference(&self) -> i32 {
        let MAGIC : i32 = 195_075;
        MAGIC
    }
}