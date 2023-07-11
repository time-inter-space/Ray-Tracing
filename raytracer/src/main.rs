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

mod moving_sphere;
//use moving_sphere::*;

mod aabb;
use aabb::*;

mod bvh;
//use bvh::*;

mod texture;
use texture::*;

mod perlin;
use perlin::*;

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
            let p = x.mat_ptr.scatter(r, &x);
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
fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new());
    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian {
            albedo: pertext.clone(),
        }),
    )));
    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian { albedo: pertext }),
    )));

    objects
}
/*fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker = Rc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian {
            albedo: checker.clone(),
        }),
    )));
    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian { albedo: checker }),
    )));

    objects
}*/
/*fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    /*let ground_material = Rc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));*/

    let checker = Rc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian { albedo: checker }),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                (a as f64) + 0.9 * random_double(),
                0.2,
                (b as f64) + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    let albedo = random_vec3() * random_vec3();
                    let center2 = center + Vec3::new(0.0, random_double_rng(0.0, 0.5), 0.0);
                    world.add(Rc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Rc::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec3_rng(0.5, 1.0);
                    let fuzz = random_double_rng(0.0, 0.5);
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    world.add(Rc::new(Sphere::new(
                        center,
                        0.2,
                        Rc::new(Dielectric::new(1.5)),
                    )))
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}*/

fn main() {
    let path = std::path::Path::new("output/book2/image7.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let world = two_perlin_spheres();
    let lookfrom = Point3::new(13.0, 2.0, 3.0);
    let lookat = Point3::new(0.0, 0.0, 0.0);
    let vfov = 20.0;
    let aperture = 0.0;

    let vup = Vec3::new(0.0, 1.0, 0.0);
    let dist_to_focus = 10.0;

    let cam = Camera::new(
        lookfrom,
        lookat,
        vup,
        vfov,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    let progress = if option_env!("CI").unwrap_or_default() == "true" {
        ProgressBar::hidden()
    } else {
        ProgressBar::new((image_height * image_width) as u64)
    };

    for j in (0..image_height).rev() {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, image_height - j - 1);

            let mut pixel_color = Color::new(0.0, 0.0, 0.0);
            for _s in 0..samples_per_pixel {
                let u = ((i as f64) + random_double()) / ((image_width - 1) as f64);
                let v = ((j as f64) + random_double()) / ((image_height - 1) as f64);
                let r = cam.get_ray(u, v, 0.0, 1.0);
                pixel_color = pixel_color + ray_color(&r, &world, max_depth);
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
