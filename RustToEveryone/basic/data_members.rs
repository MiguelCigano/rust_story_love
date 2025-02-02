use std::time::Instant;

struct LazyCache {
    has_approx: bool,
    approx: f64,
}

impl LazyCache {
    pub fn new() -> Self {
        LazyCache {
            has_approx: false,
            approx: 0.0,
        }
    }

    pub fn lazy_operation(&mut self) -> f64 {
        if !self.has_approx {
            self.approx = self.pi_as_double();
            self.has_approx = true;
        }
        self.approx
    }

    fn pi_as_double(&self) -> f64 {
        let mut sum = 0.0;
        let it: i64 = 2_000_000_000;

        for i in 0..it {
            sum += if i % 2 == 0 {
                1.0 / (2 * i + 1) as f64
            } else {
                -1.0 / (2 * i + 1) as f64
            };
        }

        sum * 4.0
    }
}

fn main() {
    let start = Instant::now();

    let mut get_pi = LazyCache::new();

    println!("First calculation of pi: {}", get_pi.lazy_operation());
    let duration = start.elapsed();
    println!("Calculation time: {:?}", duration);

    let start = Instant::now();
    println!("Obtaining the value (cache): {}", get_pi.lazy_operation());
    let duration = start.elapsed();
    println!("Obtaining pi from cache: {:?}", duration);
}