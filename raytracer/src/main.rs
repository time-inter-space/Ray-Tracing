mod vec3;
use vec3::*;

mod ray;
use ray::*;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::{fs::File, process::exit};

fn hit_sphere(center: Point3, radius: f64, r: Ray) -> bool {
    let oc = r.origin() - center;
    let a = dot(r.direction(), r.direction());
    let b = 2.0 * dot(oc, r.direction());
    let c = dot(oc, oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}
fn ray_color(r: Ray) -> Color {
    if hit_sphere(
        Point3 {
            e0: 0.0,
            e1: 0.0,
            e2: -1.0,
        },
        0.5,
        r,
    ) {
        return Color {
            e0: 1.0,
            e1: 0.0,
            e2: 0.0,
        };
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.e1 + 1.0);
    Color {
        e0: 1.0,
        e1: 1.0,
        e2: 1.0,
    }
    .mul(1.0 - t)
        + Color {
            e0: 0.5,
            e1: 0.7,
            e2: 1.0,
        }
        .mul(t)
}

fn main() {
    let path = std::path::Path::new("output/book1/image3.jpg");
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
        e1: viewport_height,
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
