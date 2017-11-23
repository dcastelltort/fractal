extern crate byteorder;
extern crate num;

extern crate clap;
use clap::{Arg, App};

extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::thread;
use std::sync::mpsc;
use std::time::Duration;

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
                            )
                            .arg(Arg::with_name("height")
                                .short("h")
                                .long("height")
                                .takes_value(true)
                            )
                             .arg(Arg::with_name("input_file")
                                .short("i")
                                .long("input_file")
                                .takes_value(true)
                                .required(true)
                            )
                            .arg(Arg::with_name("output_file")
                                .short("o")
                                .long("output_file")
                                .takes_value(true)
                                .required(true)
                            )
                            .arg(Arg::with_name("threads")
                                .short("t")
                                .long("threads")
                                .takes_value(true)
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

    if threads > 0 {
         
        let (tx, rx) = mpsc::channel();

        let tx1 = mpsc::Sender::clone(&tx);
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];

            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];

            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_secs(1));
            }
        });

         for received in rx {
            println!("Got: {}", received);
        }
    }

    let mut fractal = fractalcreator::fractal_from_file(String::from(input_file)).unwrap();

    let fractal_creator = fractalcreator::FractalCreator::new();
    
    match fractal_creator.generate_fractal(&mut fractal, String::from(output_file)) {
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
