use rand::{Rng, SeedableRng};
use pcg_rand::extension::{Pcg32Ext, Ext32};

use crate::common::ONE_MINUS_EPSILON;

pub struct RNG {
    rng: Pcg32Ext<Ext32>,
    pub seed: u64
}

impl RNG {
    // Try to never use this, only ever seed it
    pub fn new() -> Self {
        Self{
            rng: Pcg32Ext::from_entropy(),
            seed: 0u64
        }
    }

    pub fn new_seeded(seq_idx: u64) -> Self {
        Self {
            rng: Pcg32Ext::seed_from_u64(seq_idx),
            seed: seq_idx
        }
    }

    pub fn set_seed(&mut self, seq_idx: u64) {
        if self.seed == seq_idx {
            return;
        }
        self.rng = Pcg32Ext::seed_from_u64(seq_idx);
        self.seed = seq_idx;
    }

    pub fn uniform_u32(&mut self) -> u32 {
        self.rng.gen()
    }

    // has a higher chance of not b [0, 2^32 % b - 1] if b doesnt divide 2^32 fully
    pub fn uniform_u32_bounded(&mut self, b: u32) -> u32 {
        let generated: u32 = self.rng.gen();
        generated % b
    }

    pub fn uniform_f32(&mut self) -> f32 {
        let float_value = (self.uniform_u32() as f32) * (1.0 / (u32::MAX as f32 + 1.0));
        float_value.min(ONE_MINUS_EPSILON)
    }
}