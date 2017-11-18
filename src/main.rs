extern crate byteorder;
extern crate num;

extern crate clap;
use clap::{Arg, App, SubCommand};
use std::process;

mod bitmap;
mod rgb;
mod mandelbrot;
mod zoom;
mod fractalcreator;

fn main() {
    let matches = App::new("Fractal Creator")
                          .version("0.1")
                          .author("Damien Castelltort")
                          .about("Generates some mandelbrot fractal")
						  .arg(Arg::with_name("width")
						  	.short("w")
							.long("width")
							.takes_value(true)
							.required(true)
						  )
						  .arg(Arg::with_name("height")
						  	.short("h")
							.long("height")
							.takes_value(true)
							.required(true)
						  )
						  .arg(Arg::with_name("output_file")
						  	.short("of")
							.long("output_file")
							.takes_value(true)
							.required(true)
						  )
						  .get_matches();

		let width = match str::parse::<u32>(matches.value_of("width").unwrap_or("")) {
            Ok(n) => n,
            Err(e) => {
                println!("failed parsing width, {:?}", e);
                0
            }
        };

        let height = match str::parse::<u32>(matches.value_of("height").unwrap_or("")) {
            Ok(n) => n,
            Err(e) => {
                println!("failed parsing height, {:?}", e);
                0
            }
        };
        let output_file = matches.value_of("output_file").unwrap_or("");

        if width == 0 || height == 0 {
            println!("invalid width/heigh");
            std::process::exit(1);
        }
        if output_file.is_empty() {
            println!("invalid output file");
            std::process::exit(1);
        }

		let mut fractal_reator = fractalcreator::FractalCreator::new(width as i32, height as i32);

		fractal_reator.add_range(0.0, rgb::RGB::new(0.0, 0.0, 0.0));
		fractal_reator.add_range(0.3, rgb::RGB::new(255.0, 0.0, 0.0));
		fractal_reator.add_range(0.5, rgb::RGB::new(255.0, 255.0, 0.0));
		fractal_reator.add_range(1.0, rgb::RGB::new(255.0, 255.0, 255.0));

		fractal_reator.add_zoom(zoom::Zoom::new(295, 202, 0.1));
		fractal_reator.add_zoom(zoom::Zoom::new(312, 304, 0.1));
		fractal_reator.run(String::from(output_file));
}
