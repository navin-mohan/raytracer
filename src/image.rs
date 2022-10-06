use crate::vec3::Vec3;

#[derive(Clone)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Pixel {
    pub fn new(r: u8, g: u8, b: u8) -> Pixel {
        Pixel { r, g, b }
    }

    pub fn from_vec3(v: &Vec3) -> Pixel {
        Pixel::new(Pixel::clamp(v.x), Pixel::clamp(v.y), Pixel::clamp(v.z))
    }

    fn clamp(v: f64) -> u8 {
        (if v < 0.0 {
            0.0
        } else if v > 255.0 {
            255.0
        } else {
            v
        }) as u8
    }

    pub fn black() -> Pixel {
        Pixel::new(0, 0, 0)
    }
}

pub struct Image {
    height: usize,
    width: usize,
    pixels: Vec<Pixel>,
}

impl Image {
    pub fn new(height: usize, width: usize, pixels: Vec<Pixel>) -> Image {
        Image {
            height,
            width,
            pixels,
        }
    }

    pub fn at(&self, x: usize, y: usize) -> &Pixel {
        let index = x * self.width + y;
        return &self.pixels[index];
    }

    pub fn to_ppm(&self) -> String {
        let h_range = 0..self.height;
        let w_range = 0..self.width;

        let body: String = h_range
            .flat_map(|x| w_range.clone().map(move |y| (x, y)))
            .map(|(x, y)| self.at(x, y))
            .map(|pixel| format!("{} {} {}\n", pixel.r, pixel.g, pixel.b))
            .collect();

        format!("P3\n{} {}\n255\n {body}", self.width, self.height)
    }

    pub fn to_js_image_data(&self) -> Vec<u8> {
        self.pixels.iter().flat_map(|pixel| vec![pixel.r, pixel.g, pixel.b, 255]).collect()
    }
}
