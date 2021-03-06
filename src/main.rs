mod vec;
mod ray;

use std::io::{stderr, Write};
use vec::{Vec3, Point3, Color};
use ray::Ray;

const IMAGE_SIZE: u64 = 800;
const ASPECT_RATIO: f64 = 4.0 / 3.0;
const IMAGE_WIDTH: u64 = IMAGE_SIZE;
const IMAGE_HEIGHT: u64 = ((IMAGE_SIZE as f64) / ASPECT_RATIO) as u64;

fn hit_sphere(center: Point3, radius: f64, r: &Ray) -> f64 {
    let oc = r.origin() - center; // distance between ray origin and circle origin 
    let a = r.direction().dot(r.direction()); 

    let b = 2.0 * oc.dot(r.direction());
    let c = oc.dot(oc) - radius * radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        -1.0
    } else {
        (-b - discriminant.sqrt()) / (2.0 * a)
    }
}

fn ray_color(r: &Ray) -> Color {
    // if hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r) {
    //     return Color::new(1.0, 0.0, 0.0);
    // }
    let t = hit_sphere(Point3::new(0.0, 0.0, -1.0), 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - Vec3::new(0.0, 0.0, -1.0)).normalized();
        // return 0.5 * Color::new(n.x() + 1.0, n.y() + 1.0, n.z() + 1.0);
        return 0.5 * Color::new(1.0 + n.z(), n.y() + 1.0, 0.0);
    }
    let unit_direction = r.direction().normalized();
    let t = 0.5 * (unit_direction.y() + 1.0); //recenters y
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    // Image
    // const ASPECT_RATIO: f64 = 16.0 / 9.0;

    // Camera
    let viewport_height = 2.0;
    let viewport_width = ASPECT_RATIO * viewport_height;
    let focal_length = 1.0;

    let origin = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    println!("P3");
    println!("{} {}", IMAGE_WIDTH, IMAGE_HEIGHT);
    println!("255");

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {:3}", IMAGE_HEIGHT - j - 1);
        stderr().flush().unwrap();

        for i in 0..IMAGE_WIDTH {
            // let r = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            // let g = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);
            // let b = 0.25;
            // let ir = (255.999 * r) as u64;
            // let ig = (255.999 * g) as u64;
            // let ib = (255.999 * b) as u64;
            // println!("{} {} {}", ir, ig, ib);

            // let pixel_color = Color::new(
            //     (i as f64) / ((IMAGE_WIDTH - 1) as f64), 
            //     (j as f64) / ((IMAGE_HEIGHT - 1) as f64), 
            //     0.25
            // );

            let u = (i as f64) / ((IMAGE_WIDTH - 1) as f64);
            let v = (j as f64) / ((IMAGE_HEIGHT - 1) as f64);

            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);

            println!("{}", pixel_color.format_color());
        }
    }
    eprintln!("\nDone.");
}

