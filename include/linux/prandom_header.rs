/* SPDX-License-Identifier: GPL-2.0 */
/*
 * include/linux/prandom.h
 *
 * Include file for the fast pseudo-random 32-bit
 * generation.
 */

// Dependencies supplied by other translated units:
// linux/types.h, linux/once.h, linux/percpu.h, linux/random.h

#[repr(C)]
pub struct rnd_state {
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
}

extern "C" {
    pub fn prandom_u32_state(state: *mut rnd_state) -> u32;
    pub fn prandom_bytes_state(state: *mut rnd_state, buf: *mut core::ffi::c_void, nbytes: usize);
    pub fn prandom_seed_full_state(pcpu_state: *mut rnd_state);
}

// C macro equivalent: DO_ONCE(prandom_seed_full_state, (pcpu_state)).
// The DO_ONCE dependency is supplied by linux/once.h.
#[macro_export]
macro_rules! prandom_init_once {
    ($pcpu_state:expr) => {
        DO_ONCE!(prandom_seed_full_state, ($pcpu_state))
    };
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
    let i: u32 = ((seed >> 32) ^ (seed << 10) ^ seed) as u32;

    (*state).s1 = __seed(i, 2u32);
    (*state).s2 = __seed(i, 8u32);
    (*state).s3 = __seed(i, 16u32);
    (*state).s4 = __seed(i, 128u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
