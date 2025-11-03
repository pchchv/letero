use rand::{SeedableRng, rngs::SmallRng};

pub struct SmallRandom(SmallRng);

impl SmallRandom {
    pub fn new(seed: u64) -> Self {
        SmallRandom(SmallRng::seed_from_u64(seed))
    }
}