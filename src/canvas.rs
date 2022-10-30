use crate::tuple::*;

pub struct Canvas {
    pub width: u16,
    pub height: u16,
    pub pixels: Vec<Tuple>,
}

impl Canvas {
    pub fn new(width: u16, height: u16) -> Self {
        Canvas {
            width: width,
            height: height,
            pixels: vec![colour(0.0, 0.0, 0.0); (width as usize)*(height as usize)],
        }
    }

    const fn pixel_index(&self, col: u16, row: u16) -> usize {
        (col as usize)*(self.height as usize) + (row as usize)
    }

    pub fn pixel_at(&self, col: u16, row: u16) -> Tuple {
        self.pixels[self.pixel_index(col, row)]
    }

    pub fn write_pixel(&mut self, col: u16, row: u16, colour: Tuple){
        //EVENTUALLY: need to understand the borrow checker - getter func can't be used in both const and mutable contexts?
        self.pixels[(col as usize)*(self.height as usize) + (row as usize)] = colour;
    }

    pub fn to_ppm(&self) -> String {
        let mut str = "P3\n".to_owned();
        str += &self.width.to_string();
        str += " ";
        str += &self.height.to_string();
        str += "\n255\n";

        for pixel in &self.pixels {
            str += &pixel.red255().to_string();
            str += " ";
            str += &pixel.green255().to_string();
            str += " ";
            str += &pixel.blue255().to_string();
            str += "\n";
        }

        str
    }
}