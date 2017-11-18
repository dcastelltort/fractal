extern crate byteorder;
extern crate num;

extern crate clap;
use clap::{Arg, App, SubCommand};

mod bitmap;
mod rgb;
mod mandelbrot;
mod zoom;
mod fractalcreator;

fn main() {
    /*let matches = App::new("Fractal Creator")
                          .version("0.1")
                          .author("Damien Castelltort")
                          .about("Generates some mandelbrot fractal")
						  .arg(Arg::with_name("width")
						  	.short("w")
							.long("width")
							.takes_value(true)
							//.required(true)
							.default_value("800")
						  )
						  .arg(Arg::with_name("height")
						  	.short("h")
							.long("height")
							.takes_value(true)
							//.required(true)
							.default_value("600")
						  )
						  .arg(Arg::with_name("output_file")
						  	.short("of")
							.long("output_file")
							.takes_value(true)
							//.required(true)
							.default_value("test.bmp")
						  )
						  .get_matches();

		println!("{:?}", matches.value_of("width").unwrap());
		println!("{:?}", matches.value_of("height").unwrap());
		println!("{:?}", matches.value_of("output_file").unwrap());
*/
    let bFlag = true;
	if bFlag {

		let mut fractal_reator = fractalcreator::FractalCreator::new(800, 600);

		fractal_reator.add_range(0.0, rgb::RGB::new(0.0, 0.0, 0.0));
		fractal_reator.add_range(0.3, rgb::RGB::new(255.0, 0.0, 0.0));
		fractal_reator.add_range(0.5, rgb::RGB::new(255.0, 255.0, 0.0));
		fractal_reator.add_range(1.0, rgb::RGB::new(255.0, 255.0, 255.0));

		fractal_reator.add_zoom(zoom::Zoom::new(295, 202, 0.1));
		fractal_reator.add_zoom(zoom::Zoom::new(312, 304, 0.1));
		fractal_reator.run(String::from("test.bmp"));
	}
}
