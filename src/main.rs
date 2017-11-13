extern crate byteorder;
extern crate num;

mod bitmap;
mod rgb;
mod mandelbrot;
mod zoom;
mod fractalcreator;

fn main() {
    
    let mut fractalCreator = fractalcreator::FractalCreator::new(800, 600);

	fractalCreator.add_range(0.0, rgb::RGB::new(0.0, 0.0, 0.0));
	fractalCreator.add_range(0.3, rgb::RGB::new(255.0, 0.0, 0.0));
	fractalCreator.add_range(0.5, rgb::RGB::new(255.0, 255.0, 0.0));
	fractalCreator.add_range(1.0, rgb::RGB::new(255.0, 255.0, 255.0));

	println!("{}",fractalCreator.get_range(999));

	fractalCreator.add_zoom(zoom::Zoom::new(295, 202, 0.1));
	fractalCreator.add_zoom(zoom::Zoom::new(312, 304, 0.1));
	fractalCreator.run(String::from("test.bmp"));
}
