//
// Linear congruential generator (LCG) aka Snake Food.
//

use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Debug)]
pub struct Random {
    // We used a u128 seed here as that's compatible with epoch as nanoseconds.
    seed: u128
}

impl Random  {
    fn new(seed: u128) -> Self {
        Random { seed }
    }

    fn time_seed() -> Self {
        match SystemTime::now().duration_since(UNIX_EPOCH) {
            Ok(n) => Random::new(n.as_nanos()),
            Err(_) => panic!("Your clock is broken, or something."),
        }
    }

    fn next(&mut self) -> u128 {
        let a: u128 = 1664525;
        let c = 1013904223;
        let m = u128::MAX;

        self.seed = (a.wrapping_mul(self.seed).wrapping_add(c)) % m;
        self.seed
    }

    fn get(&mut self, min: u128, max: u128) -> u128 {
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
        assert_eq!(r2, 1804036966669548);
        assert_eq!(r3, 3002864631946643288923);
    }
    
    //
    // Look, my RNG is so good, I can unit test it.
    //
    #[test]
    fn get_should_return_number_in_range() {
        let mut r = Random::new(76454354354);
        assert_eq!(r.get(1, 100), 74);
        assert_eq!(r.get(1, 100), 49);
        assert_eq!(r.get(1, 100), 24);
        assert_eq!(r.get(1, 100), 99);
        assert_eq!(r.get(1, 100), 54);
        assert_eq!(r.get(1, 100), 9);
        assert_eq!(r.get(1, 100), 40);
        assert_eq!(r.get(1, 100), 99);
        assert_eq!(r.get(1, 100), 82);
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