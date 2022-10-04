use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel {r, g, b}
    }

    pub fn from_vec3(v: &Vec3) -> Pixel {
        Pixel::new(
            Pixel::clamp(v.x),
            Pixel::clamp(v.y),
            Pixel::clamp(v.z)
        )
    }

    fn clamp(v: f64) -> u8 {
        (if v < 0.0 { 0.0 } else if v > 255.0 { 255.0 } else { v }) as u8
    }

    pub fn black() -> Pixel {
        Pixel::new(0,0,0)
    }
}

pub struct Image {
    height: usize,
    width: usize,
    pixels: Vec<Pixel>
}

impl Image {
   pub fn new(height: usize, width: usize) -> Image {
       Image {
           height,
           width,
           pixels: vec![Pixel::new(0, 0, 0); height*width]
       }
   }

   pub fn at(&self, x: usize, y: usize) -> &Pixel {
       let index = y*self.width + x;
       return &self.pixels[index];
   }

   pub fn insert_pixel_at(&mut self, x: usize, y: usize, pixel: &Pixel) {
       self.pixels[y*self.width + x] = Pixel::new(pixel.r, pixel.g, pixel.b);
   }
   pub fn to_ppm(&self) -> String {
       let mut ppm = format!("P3\n{} {}\n255\n", self.width, self.height);
       for y in 0..self.height {
           for x in 0..self.width {
               let pixel = self.at(x, y);
               ppm.push_str(&format!("{} {} {}\n", pixel.r, pixel.g, pixel.b));
           } 
       }
       ppm
   }

   pub fn height(&self) -> usize {
    self.height
   }

   pub fn width(&self) -> usize{
    self.width
   }
}