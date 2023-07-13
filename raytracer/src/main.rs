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
use moving_sphere::*;

mod aabb;
use aabb::*;

mod bvh;
use bvh::*;

mod texture;
use texture::*;

mod perlin;
use perlin::*;

mod aarect;
use aarect::*;

mod cube;
use cube::*;

mod constant_medium;
use constant_medium::*;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;
use std::rc::Rc;
use std::{fs::File, process::exit};

fn ray_color(r: &Ray, background: Color, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let rec = world.hit(r, 0.001, f64::INFINITY);

    match rec {
        Some(x) => {
            let emitted = x.mat_ptr.emitted(x.u, x.v, x.p);
            let p = x.mat_ptr.scatter(r, &x);
            match p {
                Some(x) => emitted + x.first * ray_color(&x.second, background, world, depth - 1),
                None => emitted,
            }
        }
        None => background,
    }
}
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
/*fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new(4.0));
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
}*/
/*fn earth() -> HittableList {
    let earth_texture = Rc::new(ImageTexture::new("input/earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian {
        albedo: earth_texture,
    });
    let globe = Rc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut objects = HittableList::new();
    objects.add(globe);
    objects
}*/
/*fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new(4.0));
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

    let difflight = Rc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    objects.add(Rc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    objects
}*/
/*fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    objects.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Rc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Rc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    let mut box1: Rc<dyn Hittable> = Rc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let mut box2: Rc<dyn Hittable> = Rc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);

    objects
}*/
/*fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red = Rc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Rc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Rc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    objects.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Rc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Rc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    objects.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Rc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Rc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let mut box1: Rc<dyn Hittable> = Rc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Rc::new(RotateY::new(box1, 15.0));
    box1 = Rc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Rc<dyn Hittable> = Rc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Rc::new(RotateY::new(box2, -18.0));
    box2 = Rc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    objects.add(Rc::new(ConstantMedium::new_c(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    objects.add(Rc::new(ConstantMedium::new_c(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    objects
}*/
fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Rc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + (i as f64) * w;
            let z0 = -1000.0 + (j as f64) * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_rng(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Rc::new(Cube::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::new();

    objects.add(Rc::new(BvhNode::new(
        &boxes1.objects,
        0,
        boxes1.objects.len(),
        0.0,
        1.0,
    )));

    let light = Rc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    objects.add(Rc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Rc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    objects.add(Rc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Rc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Rc::new(Dielectric::new(1.5)),
    )));
    objects.add(Rc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Rc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundary = Rc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Rc::new(ConstantMedium::new_c(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    boundary = Rc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Rc::new(Dielectric::new(1.5)),
    ));
    objects.add(Rc::new(ConstantMedium::new_c(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Rc::new(Lambertian {
        albedo: Rc::new(ImageTexture::new("input/earthmap.jpg")),
    });
    objects.add(Rc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Rc::new(NoiseTexture::new(0.1));
    objects.add(Rc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Rc::new(Lambertian { albedo: pertext }),
    )));

    let mut boxes2 = HittableList::new();
    let white = Rc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Rc::new(Sphere::new(
            random_vec3_rng(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    objects.add(Rc::new(Translate::new(
        Rc::new(RotateY::new(
            Rc::new(BvhNode::new(
                &boxes2.objects,
                0,
                boxes2.objects.len(),
                0.0,
                1.0,
            )),
            15.0,
        )),
        Vec3::new(-100.0, 270.0, 395.0),
    )));

    objects
}

fn main() {
    let path = std::path::Path::new("output/book2/image22.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio = 1.0;
    let image_width = 800;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 50;
    let max_depth = 5000;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let world = final_scene();
    let lookfrom = Point3::new(478.0, 278.0, -600.0);
    let lookat = Point3::new(278.0, 278.0, 0.0);
    let vfov = 40.0;
    let aperture = 0.0;
    let background = Color::new(0.0, 0.0, 0.0);

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
                pixel_color = pixel_color + ray_color(&r, background, &world, max_depth);
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
