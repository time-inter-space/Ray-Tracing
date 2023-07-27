/*use crate::*;

use console::style;
use image::{ImageBuffer, RgbImage};
use std::fs::File;

pub fn normal_mapping_gen() {
    let path = std::path::Path::new("input/NormalMappingSource.jpg");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

    let aspect_ratio = 1.0;
    let image_width = 500;
    let image_height = ((image_width as f64) / aspect_ratio) as u32;
    let quality = 100;
    let mut img: RgbImage = ImageBuffer::new(image_width, image_height);

    let p = Vec3::new(
        (image_width / 2) as f64,
        (image_height / 2) as f64,
        image_height as f64 * 2.0,
    );

    let mut data: Vec<Vec<Color>> = Vec::new();
    for j in 0..image_height {
        data.push(Vec::new());
        for _i in 0..image_width {
            data[j as usize].push(Color::new(0.5, 0.5, 0.5));
        }
    }

    for j in 0..image_height {
        for i in 0..image_width {
            let mut pixel_color = Color::new(0.5, 0.5, 1.0);
            let cur = Vec3::new(i as f64, (image_height - 1 - j) as f64, 0.0);
            if (p - cur).length_squared() <= (image_height * image_height) as f64 * 4.25 {
                pixel_color = Color::new(0.5, 0.5, 0.5) + unit_vector(p - cur) * 0.5;
            }
            data[j as usize][i as usize] = pixel_color;
        }
    }
    let v: [[i32; 3]; 3] = [[1, 2, 1], [2, 4, 2], [1, 2, 1]];
    for j in 0..image_height {
        for i in 0..image_width {
            let pixel = img.get_pixel_mut(i, j);
            let mut pixel_color = Vec3::new(0.0, 0.0, 0.0);
            let mut tot = 0;
            for dj in 0..2 {
                if j + dj < 1 || image_height < j + dj {
                    continue;
                }
                for di in 0..2 {
                    if i + di < 1 || image_width <= i + di {
                        continue;
                    }
                    pixel_color = pixel_color
                        + data[(j + dj - 1) as usize][(i + di - 1) as usize]
                            * v[dj as usize][di as usize] as f64;
                    tot += v[dj as usize][di as usize];
                }
            }
            pixel_color = pixel_color / tot as f64;
            *pixel = image::Rgb([
                (256.0 * clamp(pixel_color.e0, 0.0, 0.999)) as u8,
                (256.0 * clamp(pixel_color.e1, 0.0, 0.999)) as u8,
                (256.0 * clamp(pixel_color.e2, 0.0, 0.999)) as u8,
            ]);
        }
    }

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
}*/
