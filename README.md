# Fractal

## Introduction

This is a simple rust program that generates fractals (mandelbrot).
It supports only RAW bmp file format as output

## Usage

USAGE:
    fractal --height <height> --output_file <output_file> --width <width>

FLAGS:
        --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -h, --height <height>
    -o, --output_file <output_file>
    -w, --width <width>

### example

fractal --width 800 --height 600 --output_file test.bmp 
