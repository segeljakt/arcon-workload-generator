use rand::distributions::uniform::*;
use rand::distributions::*;
use rand::prelude::*;
use rand::Rng;
use std::ops::Range;

type Time = i64;

struct Generator<T> {
    clock: Time,
    delay: Range<Time>,
    target_out_of_order_factor: f64,
    data: Range<T>,
    rng: ThreadRng,
    marker: std::marker::PhantomData<T>,
}

impl<T> Generator<T> {
    fn new(delay: Range<Time>, target_out_of_order_factor: f64, data: Range<T>) -> Self {
        Self {
            clock: 0,
            delay,
            target_out_of_order_factor,
            data,
            rng: rand::thread_rng(),
            marker: std::marker::PhantomData,
        }
    }
}

impl<T> Iterator for Generator<T>
where
    Standard: Distribution<T>,
    Range<T>: SampleRange<T>,
    T: SampleUniform + Clone,
{
    type Item = (Time, T);
    fn next(&mut self) -> Option<(Time, T)> {
        let distance = if self.rng.gen_range(0.0..100.0) < self.target_out_of_order_factor {
            self.rng.gen_range(self.delay.clone())
        } else {
            0
        };
        let data: T = self.rng.gen_range(self.data.clone());
        let time = self.clock - distance;
        self.clock += 10;
        Some((time, data))
    }
}

fn main() {
    let mut g = Generator::<u32>::new(0..30, 30.0, 0..100);
    loop {
        println!("{:?}", g.next());
        std::thread::sleep(std::time::Duration::new(0, 100_000_000));
    }
}
