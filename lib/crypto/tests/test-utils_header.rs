/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Test utility functions shared by the crypto library tests.
 *
 * For now this is simply a header that's included into the KUnit test suites
 * that need it.  If this gets too large it could be made its own translation
 * unit and libcrypto_test_utils module, but that seems overkill for now.
 */

// C dependencies supplied by the surrounding kernel/KUnit environment:
// kunit/test.h, linux/math.h, linux/minmax.h, linux/string.h, linux/vmalloc.h

static mut RANDOM_SEED: u64 = 0;

unsafe fn action_free_guarded_buf(buf: *mut core::ffi::c_void) {
    vfree(buf);
}

/*
 * Allocate a KUnit-managed buffer that has length @size bytes (> 0) immediately
 * followed by an unmapped page, and assert that the allocation succeeds.
 */
unsafe fn alloc_guarded_buf(test: *mut kunit, size: usize) -> *mut core::ffi::c_void {
    let full_size = round_up(size, PAGE_SIZE);
    let buf = vmalloc(full_size);

    KUNIT_ASSERT_NOT_NULL!(test, buf);
    KUNIT_ASSERT_EQ!(test, 0, kunit_add_action_or_reset(test, action_free_guarded_buf, buf));
    (buf as *mut u8).add(full_size - size) as *mut core::ffi::c_void
}

unsafe fn alloc_buf(test: *mut kunit, size: usize) -> *mut core::ffi::c_void {
    let buf = kunit_kmalloc(test, size, GFP_KERNEL);

    KUNIT_ASSERT_NOT_NULL!(test, buf);
    buf
}

unsafe fn memdup_buf(
    test: *mut kunit,
    src: *const core::ffi::c_void,
    size: usize,
) -> *mut core::ffi::c_void {
    let dst = alloc_buf(test, size);

    memcpy(dst, src, size)
}

/*
 * This is a simple linear congruential generator.  It is used only for testing,
 * which does not require cryptographically secure random numbers.  A hard-coded
 * algorithm is used instead of <linux/prandom.h> so that it matches the
 * algorithm used by the test vector generation script.  This allows the input
 * data in random test vectors to be concisely stored as just the seed.
 */
unsafe fn rand32() -> u32 {
    RANDOM_SEED = (RANDOM_SEED
        .wrapping_mul(25_214_903_917)
        .wrapping_add(11))
        & ((1u64 << 48) - 1);
    (RANDOM_SEED >> 16) as u32
}

unsafe fn rand_bytes(out: *mut u8, len: usize) {
    for i in 0..len {
        *out.add(i) = rand32() as u8;
    }
}

unsafe fn rand_bytes_seeded_from_len(out: *mut u8, len: usize) {
    RANDOM_SEED = len as u64;
    rand_bytes(out, len);
}

unsafe fn rand_bool() -> bool {
    rand32() % 2 != 0
}

/* Generate a random length, preferring small lengths. */
unsafe fn rand_length(max_len: usize) -> usize {
    let len = match rand32() % 3 {
        0 => rand32() as usize % 128,
        1 => rand32() as usize % 3072,
        _ => rand32() as usize,
    };
    len % (max_len + 1)
}

unsafe fn rand_offset(max_offset: usize) -> usize {
    core::cmp::min(rand32() as usize % 128, max_offset)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
