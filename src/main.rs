extern crate png;
extern crate rayon;
use rayon::prelude::*;

pub mod raytrace;
pub mod scenes;

use raytrace::camera::Camera;
use raytrace::vec::Vec3;
use std::time::{Duration, Instant};
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use raytrace::renderer::{Renderer, Background, GradientBackground};
use raytrace::bvh::BVHNode;
use std::sync::Arc;
use raytrace::util::degrees_to_radians;
use raytrace::hittable::Hittable;

fn main() {
    let start = Instant::now();
    let width = 800;
    let height = 800;
    let default_camera = Camera::new(
        Vec3::new(13.0, 2.0, 3.0),
        Vec3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        degrees_to_radians(20.0),
        width as f64 / height as f64,
        0.0,
        10.0
    );

    const SCENE_NAME: &str = "rtiw_final";

    let (camera, objects, background): (Camera, Arc<dyn Hittable>, Option<Box<dyn Background>>) = match SCENE_NAME {
        "rtiw_final" => (default_camera, BVHNode::construct(scenes::rtiw_final::generate()), Some(Box::new(GradientBackground{}))),
        "image_texture" => (default_camera, BVHNode::construct(scenes::image_texture::generate()), Some(Box::new(GradientBackground{}))),
        "box" => (default_camera, BVHNode::construct(scenes::box_scene::generate()), Some(Box::new(GradientBackground{}))),
        "marble" => (default_camera, BVHNode::construct(scenes::marble::generate()), Some(Box::new(GradientBackground{}))),
        "emitting_scene" => (scenes::emitting_scene::camera(width, height), BVHNode::construct(scenes::emitting_scene::generate()), None),
        "cornell_box" => (scenes::cornell_box::camera(width as f64 /  height as f64), BVHNode::construct(scenes::cornell_box::generate()), None),
        "cornell_smoke" => (scenes::cornell_smoke::camera(width as f64 /  height as f64), BVHNode::construct(scenes::cornell_smoke::generate()), None),
        _ => panic!("Invalid scene name")
    };  

    let renderer = Renderer::new(width, height, 10, camera, objects, background);

    let mut data = vec![0u8; (width * height * 4) as usize];

    // for y in 0..height {
    //     for x in 0..width {
    //         let mut color = renderer.color_at(x as f64, y as f64);
    //         color.x = f64::sqrt(color.x);
    //         color.y = f64::sqrt(color.y);
    //         color.z = f64::sqrt(color.z);
    //         let r = (255.99 * color.x) as u8;
    //         let g = (255.99 * color.y) as u8;
    //         let b = (255.99 * color.z) as u8;
    //         let i = ((x + (height - y - 1) * width) * 4) as usize;
    //         data[i] = r;
    //         data[i + 1] = g;
    //         data[i + 2] = b;
    //         data[i + 3] = 255;
    //     }
    // }

    data
        .par_chunks_mut((4 * width) as usize)
        .rev()
        .enumerate()
        .for_each(|(j, row)| {
            for (i, rgba) in row.chunks_mut(4).enumerate() {
                let color = renderer.color_at(i as f64, j as f64);
                let mut iter = rgba.iter_mut();
                *iter.next().unwrap() = (255.99 * color.x.sqrt()) as u8;
                *iter.next().unwrap() = (255.99 * color.y.sqrt()) as u8;
                *iter.next().unwrap() = (255.99 * color.z.sqrt()) as u8;
                *iter.next().unwrap() = 255;
            }
        });

    // write image to png
    let path = Path::new("./image.png");
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height); // Width is 2 pixels and height is 1.
    encoder.set_color(png::ColorType::RGBA);
    encoder.set_depth(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&data).unwrap(); // Save

    let elapsed = start.elapsed();
    eprintln!("rendered {} x {} with {} samples in {}", width, height, renderer.samples, human_readable_time(elapsed));
}

const SECONDS_IN_HOUR: f64 = 3600.0;
const SECONDS_IN_MINUTE: f64 = 60.0;
fn human_readable_time(duration: Duration) -> String {
    let mut seconds = duration.as_secs() as f64;
    let hours = f64::floor(seconds / SECONDS_IN_HOUR) as u32;
    seconds -= hours as f64 * SECONDS_IN_HOUR;
    let minutes = f64::floor(seconds / SECONDS_IN_MINUTE) as u32;
    seconds -= minutes as f64 * SECONDS_IN_MINUTE;

    let mut parts: Vec<String> = Vec::new();
    if hours > 0 {
        parts.push(format!("{} hour{}", hours, if hours > 1{"s"} else {""}))
    }

    if minutes > 0 {
        parts.push(format!("{} minute{}", minutes, if minutes > 1{"s"} else {""}))
    }
    
    if seconds > 0.0 {
        parts.push(format!("{} second{}", seconds, if seconds > 1.0{"s"} else {""}))
    }

    parts.join(" ")
}