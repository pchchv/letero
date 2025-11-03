use rand::{Rng, SeedableRng, rngs::SmallRng};

pub struct SmallRandom(SmallRng);

impl SmallRandom {
    pub fn new(seed: u64) -> Self {
        SmallRandom(SmallRng::seed_from_u64(seed))
    }
}

#[cfg_attr(test, mockall::automock)]
pub trait RandomGenerator: Sync + Send {
    fn get_salt(&mut self) -> String;
}

impl RandomGenerator for SmallRandom {
    fn get_salt(&mut self) -> String {
        let mut result = [0u8; 16];
        self.0.fill(&mut result);
        hex::encode(result)
    }
}