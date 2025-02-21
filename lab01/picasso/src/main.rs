use bmp::{consts, Image};
use std::env;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        eprintln!("Usage: cargo run -- <image1.bmp> <image2.bmp> ...");
        return;
    }

    for file in &args {
        println!("===== {} =====", file);

        match bmp::open(file) {
            Ok(img) => print_image(&img),
            Err(err) => eprintln!("Error! {:?}", err),
        }
    }
}

fn print_image(img: &Image) {
    for y in 0..img.get_height() {
        for x in 0..img.get_width() {
            let pixel = img.get_pixel(x, y);

            let pixel_repr =
                if (pixel.r, pixel.g, pixel.b) == (consts::RED.r, consts::RED.g, consts::RED.b) {
                    "R"
                } else if (pixel.r, pixel.g, pixel.b)
                    == (consts::LIME.r, consts::LIME.g, consts::LIME.b)
                {
                    "G"
                } else if (pixel.r, pixel.g, pixel.b)
                    == (consts::BLUE.r, consts::BLUE.g, consts::BLUE.b)
                {
                    "B"
                } else if (pixel.r, pixel.g, pixel.b)
                    == (consts::WHITE.r, consts::WHITE.g, consts::WHITE.b)
                {
                    "W"
                } else {
                    "?"
                };

            print!("{} ", pixel_repr);
        }
        println!();
    }
}
