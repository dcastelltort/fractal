extern crate byteorder;
extern crate num;

extern crate clap;
use clap::{Arg, App};

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::error::Error;

mod bitmap;
mod rgb;
mod mandelbrot;
mod zoom;
mod fractalcreator;

fn app() -> Result<(),&'static str> {
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
                                .short("o")
                                .long("output_file")
                                .takes_value(true)
                                .required(true)
                            )
                            .get_matches();

    let width = match str::parse::<u32>(matches.value_of("width").unwrap_or("")) {
        Ok(n) => n,
        Err(_) => 0
    };

    let height = match str::parse::<u32>(matches.value_of("height").unwrap_or("")) {
        Ok(n) => n,
        Err(_) => 0
    };
    let output_file = matches.value_of("output_file").unwrap_or("");

    if width == 0 || height == 0 {
        return Err("invalid width/height");
    }
    if output_file.is_empty() {
        return Err("invalid output file");
    }


    let mut fractal = fractalcreator::fractal_from_file(String::from("examples/fractal.json")).unwrap();

    let fractal_creator = fractalcreator::FractalCreator::new();
    fractal_creator.generateFractal(&mut fractal, String::from(output_file));

    Ok(())
}

fn main() {
    std::process::exit( match app() {
        Ok(()) => 0,
        Err(err) => {
            println!("{:?}", err);
            1
        }
    });
}
