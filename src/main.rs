extern crate byteorder;
extern crate num;

mod bitmap;
mod rgb;
mod mandelbrot;
mod zoom;
mod fractalcreator;

fn main() {
    
    let mut fractal_reator = fractalcreator::FractalCreator::new(800, 600);

	fractal_reator.add_range(0.0, rgb::RGB::new(0.0, 0.0, 0.0));
	fractal_reator.add_range(0.3, rgb::RGB::new(255.0, 0.0, 0.0));
	fractal_reator.add_range(0.5, rgb::RGB::new(255.0, 255.0, 0.0));
	fractal_reator.add_range(1.0, rgb::RGB::new(255.0, 255.0, 255.0));

	fractal_reator.add_zoom(zoom::Zoom::new(295, 202, 0.1));
	fractal_reator.add_zoom(zoom::Zoom::new(312, 304, 0.1));
	fractal_reator.run(String::from("test.bmp"));
}
