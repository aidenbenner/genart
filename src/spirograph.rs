use super::point::Point;
use std::option::Option;
use rand::Rng;
use std::option;
use std::boxed;

pub struct Epicycle {
    pub r : f64,
    pub freq : f64,
    pub center : Point,
    pub inner : Option<Box<Epicycle>>,
}

impl Epicycle {
    pub fn val_at_t(&self, t : f64) -> Point {
        let x = self.r * (t * self.freq).cos();
        let y = self.r * (t * self.freq).sin();

        let inner_val = match &self.inner {
            None => Point::ORIGIN,
            Some(e) => e.val_at_t(t)
        };
        // println!("inner {} {} ", inner_val.x, inner_val.y);

        return inner_val.add(Point::new(self.center.x + x, self.center.y + y));
    }

    pub fn add_cycle(&mut self, epi : Epicycle) {
        self.inner = Some(Box::new(epi));
    }

    pub fn gen_points(&self, num_points : i32, time_span : f64) -> Vec<Point> {
        let mut points = vec![];

        for _i in 0..num_points {
            let t = (_i as f64 / num_points as f64) * time_span;
            points.push(self.val_at_t(t));
        }

        return points;
    }

    pub fn rand_epi<'b>(center : Point, freq : f64, r : f64, num_inner : u8) -> Epicycle {
        println!("--------{} ", num_inner);
        let mut rng = rand::thread_rng();
        // let freq = 200.;
        let freq = 10. * rng.gen_range(1, 100) as f64;
        let inner = match num_inner {
            0 => None,
            _ => Some (
                Box::new(
                    Epicycle::rand_epi(Point::ORIGIN, freq, r / 3., num_inner - 1)
                )
            ),
        };

        return Epicycle{center, r, freq, inner};
    }
}


