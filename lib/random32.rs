// SPDX-License-Identifier: GPL-2.0
//! Tausworthe pseudo-random generator state helpers from lib/random32.c.
//!
//! This is the maximally equidistributed combined Tausworthe generator of
//! L'Ecuyer (1999), period ~2^113, with the original C arithmetic. These
//! helpers are not cryptographically secure.

use kernel::bindings::get_random_bytes;
use kernel::ffi::{c_int, c_uint, c_void};

/// `struct rnd_state` from linux/prandom.h; `random32_helpers.c` asserts
/// the C layout matches.
#[repr(C)]
pub struct rnd_state {
    s1: u32,
    s2: u32,
    s3: u32,
    s4: u32,
}

extern "C" {
    fn lupos_random32_next_possible_cpu(cpu: c_int) -> c_int;
    fn lupos_random32_nr_cpu_ids() -> c_uint;
    fn lupos_random32_cpu_state(pcpu_state: *mut rnd_state, cpu: c_int) -> *mut rnd_state;
}

/// `TAUSWORTHE(s, a, b, c, d)`: `((s & c) << d) ^ (((s << a) ^ s) >> b)` on u32.
#[inline(always)]
const fn tausworthe(s: u32, a: u32, b: u32, c: u32, d: u32) -> u32 {
    ((s & c) << d) ^ (((s << a) ^ s) >> b)
}

/// `__seed(x, m)` from linux/prandom.h: keep each component above its minimum.
#[inline(always)]
const fn seed(x: u32, m: u32) -> u32 {
    if x < m {
        x.wrapping_add(m)
    } else {
        x
    }
}

/// Advances the state and returns the next 32-bit pseudo-random value.
///
/// # Safety
/// `state` points to a valid, exclusively accessed `struct rnd_state`.
#[no_mangle]
pub unsafe extern "C" fn prandom_u32_state(state: *mut rnd_state) -> u32 {
    // SAFETY: the caller provides a valid state pointer.
    let st = unsafe { &mut *state };
    st.s1 = tausworthe(st.s1, 6, 13, 4294967294, 18);
    st.s2 = tausworthe(st.s2, 2, 27, 4294967288, 2);
    st.s3 = tausworthe(st.s3, 13, 21, 4294967280, 7);
    st.s4 = tausworthe(st.s4, 3, 12, 4294967168, 13);
    st.s1 ^ st.s2 ^ st.s3 ^ st.s4
}

/// Fills `bytes` bytes at `buf` from the generator, as the C version does:
/// whole native-endian words first, then the low bytes of one more value.
///
/// # Safety
/// `state` is valid and `buf` is writable for `bytes` bytes.
#[no_mangle]
pub unsafe extern "C" fn prandom_bytes_state(state: *mut rnd_state, buf: *mut c_void, bytes: usize) {
    let mut ptr = buf.cast::<u8>();
    let mut bytes = bytes;
    // SAFETY: every write stays within the caller's buf..buf+bytes range.
    unsafe {
        while bytes >= core::mem::size_of::<u32>() {
            ptr.cast::<u32>().write_unaligned(prandom_u32_state(state));
            ptr = ptr.add(core::mem::size_of::<u32>());
            bytes -= core::mem::size_of::<u32>();
        }
        if bytes > 0 {
            let mut rem = prandom_u32_state(state);
            loop {
                ptr.write(rem as u8);
                ptr = ptr.add(1);
                bytes -= 1;
                rem >>= 8;
                if bytes == 0 {
                    break;
                }
            }
        }
    }
}

/// Runs the generator ten times to satisfy the recurrence condition.
///
/// # Safety
/// `state` points to a valid `struct rnd_state`.
#[cfg_attr(any(CONFIG_KUNIT, CONFIG_KUNIT = "m"), no_mangle)]
pub unsafe extern "C" fn prandom_warmup(state: *mut rnd_state) {
    for _ in 0..10 {
        // SAFETY: forwarded from the caller's contract.
        unsafe { prandom_u32_state(state) };
    }
}

/// Seeds every possible CPU's state from get_random_bytes() and warms it up.
///
/// # Safety
/// `pcpu_state` is a valid per-CPU `struct rnd_state` allocation.
#[no_mangle]
pub unsafe extern "C" fn prandom_seed_full_state(pcpu_state: *mut rnd_state) {
    // SAFETY: the native helpers implement for_each_possible_cpu() and
    // per_cpu_ptr(); each per-CPU state is written only by this seeding pass.
    unsafe {
        let mut cpu = lupos_random32_next_possible_cpu(-1);
        while (cpu as c_uint) < lupos_random32_nr_cpu_ids() {
            let state = lupos_random32_cpu_state(pcpu_state, cpu);
            let mut seeds = [0u32; 4];
            get_random_bytes(seeds.as_mut_ptr().cast(), core::mem::size_of_val(&seeds));
            (*state).s1 = seed(seeds[0], 2);
            (*state).s2 = seed(seeds[1], 8);
            (*state).s3 = seed(seeds[2], 16);
            (*state).s4 = seed(seeds[3], 128);
            prandom_warmup(state);
            cpu = lupos_random32_next_possible_cpu(cpu);
        }
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
