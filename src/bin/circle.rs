use image::{ RgbImage, Rgb };

fn plot_circle_points(img: &mut image::ImageBuffer<Rgb<u8>, Vec<u8>>, color: Rgb<u8>, xc: i32, yc: i32, x: i32, y: i32) {
    img.put_pixel((xc + x) as u32, (yc + y) as u32, color);
    img.put_pixel((xc - x) as u32, (yc + y) as u32, color);
    img.put_pixel((xc + x) as u32, (yc - y) as u32, color);
    img.put_pixel((xc - x) as u32, (yc - y) as u32, color);
    img.put_pixel((xc + y) as u32, (yc + x) as u32, color);
    img.put_pixel((xc - y) as u32, (yc + x) as u32, color);
    img.put_pixel((xc + y) as u32, (yc - x) as u32, color);
    img.put_pixel((xc - y) as u32, (yc - x) as u32, color);
}

fn make_circle(img: &mut image::ImageBuffer<Rgb<u8>, Vec<u8>>, color: Rgb<u8>, xc: i32, yc: i32, r: i32) {
    let mut x = 0;
    let mut y = r;
    let mut p = 1 - r;

    plot_circle_points(img, color, xc, yc, x, y);

    while x < y {
        for i in 0..=y {
            plot_circle_points(img, color, xc, yc, x, i);
        }
        x += 1;
        if p < 0 {
            p += 2 * x + 1;
        } else {
            y -= 1;
            p += 2 * (x - y) + 1;
        }
    }
}

fn main() {
    let mut img = RgbImage::new(2048, 2048);
    /*
    for x in 0..=255 {
        for y in 0..=255 {
            img.put_pixel(x, y, Rgb([x as u8, y as u8, ((x + y) << 1) as u8]));
        }
    }
    */
    make_circle(&mut img, Rgb([255, 0, 0]), 1023, 1023, 512);
    img.save_with_format("test", image::ImageFormat::Png).unwrap();
}