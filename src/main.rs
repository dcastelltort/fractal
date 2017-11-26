extern crate byteorder;
extern crate num;

extern crate clap;
use clap::{Arg, App};

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

extern crate crossbeam;

extern crate hprof;

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
                                .help("override default width from the fractal file")
                            )
                            .arg(Arg::with_name("height")
                                .short("h")
                                .long("height")
                                .takes_value(true)
                                .help("override default height from the fractal file")
                            )
                             .arg(Arg::with_name("input_file")
                                .short("i")
                                .long("input_file")
                                .takes_value(true)
                                .required(true)
                                .help("specify input JSON formatted file describing the fractal")
                            )
                            .arg(Arg::with_name("output_file")
                                .short("o")
                                .long("output_file")
                                .takes_value(true)
                                .required(true)
                                .help("the file that will store the rendered fractal")
                            )
                            .arg(Arg::with_name("threads")
                                .short("t")
                                .long("threads")
                                .takes_value(true)
                                .help("specify the number of threads to be used during render")
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
    
    let threads = match str::parse::<u32>(matches.value_of("threads").unwrap_or("1")) {
        Ok(n) => n,
        Err(_) => 0
    };

    let output_file = matches.value_of("output_file").unwrap_or("");
    let input_file = matches.value_of("input_file").unwrap_or("");


    if matches.is_present("width") && width == 0 {
        return Err("invalid width");
    }

    if matches.is_present("height") && height == 0 {
        return Err("invalid height");
    }

    if matches.is_present("threads") && threads == 0 {
        return Err("invalid threads");
    }

    if output_file.is_empty() {
        return Err("invalid output file");
    }
    if input_file.is_empty() {
        return Err("invalid input file");
    }

    println!("num threads: {:?}", threads);
    let mut fractal = fractalcreator::fractal_from_file(String::from(input_file)).unwrap();

    let fractal_creator = fractalcreator::FractalCreator::new();
    
    match fractal_creator.generate_fractal(&mut fractal, String::from(output_file), threads) {
        Ok(_) => Ok(()),
        Err(_) => Err("error generating fractal")
    }
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
