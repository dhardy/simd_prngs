use std::simd::*;

use rng_impl::*;

macro_rules! make_xorshift128plus {
    ($rng_name:ident, $vector:ident) => {
        pub struct $rng_name {
            s0: $vector,
            s1: $vector,
        }

        impl $rng_name {
            #[inline(always)]
            pub fn generate(&mut self) -> $vector {
                let mut s1 = self.s0;
                let s0 = self.s1;
                let result = s0 + s1;
                self.s0 = s0;
                s1 ^= s1 << 23; // a
                self.s1 = s1 ^ s0 ^ (s1 >> 18) ^ (s0 >> 5); // b, c
                result
            }
        }

        impl SeedableRng for $rng_name {
            type Seed = [u8; 0];

            #[inline(always)]
            fn from_seed(_seed: Self::Seed) -> Self {
                unimplemented!()
            }

            fn from_rng<R: RngCore>(mut rng: R) -> Result<Self, Error> {
                const ZERO: $vector = $vector::splat(0);

                let mut seeds = [$vector::default(); 2];
                while seeds
                    .iter()
                    // `splat(true)`
                    .fold(ZERO.eq(ZERO), |acc, s| acc & s.eq(&ZERO))
                    .any()
                {
                    rng.try_fill(seeds.as_byte_slice_mut())?;
                }

                Ok(Self {
                    s0: seeds[0],
                    s1: seeds[1],
                })
            }
        }
    };
}

make_xorshift128plus! { Xorshift128PlusX2, u64x2 }
make_xorshift128plus! { Xorshift128PlusX4, u64x4 }
make_xorshift128plus! { Xorshift128PlusX8, u64x8 }
