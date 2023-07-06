mod vec3;
use vec3::*;

mod ray;
use ray::*;

mod sphere;
use sphere::*;

mod hittable;
use hittable::*;

mod hittable_list;
use hittable_list::*;

mod rtweekend;
use rtweekend::*;

mod camera;
use camera::*;

mod material;
use material::*;

mod pair;
use pair::*;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::rc::Rc;
use std::{fs::File, process::exit};

fn ray_color(r: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }
    let rec = world.hit(r, 0.001, f64::INFINITY);
    match rec {
        Some(x) => {
            let p = (*x.mat_ptr).scatter(r, &x);
            match p {
                Some(x) => {
                    return x.first * ray_color(&x.second, world, depth - 1);
                }
                None => {
                    return Color::new(0.0, 0.0, 0.0);
                }
            }
        }
        None => {}
    }
    let unit_direction = unit_vector(r.direction());
    let t = 0.5 * (unit_direction.e1 + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    let path = std::path::Path::new("output/book1/image12.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.3)));
    let material_left = Rc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3));
    let material_right = Rc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        material_center,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Rc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let cam = Camera::new();

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, image_height - j - 1);

            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            let mut s = 0;
            loop {
                if s >= samples_per_pixel {
                    break;
                }
                let u = ((i as f64) + random_double()) / ((image_width - 1) as f64);
                let v = ((j as f64) + random_double()) / ((image_height - 1) as f64);
                let r = cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
                s += 1;
            }
            let mut r = pixel_color.e0;
            let mut g = pixel_color.e1;
            let mut b = pixel_color.e2;
            let scale = 1.0 / (samples_per_pixel as f64);
            r = (scale * r).sqrt();
            g = (scale * g).sqrt();
            b = (scale * b).sqrt();
            *pixel = image::Rgb([
                (256.0 * clamp(r, 0.0, 0.999)) as u8,
                (256.0 * clamp(g, 0.0, 0.999)) as u8,
                (256.0 * clamp(b, 0.0, 0.999)) as u8,
            ]);
            progress.inc(1);
        }
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
