/* SPDX-License-Identifier: GPL-2.0 */

/* Original C header included <linux/types.h> for fixed-width integer aliases. */

#[repr(C)]
pub struct rnd_state {
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
}

/*
 * Handle minimum values for seeds
 */
#[inline]
pub fn __seed(x: u32, m: u32) -> u32 {
    if x < m {
        x.wrapping_add(m)
    } else {
        x
    }
}

/**
 * prandom_seed_state - set seed for prandom_u32_state().
 * @state: pointer to state structure to receive the seed.
 * @seed: arbitrary 64-bit value to use as a seed.
 */
#[inline]
pub unsafe fn prandom_seed_state(state: *mut rnd_state, seed: u64) {
    let i: u32 = (((seed >> 32) ^ (seed << 10) ^ seed) & 0xffffffffu64) as u32;

    unsafe {
        (*state).s1 = __seed(i, 2u32);
        (*state).s2 = __seed(i, 8u32);
        (*state).s3 = __seed(i, 16u32);
        (*state).s4 = __seed(i, 128u32);
    }
}

/**
 *	prandom_u32_state - seeded pseudo-random number generator.
 *	@state: pointer to state structure holding seeded state.
 *
 *	This is used for pseudo-randomness with no outside seeding.
 *	For more random results, use get_random_u32().
 */
#[inline]
pub unsafe fn prandom_u32_state(state: *mut rnd_state) -> u32 {
    #[inline]
    fn TAUSWORTHE(s: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
        ((s & c) << d) ^ (((s << a) ^ s) >> b)
    }

    unsafe {
        (*state).s1 = TAUSWORTHE((*state).s1, 6u32, 13u32, 4294967294u32, 18u32);
        (*state).s2 = TAUSWORTHE((*state).s2, 2u32, 27u32, 4294967288u32, 2u32);
        (*state).s3 = TAUSWORTHE((*state).s3, 13u32, 21u32, 4294967280u32, 7u32);
        (*state).s4 = TAUSWORTHE((*state).s4, 3u32, 12u32, 4294967168u32, 13u32);

        (*state).s1 ^ (*state).s2 ^ (*state).s3 ^ (*state).s4
    }
}
