//
// Linear congruential generator (LCG) aka Snake Food.
//

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Random {
    // We used a u128 seed here as that's compatible with epoch as nanoseconds.
    seed: u32,
}

impl Random {
    fn new(seed: u32) -> Self {
        Random { seed }
    }

    //
    // Gets a new instance of `Random` with a seed of nanoseconds since `UNIX_EPOCH`.
    //
    pub fn time_seed() -> Self {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => Random::new(n.subsec_nanos()/10^9),
            Err(_) => panic!("Your clock is broken, or something."),
        }
    }

    //
    // The LCG algorithm
    // https://en.wikipedia.org/wiki/Linear_congruential_generator
    //
    fn next(&mut self) -> u32 {
        let a: u32 = 1664525;
        let c = 1013904223;
        let m = u32::MAX;

        self.seed = a.wrapping_mul(self.seed).wrapping_add(c) % m;
        self.seed
    }

    //
    // Some modulo wizardry to effectively get an output from the LCG within a min/max range
    //
    pub fn get(&mut self, min: u32, max: u32) -> u32 {
        min + (self.next() % (max - min + 1))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn next_should_produce_next_number() {
        let mut r = Random::new(42);
        let r1 = r.next();
        let r2 = r.next();
        let r3 = r.next();

        assert_eq!(r1, 1083814273);
        assert_eq!(r2, 378494188);
        assert_eq!(r3, 2479403867);
    }

    //
    // Look, my RNG is so good, I can unit test it.
    //
    #[test]
    fn get_should_return_number_in_range() {
        let mut r = Random::new(764543543);
        assert_eq!(r.get(1, 100), 99);
        assert_eq!(r.get(1, 100), 90);
        assert_eq!(r.get(1, 100), 45);
        assert_eq!(r.get(1, 100), 64);
        assert_eq!(r.get(1, 100), 27);
        assert_eq!(r.get(1, 100), 90);
        assert_eq!(r.get(1, 100), 1);
        assert_eq!(r.get(1, 100), 20);
        assert_eq!(r.get(1, 100), 47);
    }

    //
    // Make sure when using a time seed, the result from get() is still
    // within the expected range.
    //
    #[test]
    fn get_with_time_seed_should_return_number_in_range() {
        let mut r = Random::time_seed();

        for _ in 1..10000 {
            let n = r.get(1, 100);

            assert!(n >= 1 && n <= 100);
        }
    }
}
