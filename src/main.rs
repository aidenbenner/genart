extern crate image;
extern crate cairo;
extern crate rand;

use cairo::{ ImageSurface, Format, Context };

fn main() {
    let width = 1200;
    let height = 1200;
    let surface = ImageSurface::create(Format::ARgb32, width, height)
        .expect("surface create fail");
    let context = Context::new(&surface);
    context.set_source_rgb(1.0, 1.0, 1.0);
    context.paint();
    context.set_source_rgb(0.0, 0.0, 0.0);
    for _i in 0..100 {
        let x = rand::random::<f64>() * 600.0;
        let y = rand::random::<f64>() * 600.0;
        context.line_to(x, y);
    }
    context.stroke();

    use std::fs::File;
    let mut file = File::create("output.png")
        .expect("file create fail");

    surface.write_to_png(&mut file)
    .expect("file write fail");

    println!("Hello, world!");
}
