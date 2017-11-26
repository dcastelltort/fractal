# Fractal

## Introduction

This is a simple rust program that generates fractals (mandelbrot).
It supports only RAW bmp file format as output
Note: don't use this as a reference, it is just testing/learning things in with Rust.

## Usage

USAGE:
    fractal [OPTIONS] --input_file <input_file> --output_file <output_file>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --height <height>              override default height from the fractal file
    -i, --input_file <input_file>      specify input JSON formatted file describing the fractal
    -o, --output_file <output_file>    the file that will store the rendered fractal
    -t, --threads <threads>            specify the number of threads to be used during render
    -w, --width <width>                override default width from the fractal file

### examples
fractal -w 800 -h 600 -i examples/fractal.json -o test.bmp -t 4