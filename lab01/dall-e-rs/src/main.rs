use bmp::consts;
use bmp::{Image, Pixel};

fn main() {
    let path = "pixel.bmp";
    let mut image = Image::new(200, 200);
    for (x, y) in image.coordinates() {
        if y < 100 {
            image.set_pixel(x, y, Pixel::new(0, 0, 0));
        } else {
            image.set_pixel(x, y, Pixel::new(255, 0, 0));
        }
    }

    let head_x = 100;
    let head_y = 50;
    let head_radius = 15;
    for (x, y) in image.coordinates() {
        let dx = (x as i32) - head_x;
        let dy = (y as i32) - head_y;
        if dx * dx + dy * dy < head_radius * head_radius {
            image.set_pixel(x, y, consts::YELLOW);
        }
    }

    for y in (head_y + head_radius)..(head_y + head_radius + 40) {
        image.set_pixel(head_x as u32, y as u32, consts::YELLOW);
    }

    for x in (head_x - 20)..(head_x + 20) {
        image.set_pixel(x as u32, (head_y + head_radius + 10) as u32, consts::YELLOW);
    }

    for i in 0..20 {
        image.set_pixel(
            (head_x - i) as u32,
            (head_y + head_radius + 40 + i) as u32,
            consts::YELLOW,
        );
        image.set_pixel(
            (head_x + i) as u32,
            (head_y + head_radius + 40 + i) as u32,
            consts::YELLOW,
        );
    }

    image.save(path).expect("This should save correctly.");
}
