use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Vec3 {
    e0: f64,
    e1: f64,
    e2: f64,
}
impl std::ops::Add for Vec3 {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self {
            e0: self.e0 + other.e0,
            e1: self.e1 + other.e1,
            e2: self.e2 + other.e2,
        }
    }
}
impl std::ops::Sub for Vec3 {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self {
            e0: self.e0 - other.e0,
            e1: self.e1 - other.e1,
            e2: self.e2 - other.e2,
        }
    }
}
impl Vec3 {
    fn mul(self, other: f64) -> Self {
        Self {
            e0: self.e0 * other,
            e1: self.e1 * other,
            e2: self.e2 * other,
        }
    }
    fn div(self, other: f64) -> Self {
        Self {
            e0: self.e0 / other,
            e1: self.e1 / other,
            e2: self.e2 / other,
        }
    }
    fn length(self) -> f64 {
        self.length_squared().sqrt()
    }
    fn length_squared(self) -> f64 {
        self.e0 * self.e0 + self.e1 * self.e1 + self.e2 * self.e2
    }
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Ray {
    orig: Vec3,
    dir: Vec3,
}
impl Ray {
    /*fn at(self, t: f64) -> Vec3 {
        self.orig + self.dir.mul(t)
    }
    fn origin(self) -> Vec3 {
        self.orig
    }*/
    fn direction(self) -> Vec3 {
        self.dir
    }
}
fn unit_vector(v: Vec3) -> Vec3 {
    v.div(v.length())
}
fn ray_color(r: Ray) -> Vec3 {
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.e1 + 1.0);
    Vec3 {
        e0: 1.0,
        e1: 1.0,
        e2: 1.0,
    }
    .mul(1.0 - t)
        + Vec3 {
            e0: 0.5,
            e1: 0.7,
            e2: 1.0,
        }
        .mul(t)
}

fn main() {
    let path = std::path::Path::new("output/book1/image2.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3 {
        e0: 0.0,
        e1: 0.0,
        e2: 0.0,
    };
    let horizontal = Vec3 {
        e0: viewport_width,
        e1: 0.0,
        e2: 0.0,
    };
    let vertical = Vec3 {
        e0: 0.0,
        e1: viewport_width,
        e2: 0.0,
    };
    let lower_left_corner = origin
        - horizontal.div(2.0)
        - vertical.div(2.0)
        - Vec3 {
            e0: 0.0,
            e1: 0.0,
            e2: focal_length,
        };

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, image_height - j - 1);

            let u = (i as f64) / ((image_width - 1) as f64);
            let v = (j as f64) / ((image_height - 1) as f64);
            let r = Ray {
                orig: origin,
                dir: lower_left_corner + horizontal.mul(u) + vertical.mul(v) - origin,
            };
            let pixel_color = ray_color(r);
            *pixel = image::Rgb([
                (255.999 * pixel_color.e0) as u8,
                (255.999 * pixel_color.e1) as u8,
                (255.999 * pixel_color.e2) as u8,
            ]);
        }
        progress.inc(1);
    }
    progress.finish();

    println!(
        "Ouput image as \"{}\"",
        style(path.to_str().unwrap()).yellow()
    );
    let output_image = image::DynamicImage::ImageRgb8(img);
    let mut output_file = File::create(path).unwrap();
    match output_image.write_to(&mut output_file, image::ImageOutputFormat::Jpeg(quality)) {
        Ok(_) => {}
        Err(_) => println!("{}", style("Outputting image fails.").red()),
    }

    exit(0);
}
