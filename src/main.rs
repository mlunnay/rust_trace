extern crate png;

pub mod raytrace;
pub mod scenes;

use raytrace::camera::Camera;
use raytrace::vec::Vec3;
use std::time::{Duration, Instant};
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
use raytrace::renderer::Renderer;
use raytrace::bvh::BVHNode;
use std::rc::Rc;
use raytrace::util::degrees_to_radians;
use std::collections::HashMap;
use raytrace::hittable::Hittable;
use raytrace::ray::Ray;

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

    let gradient_background = |ray: Ray| {
        let unit_direction = ray.direction.normalize();
        let t = 0.5 * (unit_direction.y + 1.0);
        (1.0-t) * Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0)
    };

    let mut scenes = HashMap::new();

    scenes.insert("rtiw_final", Scene{
        camera: None,
        objects: BVHNode::construct(scenes::rtiw_final::generate()),
        background: Some(Rc::new(gradient_background))
    });
    scenes.insert("image_texture", Scene{
        camera: None,
        objects: BVHNode::construct(scenes::image_texture::generate()),
        background: Some(Rc::new(gradient_background))
    });
    scenes.insert("box", Scene{
        camera: None,
        objects: BVHNode::construct(scenes::box_scene::generate()),
        background: Some(Rc::new(gradient_background))
    });
    scenes.insert("marble", Scene{
        camera: None,
        objects: BVHNode::construct(scenes::marble::generate()),
        background: Some(Rc::new(gradient_background))
    });
    scenes.insert("emitting", Scene{
        camera: Some(scenes::emitting_scene::camera(width, height)),
        objects: BVHNode::construct(scenes::emitting_scene::generate()),
        background: None
    });
    scenes.insert("cornell_box", Scene{
        camera: Some(scenes::cornell_box::camera(width as f64 /  height as f64)),
        objects: BVHNode::construct(scenes::cornell_box::generate()),
        background: None
    });
    scenes.insert("cornell_smoke", Scene{
        camera: Some(scenes::cornell_smoke::camera(width as f64 /  height as f64)),
        objects: BVHNode::construct(scenes::cornell_smoke::generate()),
        background: None
    });

    const SCENE_NAME: &str = "rtiw_final";

    let scene = scenes.get(SCENE_NAME).expect("Invalid scene name");
    let camera = scene.camera.unwrap_or_else(||{
        default_camera
    });
    let objects = scene.objects.clone();
    let background = match &scene.background {
        Some(bg) => Some(bg.clone()),
        None => None
    };
    
    let renderer = Renderer::new(width, height, 10, camera, objects, background);

    let mut data: Vec<u8> = vec![0; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let mut color = renderer.color_at(x as f64, y as f64);
            color.x = f64::sqrt(color.x);
            color.y = f64::sqrt(color.y);
            color.z = f64::sqrt(color.z);
            let r = (255.99 * color.x) as u8;
            let g = (255.99 * color.y) as u8;
            let b = (255.99 * color.z) as u8;
            let i = ((x + (height - y - 1) * width) * 4) as usize;
            data[i] = r;
            data[i + 1] = g;
            data[i + 2] = b;
            data[i + 3] = 255;
        }
    }

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

struct Scene {
    camera: Option<Camera>,
    objects: Rc<dyn Hittable>,
    background: Option<raytrace::renderer::BgFunc>
}
