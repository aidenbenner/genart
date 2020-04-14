extern crate image;
extern crate ini;
extern crate cairo;
extern crate rand;

use rand_distr::{Normal, Distribution, Uniform};
use cairo::{ ImageSurface, Format, Context, LineCap, Antialias};
use palette::{Hsv, Srgb};
use ini::Ini;

mod point;
mod spirograph;
use spirograph::Epicycle;

use point::Point;

use rand::{thread_rng, Rng};
use std::io::Write;
// use splines::{Interpolation, Key, Spline};

struct Circle {
    center : Point,
    r : f64,
}

impl Circle {
    fn contains(&self, point : &Point) -> bool {
        return self.center.dist(&point) < self.r;
    }

    fn intersects(a : &Circle, b : &Circle) -> bool {
        let dist = a.center.dist(&b.center);
        return dist < (a.r + b.r);
    }

    fn draw(&self, context : &Context ) {
        let mut rng = rand::thread_rng();

        let colors = vec![
            (0.14, 0.48, 0.63),
            (0.44, 0.76, 0.70),
            (0.7, 0.86, 0.75),
            (1.0, 0.09, 0.33),
        ];

        let mut rand_color = || {
            let mut rng = rand::thread_rng();
            let ind = rng.gen_range(0, colors.len());
            return colors[ind];
        };

        let (r,g,b) = rand_color ();
        context.set_source_rgb(r, g, b);
        context.arc(self.center.x, self.center.y, self.r, 0., 360.);
        context.fill();

        context.set_source_rgb(1.0, 1.0, 1.0);

        for _i in 0..6 {
            let mut rand_point_on_circ = || {
                let mut make_angle = || rng.gen_range(0., 360.);
                let angle = make_angle () as f64;
                let x = angle.cos() * self.r;
                let y = angle.sin() * self.r;
                return Point {x:x + self.center.x, y:y + self.center.y};
            };

            let a = rand_point_on_circ ();
            let b = rand_point_on_circ ();


            context.move_to(a.x, a.y);
            context.line_to(b.x, b.y);
            context.stroke();
        }
        context.set_source_rgb(0., 0., 0.);
    }

    /*
    fn rand_circle(xmax : f64, ymax : f64, rmax : f64) -> Circle {
    }*/
}


fn main() {
    let config_str = std::fs::read_to_string("info.cfg")
        .expect("config fail");

    let tokens :Vec<&str> = config_str.split('\n').collect();
    let title = tokens[0];
    let version : i32 = tokens[1].parse().unwrap();

    println!("Rendering {} {}:", title, version);


    let mut rng = rand::thread_rng();

    let width = 2560;
    let height = 1440;

    let normal = Normal::new((width / 2) as f64, 1000 as f64).unwrap();
    let uniform = Uniform::new(0, width);

    let surface = ImageSurface::create(Format::ARgb32, width, height)
        .expect("surface create fail");
    let context = Context::new(&surface);

    //context.set_source_rgb(247. / 255., 247. / 255., 1.0);
    context.set_source_rgb(0.07, 0.16, 0.24);
    context.set_source_rgb(0.17, 0.26, 0.34);
    context.paint();

    context.set_source_rgb(0.0, 0.0, 0.0);

    let rad = height as f64 / 2.2;
    let fwidth = width as f64;
    let fheight = height as f64;
    let x_start = fwidth / 2. - rad;
    let x_end = fwidth / 2. + rad;
    // context.arc(width as f64 / 2.0, height as f64 / 2., rad, 0., 360.);
    // context.clip();

    context.set_source_rgb(1.0, 1.0, 1.0);


    /*
    let epi3 = Epicycle {r:50., freq:200., center:Point::new(0., 0.), inner:None};
    let epi2 = Epicycle {r:100., freq:80., center:Point::new(0., 0.), inner:Some(&epi3)};
    let mut epi = Epicycle {r:600., freq:10., center:Point::new(fwidth / 2., fheight / 2.), inner:Some(&epi2)};
    */
    for z in 1..3 {
        let center = Point::new(fwidth / 2., fheight / 2.);
        let mut epi = Epicycle::rand_epi(center, 20., z as f64 * 300., 2);
        let points = epi.gen_points(2000, 200.);

        context.set_antialias(Antialias::Subpixel);
        context.set_line_width(0.1);
        context.set_line_cap(LineCap::Round);

        for i in 2..points.len() {
            let p1 = points.get(i).unwrap();
            let p2 = points.get(i - 1).unwrap();
            let p3 = points.get(i - 2).unwrap();

            // let rand_point = points.get(rng.gen_range(0, points.len())).unwrap();
            // println!("{} {} ", p.x, p.y);
            context.move_to(p1.x, p1.y);
            context.curve_to(p1.x, p1.y, p2.x, p2.y, p3.x, p3.y);
            context.stroke();
        }

    }


    /*
    let mut circs = Vec::new();
    // let total_itrs = 20000000;
    let total_itrs = 200000000;

    'outer: for _i in 0..total_itrs {
        if _i % 100 == 0 {
            println!("{} / {}", _i, total_itrs);
        }
        let x = rng.gen_range(x_start, x_end);
        let y = uniform.sample(&mut rand::thread_rng());

        let center = Point {
            x: x as f64,
            y: y as f64,
        };

        let r = rng.gen_range(1., 200.);
        let circ_a = Circle {
            center: center,
            r:r
        };
        for circ_b in &circs {
            if Circle::intersects(&circ_a, circ_b) {
                continue 'outer;
            }
        }
        circ_a.draw(&context);
        circs.push(circ_a);
        context.stroke();
    }*/


    /*
    let vlines = 10;
    for _i in 0..vlines {
        let x = uniform.sample(&mut rand::thread_rng());
        let y = uniform.sample(&mut rand::thread_rng());

        let center = Point {
            x: x as f64,
            y: y as f64,
        };

        let circ = Circle {
            center: center,
            r: 10.
        };

        circ.draw(&context);

        context.move_to(0., y as f64);
        context.line_to(height as f64, y as f64);

        context.move_to(x as f64, 0.);
        context.line_to(x as f64, height as f64);
        context.stroke();
    }
    */
    /*

    circ_sizes = vec!{50, 40, 30, 20, 10, 5, 2, 1};
    for c in circ_sizes {



    }*/


    /*
    let mut points = Vec::new();
    let mut keys = Vec::new();
    for _i in 0..100 {
        let x = normal.sample(&mut rand::thread_rng()) as f64;
        let y = normal.sample(&mut rand::thread_rng()) as f64;

        keys.push(Key::new(x, y, Interpolation::Bezier(90.)));
        points.push((x, y));
        // context.arc(x,y, 10., 0., 280.);
        // context.line_to(x, y);
        // context.stroke();
    }
    let spline = Spline::from_vec(keys);

    let num_points = 1000;
    let mut last_x = None;
    let mut last_y = None;
    for _i in 0..num_points {
        let x= (width as f64) * ((_i as f64) / (num_points as f64));
        let y = spline.sample(x as f64);
        if y.is_some() {
            if last_x.is_some() {
                let y = y.unwrap();
                println!("{}, {}", x, y);
                context.move_to(last_x.unwrap(), last_y.unwrap());
                context.line_to(x, y);
                context.stroke();
            }

            last_x = Some(x);
            last_y = Some(y.unwrap());
        }
    }*/
    use std::fs::File;

    let mut output = File::create("output.png").expect("");
    surface.write_to_png(&mut output)
    .expect("file write fail");

    let mut file = File::create(format!("prints/{}_{}.png", title, version))
        .expect("file create fail");
    surface.write_to_png(&mut file)
    .expect("file write fail");

    let mut conf = File::create("info.cfg").unwrap();
    let conf_str = format!("{}\n{}", title, version + 1);
    println!("{}", conf_str);
    conf.write_all(conf_str.as_bytes());
}
