use bmp::{Image, Pixel};
use bmp::consts;

fn main() {
    let path = std::env::args().nth(1).expect("You must provide a path.");
    let operation = std::env::args().nth(2).expect("You must provide an operation.");

    if operation.as_str() == "pixel" {
        draw_pixel(path.as_str());
    } else if operation.as_str() == "diagonal" {
        draw_diagonal(path.as_str());
    } else if operation.as_str() == "x" {
        draw_x(path.as_str());
    } else if operation.as_str() == "abo" {
        draw_abo(path.as_str());
    } else {
        eprintln!("The operation {operation} was not recognised!");
    }
}

fn draw_pixel(path: &str) {
    let mut image = Image::new(100, 100);
    image.set_pixel(50, 50, Pixel::new(255, 255, 255));
    image.save(path).expect("This should save correctly.");
}

fn draw_diagonal(path: &str) {
    let mut image = Image::new(100, 100);
    for i in 1..100 {
        image.set_pixel(i, i, Pixel::new(255, 255, 255));
        
    }
    image.save(path).expect("This should save correctly.");
}

fn draw_x(path: &str) {
    let mut image = Image::new(100, 100);
    for i in 1..100 {
        image.set_pixel(i, i, Pixel::new(255, 255, 255));
        image.set_pixel(99-i, i, Pixel::new(255, 255, 255))
    }
    image.save(path).expect("This should save correctly.");
}

fn draw_abo(path: &str) {
    let mut image = Image::new(200, 100);
    for (x, y) in image.coordinates() {
        if y < 50 {
            image.set_pixel(x, y ,Pixel::new(0, 0, 0));
        } else {
            image.set_pixel(x, y ,Pixel::new(255, 0, 0));
        }
        
    }

    for (x, y) in image.coordinates() {
        let dx = (x as i32) - 100;
        let dy = (y  as i32) - 50;
        if dx * dx + dy * dy < 25 * 25 {
            image.set_pixel(x, y , consts::YELLOW);
        }
    }


    image.save(path).expect("This should save correctly.");
}
