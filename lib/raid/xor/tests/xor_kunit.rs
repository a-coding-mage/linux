// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * Unit test the XOR library functions.
 *
 * Copyright 2024 Google LLC
 * Copyright 2026 Christoph Hellwig
 *
 * Based on the CRC tests by Eric Biggers <ebiggers@google.com>.
 */

// Dependencies supplied by the kernel and other translation units:
// kunit/test.h, linux/prandom.h, linux/string_choices.h, linux/vmalloc.h,
// and linux/raid/xor.h.

const XOR_KUNIT_SEED: u32 = 42;
const XOR_KUNIT_MAX_BYTES: usize = 16384;
const XOR_KUNIT_MAX_BUFFERS: usize = 64;
const XOR_KUNIT_NUM_TEST_ITERS: usize = 1000;

static mut rng: rnd_state = rnd_state {};
static mut test_buffers: [*mut core::ffi::c_void; XOR_KUNIT_MAX_BUFFERS] =
    [core::ptr::null_mut(); XOR_KUNIT_MAX_BUFFERS];
static mut test_dest: *mut core::ffi::c_void = core::ptr::null_mut();
static mut test_ref: *mut core::ffi::c_void = core::ptr::null_mut();
static mut test_buflen: usize = 0;

#[repr(C)]
struct rnd_state;
struct kunit;
struct kunit_suite;

unsafe extern "C" {
    fn prandom_u32_state(state: *mut rnd_state) -> u32;
    fn prandom_bytes_state(state: *mut rnd_state, buf: *mut core::ffi::c_void, len: usize);
    fn prandom_seed_state(state: *mut rnd_state, seed: u32);
    fn vmalloc(size: usize) -> *mut core::ffi::c_void;
    fn vfree(ptr: *mut core::ffi::c_void);
    fn xor_gen(dest: *mut core::ffi::c_void, srcs: *mut *mut core::ffi::c_void,
        src_cnt: u32, bytes: u32);
}

unsafe fn rand32() -> u32 {
    prandom_u32_state(&raw mut rng)
}

/* Reference implementation using dumb byte-wise XOR */
unsafe fn xor_ref(dest: *mut core::ffi::c_void, srcs: *mut *mut core::ffi::c_void,
    src_cnt: u32, bytes: u32) {
    let d = dest as *mut u8;
    for off in 0..bytes as usize {
        for idx in 0..src_cnt as usize {
            let src = *srcs.add(idx) as *mut u8;
            *d.add(off) ^= *src.add(off);
        }
    }
}

/* Generate a random length that is a multiple of 512. */
unsafe fn random_length(max_length: u32) -> u32 {
    ((rand32() % max_length) + 1 + 511) & !511
}

/* Generate a random alignment that is a multiple of 64. */
unsafe fn random_alignment(max_alignment: u32) -> u32 {
    ((rand32() % max_alignment) + 1) & !63
}

unsafe fn xor_generate_random_data() {
    prandom_bytes_state(&raw mut rng, test_dest, test_buflen);
    core::ptr::copy_nonoverlapping(test_dest as *const u8, test_ref as *mut u8, test_buflen);
    for i in 0..XOR_KUNIT_MAX_BUFFERS {
        prandom_bytes_state(&raw mut rng, test_buffers[i], test_buflen);
    }
}

/* Test that xor_gen gives the same result as a reference implementation. */
unsafe fn xor_test(test: *mut kunit) {
    let mut aligned_buffers = [core::ptr::null_mut(); XOR_KUNIT_MAX_BUFFERS];
    for _i in 0..XOR_KUNIT_NUM_TEST_ITERS {
        let nr_buffers = (rand32() % XOR_KUNIT_MAX_BUFFERS as u32) + 1;
        let len = random_length(XOR_KUNIT_MAX_BYTES as u32);
        let max_alignment = XOR_KUNIT_MAX_BYTES as u32 - len;
        let mut align = 0u32;
        let buffers: *mut *mut core::ffi::c_void;

        if rand32() % 8 == 0 {
            /* Refresh the data occasionally. */
            xor_generate_random_data();
        }

        /* If we're not using the entire buffer size, inject randomized alignment into the buffer. */
        if max_alignment == 0 {
            buffers = test_buffers.as_mut_ptr();
        } else if rand32() % 2 == 0 {
            /* Use random alignments mod 64 */
            for j in 0..nr_buffers as usize {
                aligned_buffers[j] = test_buffers[j].add(random_alignment(max_alignment) as usize);
            }
            buffers = aligned_buffers.as_mut_ptr();
            align = random_alignment(max_alignment);
        } else {
            /* Go up to the guard page, to catch buffer overreads */
            align = test_buflen as u32 - len;
            for j in 0..nr_buffers as usize {
                aligned_buffers[j] = test_buffers[j].add(align as usize);
            }
            buffers = aligned_buffers.as_mut_ptr();
        }

        /* Compute the XOR, and verify that it equals the XOR computed by a simple byte-at-a-time reference implementation. */
        xor_ref(test_ref.add(align as usize), buffers, nr_buffers, len);
        xor_gen(test_dest.add(align as usize), buffers, nr_buffers, len);
        // KUNIT_EXPECT_MEMEQ_MSG(test, test_ref + align, test_dest + align, len, ...)
        let _ = test;
    }
}

unsafe fn xor_benchmark(test: *mut kunit) {
    const NR_TO_TEST: [u32; 10] = [4, 5, 6, 7, 8, 10, 12, 15, 16, 32];
    const LEN_TO_TEST: [u32; 2] = [4096, 16384];
    let _ = test;
    // CONFIG_XOR_BENCHMARK conditional and KUnit logging/assertion macros are external.
    for i in 0..NR_TO_TEST.len() {
        for j in 0..LEN_TO_TEST.len() {
            for _l in 0..10 {
                xor_gen(test_dest, test_buffers.as_mut_ptr(), NR_TO_TEST[i], LEN_TO_TEST[j]);
            }
        }
    }
    // The remaining timing, preemption, logging, and assertion operations are supplied by KUnit.
}

static mut xor_test_cases: [*const core::ffi::c_void; 3] = [
    xor_test as *const core::ffi::c_void,
    xor_benchmark as *const core::ffi::c_void,
    core::ptr::null(),
];

unsafe fn xor_suite_init(_suite: *mut kunit_suite) -> i32 {
    test_buflen = (XOR_KUNIT_MAX_BYTES + 4095) & !4095;
    test_ref = vmalloc(test_buflen);
    if test_ref.is_null() { return -12; }
    test_dest = vmalloc(test_buflen);
    if test_dest.is_null() { vfree(test_ref); return -12; }
    for i in 0..XOR_KUNIT_MAX_BUFFERS {
        test_buffers[i] = vmalloc(test_buflen);
        if test_buffers[i].is_null() {
            for j in (0..i).rev() { vfree(test_buffers[j]); }
            vfree(test_dest);
            vfree(test_ref);
            return -12;
        }
    }
    prandom_seed_state(&raw mut rng, XOR_KUNIT_SEED);
    xor_generate_random_data();
    0
}

unsafe fn xor_suite_exit(_suite: *mut kunit_suite) {
    vfree(test_ref);
    vfree(test_dest);
    for i in 0..XOR_KUNIT_MAX_BUFFERS { vfree(test_buffers[i]); }
}

#[repr(C)]
struct xor_test_suite {
    name: *const u8,
    test_cases: *mut *const core::ffi::c_void,
    suite_init: unsafe fn(*mut kunit_suite) -> i32,
    suite_exit: unsafe fn(*mut kunit_suite),
}

static mut xor_test_suite_instance: xor_test_suite = xor_test_suite {
    name: b"xor\0".as_ptr(),
    test_cases: unsafe { xor_test_cases.as_mut_ptr() },
    suite_init: xor_suite_init,
    suite_exit: xor_suite_exit,
};

// kunit_test_suite(xor_test_suite);
// MODULE_DESCRIPTION("Unit test for the XOR library functions");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
