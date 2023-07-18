mod vec3;
use vec3::*;

mod ray;
use ray::*;

mod sphere;
//use sphere::*;

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

mod aarect;
use aarect::*;

mod cube;
use cube::*;

mod constant_medium;
//use constant_medium::*;

mod onb;
use onb::*;

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::{MultiProgress, ProgressBar};
use std::sync::{mpsc, Arc};
use std::thread::JoinHandle;
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
                Some(y) => {
                    let on_light = Point3::new(
                        random_double_rng(213.0, 343.0),
                        554.0,
                        random_double_rng(227.0, 332.0),
                    );
                    let mut to_light = on_light - x.p;
                    let distance_squared = to_light.length_squared();
                    to_light = unit_vector(to_light);

                    if dot(to_light, x.normal) < 0.0 {
                        return emitted;
                    }

                    let light_area = (343.0 - 213.0) * (332.0 - 227.0);
                    let light_cosine = to_light.e1.abs();
                    if light_cosine < 0.000001 {
                        return emitted;
                    }

                    let pdf = distance_squared / (light_cosine * light_area);
                    let scattered = Ray::new(x.p, to_light, r.time());
                    let albedo = y.first;

                    emitted
                        + albedo
                            * x.mat_ptr.scattering_pdf(r, &x, &scattered)
                            * ray_color(&scattered, background, world, depth - 1)
                            / pdf
                }
                None => emitted,
            }
        }
        None => background,
    }
}
/*fn random_scene() -> HittableList {
    let mut world = HittableList::new();

    /*let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));*/

    let checker = Arc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian { albedo: checker }),
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
                    world.add(Arc::new(MovingSphere::new(
                        center,
                        center2,
                        0.0,
                        1.0,
                        0.2,
                        Arc::new(Lambertian::new(albedo)),
                    )));
                } else if choose_mat < 0.95 {
                    let albedo = random_vec3_rng(0.5, 1.0);
                    let fuzz = random_double_rng(0.0, 0.5);
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Metal::new(albedo, fuzz)),
                    )));
                } else {
                    world.add(Arc::new(Sphere::new(
                        center,
                        0.2,
                        Arc::new(Dielectric::new(1.5)),
                    )))
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    world
}*/
/*fn two_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let checker = Arc::new(CheckerTexture::new(
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian {
            albedo: checker.clone(),
        }),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian { albedo: checker }),
    )));

    objects
}*/
/*fn two_perlin_spheres() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian {
            albedo: pertext.clone(),
        }),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian { albedo: pertext }),
    )));

    objects
}*/
/*fn earth() -> HittableList {
    let earth_texture = Arc::new(ImageTexture::new("input/earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian {
        albedo: earth_texture,
    });
    let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut objects = HittableList::new();
    objects.add(globe);
    objects
}*/
/*fn simple_light() -> HittableList {
    let mut objects = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian {
            albedo: pertext.clone(),
        }),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian { albedo: pertext }),
    )));

    let difflight = Arc::new(DiffuseLight::new(Color::new(4.0, 4.0, 4.0)));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    objects.add(Arc::new(XYRect::new(3.0, 5.0, 1.0, 3.0, -2.0, difflight)));

    objects
}*/
fn cornell_box() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(15.0, 15.0, 15.0)));

    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRect::new(
        213.0, 343.0, 227.0, 332.0, 554.0, light,
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    let mut box1: Arc<dyn Hittable> = Arc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    objects.add(box1);

    let mut box2: Arc<dyn Hittable> = Arc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    objects.add(box2);

    objects
}
/*fn cornell_smoke() -> HittableList {
    let mut objects = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));

    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 555.0, green)));
    objects.add(Arc::new(YZRect::new(0.0, 555.0, 0.0, 555.0, 0.0, red)));
    objects.add(Arc::new(XZRect::new(
        113.0, 443.0, 127.0, 432.0, 554.0, light,
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));
    objects.add(Arc::new(XZRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        white.clone(),
    )));
    objects.add(Arc::new(XYRect::new(
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable> = Arc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    ));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable> = Arc::new(Cube::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white,
    ));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    objects.add(Arc::new(ConstantMedium::new_c(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));
    objects.add(Arc::new(ConstantMedium::new_c(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    objects
}*/
/*fn final_scene() -> HittableList {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

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

            boxes1.add(Arc::new(Cube::new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut objects = HittableList::new();

    objects.add(Arc::new(BvhNode::new(
        &boxes1.objects,
        0,
        boxes1.objects.len(),
        0.0,
        1.0,
    )));

    let light = Arc::new(DiffuseLight::new(Color::new(7.0, 7.0, 7.0)));
    objects.add(Arc::new(XZRect::new(
        123.0, 423.0, 147.0, 412.0, 554.0, light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let moving_sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    objects.add(Arc::new(MovingSphere::new(
        center1,
        center2,
        0.0,
        1.0,
        50.0,
        moving_sphere_material,
    )));

    objects.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    objects.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let mut boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(boundary.clone());
    objects.add(Arc::new(ConstantMedium::new_c(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));
    boundary = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    objects.add(Arc::new(ConstantMedium::new_c(
        boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian {
        albedo: Arc::new(ImageTexture::new("input/earthmap.jpg")),
    });
    objects.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));
    let pertext = Arc::new(NoiseTexture::new(0.1));
    objects.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian { albedo: pertext }),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _j in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            random_vec3_rng(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    objects.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(
            Arc::new(BvhNode::new(
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
}*/

fn main() {
    let path = std::path::Path::new("output/book3/image4.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio = 1.0;
    let image_width = 600;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let samples_per_pixel = 100;
    let max_depth = 50;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let world = cornell_box();
    let lookfrom = Point3::new(278.0, 278.0, -800.0);
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

    let tot: usize = 8;
    let mut list: Vec<Vec<Pair<u32, u32>>> = Vec::new();
    for _i in 0..tot {
        list.push(Vec::new());
    }
    for j in 0..image_height {
        for i in 0..image_width {
            let num = random_int_rng(0, (tot - 1) as i32) as usize;
            list[num].push(Pair::new(j, i));
        }
    }
    let multi_progress = MultiProgress::new();
    let mut threads: Vec<JoinHandle<()>> = Vec::new();
    let mut recv: Vec<_> = Vec::new();
    for it in list {
        let (tx, rx) = mpsc::channel();
        recv.push(rx);
        let world_clone = world.clone();
        let it_clone = it.clone();
        let len = it_clone.len();
        let progress = multi_progress.add(ProgressBar::new(100));
        let mut cur_percent: u64 = 0;
        let handle = std::thread::spawn(move || {
            let mut color_list: Vec<Pair<Pair<u32, u32>, Color>> = Vec::new();
            for (nxt, pos) in it_clone.iter().enumerate() {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _s in 0..samples_per_pixel {
                    let u = ((pos.second as f64) + random_double()) / ((image_width - 1) as f64);
                    let v = ((pos.first as f64) + random_double()) / ((image_height - 1) as f64);
                    let r = cam.get_ray(u, v, 0.0, 1.0);
                    pixel_color = pixel_color + ray_color(&r, background, &world_clone, max_depth);
                }
                color_list.push(Pair::new(*pos, pixel_color));
                let nxt_percent = (100 * nxt / len) as u64;
                if nxt_percent != cur_percent {
                    cur_percent = nxt_percent;
                    progress.set_position(cur_percent);
                }
            }
            tx.send(color_list).unwrap();
            progress.finish();
        });
        threads.push(handle);
    }
    multi_progress.join_and_clear().unwrap();

    for it in recv.iter().take(tot) {
        let info = it.recv().unwrap();
        for k in info {
            let j = k.first.first;
            let i = k.first.second;
            let pixel_color = k.second;
            let pixel = img.get_pixel_mut(i, image_height - j - 1);
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
        }
    }

    /*let progress = if option_env!("CI").unwrap_or_default() == "true" {
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
    progress.finish();*/

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
