extern crate image;
extern crate num_complex;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let zoom = args[1].parse::<f32>().unwrap();
    let transx = args[2].parse::<f32>().unwrap();
    let transy = args[3].parse::<f32>().unwrap();

    // Image width and height
    let imgx = 800;
    let imgy = 800;

    let scalex = zoom / imgx as f32;
    let scaley = zoom / imgy as f32;

    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    
    // Black background
    for pixel in imgbuf.pixels_mut() {
        *pixel = image::Rgb([0,0,0]);
    }

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let cx = x as f32 * scalex  + transx;
        let cy = y as f32 * scaley + transy;

        let c = num_complex::Complex::new(cx, cy);
        let mut z = num_complex::Complex::new(0.0, 0.0);

        for i in 0..255 {
            if z.norm() > 2.0 {
                    *pixel = image::Rgb([i as u8, 0, 0]);
                    break;
            } else {
                z = z * z + c;
            }
        }
    }

    // Save the image
    imgbuf.save("fractal1.png").unwrap();

}
