// SPDX-License-Identifier: GPL-2.0
/*
 * This is a maximally equidistributed combined Tausworthe generator
 * based on code from GNU Scientific Library 1.5 (30 Jun 2004)
 *
 * lfsr113 version:
 *
 * x_n = (s1_n ^ s2_n ^ s3_n ^ s4_n)
 *
 * s1_{n+1} = (((s1_n & 4294967294) << 18) ^ (((s1_n <<  6) ^ s1_n) >> 13))
 * s2_{n+1} = (((s2_n & 4294967288) <<  2) ^ (((s2_n <<  2) ^ s2_n) >> 27))
 * s3_{n+1} = (((s3_n & 4294967280) <<  7) ^ (((s3_n << 13) ^ s3_n) >> 21))
 * s4_{n+1} = (((s4_n & 4294967168) << 13) ^ (((s4_n <<  3) ^ s4_n) >> 12))
 *
 * The period of this generator is about 2^113 (see erratum paper).
 *
 * From: P. L'Ecuyer, "Maximally Equidistributed Combined Tausworthe
 * Generators", Mathematics of Computation, 65, 213 (1996), 203--213:
 * http://www.iro.umontreal.ca/~lecuyer/myftp/papers/tausme.ps
 * ftp://ftp.iro.umontreal.ca/pub/simulation/lecuyer/papers/tausme.ps
 *
 * There is an erratum in the paper "Tables of Maximally Equidistributed
 * Combined LFSR Generators", Mathematics of Computation, 68, 225 (1999),
 * 261--269: http://www.iro.umontreal.ca/~lecuyer/myftp/papers/tausme2.ps
 *
 *      ... the k_j most significant bits of z_j must be non-zero,
 *      for each j. (Note: this restriction also applies to the
 *      computer code given in [4], but was mistakenly not mentioned
 *      in that paper.)
 *
 * This affects the seeding procedure by imposing the requirement
 * s1 > 1, s2 > 7, s3 > 15, s4 > 127.
 */

/// Seeded pseudo-random number generator state, supplied by the kernel.
#[repr(C)]
pub struct rnd_state {
    pub s1: u32,
    pub s2: u32,
    pub s3: u32,
    pub s4: u32,
}

#[inline]
pub unsafe fn prandom_u32_state(state: *mut rnd_state) -> u32 {
    (*state).s1 = (((*state).s1 & 4294967294u32) << 18)
        ^ ((((*state).s1 << 6) ^ (*state).s1) >> 13);
    (*state).s2 = (((*state).s2 & 4294967288u32) << 2)
        ^ ((((*state).s2 << 2) ^ (*state).s2) >> 27);
    (*state).s3 = (((*state).s3 & 4294967280u32) << 7)
        ^ ((((*state).s3 << 13) ^ (*state).s3) >> 21);
    (*state).s4 = (((*state).s4 & 4294967168u32) << 13)
        ^ ((((*state).s4 << 3) ^ (*state).s4) >> 12);

    (*state).s1 ^ (*state).s2 ^ (*state).s3 ^ (*state).s4
}

pub unsafe fn prandom_bytes_state(state: *mut rnd_state, buf: *mut core::ffi::c_void, mut bytes: usize) {
    let mut ptr = buf as *mut u8;

    while bytes >= core::mem::size_of::<u32>() {
        // External kernel equivalent of put_unaligned.
        let value = prandom_u32_state(state);
        core::ptr::copy_nonoverlapping(
            &value as *const u32 as *const u8,
            ptr,
            core::mem::size_of::<u32>(),
        );
        ptr = ptr.add(core::mem::size_of::<u32>());
        bytes -= core::mem::size_of::<u32>();
    }

    if bytes > 0 {
        let mut rem = prandom_u32_state(state);
        loop {
            *ptr = rem as u8;
            ptr = ptr.add(1);
            bytes -= 1;
            rem >>= u8::BITS;
            if bytes == 0 {
                break;
            }
        }
    }
}

#[cfg(feature = "kunit")]
pub unsafe fn prandom_warmup(state: *mut rnd_state);

pub unsafe fn prandom_warmup(state: *mut rnd_state) {
    /* Calling RNG ten times to satisfy recurrence condition */
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
    prandom_u32_state(state);
}

pub unsafe fn prandom_seed_full_state(pcpu_state: *mut rnd_state) {
    // `for_each_possible_cpu`, `per_cpu_ptr`, `get_random_bytes`, and `__seed`
    // are supplied by the surrounding kernel build and are intentionally not
    // implemented here.
    /* for_each_possible_cpu(i) {
        let state = per_cpu_ptr(pcpu_state, i);
        let mut seeds = [0u32; 4];

        get_random_bytes(&mut seeds as *mut _ as *mut core::ffi::c_void,
                         core::mem::size_of_val(&seeds));
        (*state).s1 = __seed(seeds[0], 2u32);
        (*state).s2 = __seed(seeds[1], 8u32);
        (*state).s3 = __seed(seeds[2], 16u32);
        (*state).s4 = __seed(seeds[3], 128u32);

        prandom_warmup(state);
    } */
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
