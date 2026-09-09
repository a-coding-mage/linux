// SPDX-License-Identifier: GPL-2.0-only
/*
 * Test cases for <linux/hash.h> and <linux/stringhash.h>
 * This just verifies that various ways of computing a hash
 * produce the same thing and, for cases where a k-bit hash
 * value is requested, is of the requested size.
 *
 * We fill a buffer with a 255-byte null-terminated string,
 * and use both full_name_hash() and hashlen_string() to hash the
 * substrings from i to j, where 0 <= i < j < 256.
 *
 * The returned values are used to check that __hash_32() and
 * __hash_32_generic() compute the same thing.  Likewise hash_32()
 * and hash_64().
 */

/* Kernel and KUnit dependencies are supplied by other translation units. */

/* 32-bit XORSHIFT generator.  Seed must not be zero. */
fn xorshift(mut seed: u32) -> u32 {
    seed ^= seed << 13;
    seed ^= seed >> 17;
    seed ^= seed << 5;
    seed
}

/* Given a non-zero x, returns a non-zero byte. */
fn mod255(mut x: u32) -> u8 {
    x = (x & 0xffff) + (x >> 16); // 1 <= x <= 0x1fffe
    x = (x & 0xff) + (x >> 8); // 1 <= x <= 0x2fd
    x = (x & 0xff) + (x >> 8); // 1 <= x <= 0x100
    x = (x & 0xff) + (x >> 8); // 1 <= x <= 0xff
    x as u8
}

/* Fill the buffer with non-zero bytes. */
fn fill_buf(buf: *mut i8, len: usize, mut seed: u32) {
    for i in 0..len {
        seed = xorshift(seed);
        unsafe { *buf.add(i) = mod255(seed) as i8; }
    }
}

/* Holds most testing variables for the int test. */
#[repr(C)]
struct test_hash_params {
    /* Pointer to integer to be hashed. */
    h64: *mut u64,
    /* Low 32-bits of integer to be hashed. */
    h0: u32,
    /* Arch-specific hash result. */
    h1: u32,
    /* Generic hash result. */
    h2: u32,
    /* ORed hashes of given size (in bits). */
    hash_or: *mut [[u32; 33]; 2],
}

/* #ifdef HAVE_ARCH__HASH_32 */
unsafe fn test_int__hash_32(test: *mut kunit, params: *mut test_hash_params) {
    (*(*params).hash_or)[1][0] |= {
        (*params).h2 = __hash_32_generic((*params).h0);
        (*params).h2
    };
    /* #if HAVE_ARCH__HASH_32 == 1 */
    KUNIT_EXPECT_EQ_MSG(test, (*params).h1, (*params).h2,
        "__hash_32(%#x) = %#x != __hash_32_generic() = %#x",
        (*params).h0, (*params).h1, (*params).h2);
    /* #endif */
}
/* #endif */

/* #ifdef HAVE_ARCH_HASH_64 */
unsafe fn test_int_hash_64(test: *mut kunit, params: *mut test_hash_params,
                           m: *const u32, k: *const i32) {
    (*params).h2 = hash_64_generic(*(*params).h64, *k);
    /* #if HAVE_ARCH_HASH_64 == 1 */
    KUNIT_EXPECT_EQ_MSG(test, (*params).h1, (*params).h2,
        "hash_64(%#llx, %d) = %#x != hash_64_generic() = %#x",
        *(*params).h64, *k, (*params).h1, (*params).h2);
    /* #else */
    KUNIT_EXPECT_LE_MSG(test, (*params).h1, (*params).h2,
        "hash_64_generic(%#llx, %d) = %#x > %#x",
        *(*params).h64, *k, (*params).h1, *m);
    /* #endif */
}
/* #endif */

/*
 * Test the various integer hash functions.  h64 (or its low-order bits)
 * is the integer to hash.  hash_or accumulates the OR of the hash values,
 * which are later checked to see that they cover all the requested bits.
 *
 * Because these functions (as opposed to the string hashes) are all
 * inline, the code being tested is actually in the module, and you can
 * recompile and re-test the module without rebooting.
 */
unsafe fn test_int_hash(test: *mut kunit, mut h64: u64, hash_or: *mut [[u32; 33]; 2]) {
    let mut params = test_hash_params { h64: &mut h64, h0: h64 as u32, h1: 0, h2: 0, hash_or };

    (*hash_or)[0][0] |= { params.h1 = __hash_32(params.h0); params.h1 };
    test_int__hash_32(test, &mut params);

    for k in 1..=32 {
        let m: u32 = ((2u32 << (k - 1)) - 1);
        (*hash_or)[0][k as usize] |= { params.h1 = hash_32(params.h0, k); params.h1 };
        KUNIT_EXPECT_LE_MSG(test, params.h1, m, "hash_32(%#x, %d) = %#x > %#x", params.h0, k, params.h1, m);
        (*hash_or)[1][k as usize] |= { params.h1 = hash_64(h64, k); params.h1 };
        KUNIT_EXPECT_LE_MSG(test, params.h1, m, "hash_64(%#llx, %d) = %#x > %#x", h64, k, params.h1, m);
        test_int_hash_64(test, &mut params, &m, &k);
    }
}

const SIZE: usize = 256; // Run time is cubic in SIZE

unsafe fn test_string_or(test: *mut kunit) {
    let mut buf = [0i8; SIZE + 1];
    let mut string_or = 0u32;
    fill_buf(buf.as_mut_ptr(), SIZE, 1);
    for j in (1..=SIZE).rev() {
        buf[j] = 0;
        for i in 0..=j {
            string_or |= full_name_hash(buf.as_ptr().add(i), buf.as_ptr().add(i), j - i);
        }
    }
    KUNIT_EXPECT_EQ_MSG(test, string_or, u32::MAX, "OR of all string hash results = %#x != %#x", string_or, u32::MAX);
}

unsafe fn test_hash_or(test: *mut kunit) {
    let mut buf = [0i8; SIZE + 1];
    let mut hash_or = [[0u32; 33]; 2];
    let mut h64 = 0u64;
    fill_buf(buf.as_mut_ptr(), SIZE, 1);
    for j in (1..=SIZE).rev() {
        buf[j] = 0;
        for i in 0..=j {
            let hashlen = hashlen_string(buf.as_ptr().add(i), buf.as_ptr().add(i));
            let h0 = full_name_hash(buf.as_ptr().add(i), buf.as_ptr().add(i), j - i);
            KUNIT_EXPECT_EQ_MSG(test, hashlen_len(hashlen), (j - i) as u32, "hashlen_string(%d..%d) returned length %u, expected %d", i, j, hashlen_len(hashlen), j - i);
            KUNIT_EXPECT_EQ_MSG(test, hashlen_hash(hashlen), h0, "hashlen_string(%d..%d) = %08x != full_name_hash() = %08x", i, j, hashlen_hash(hashlen), h0);
            h64 = (h64 << 32) | h0 as u64;
            test_int_hash(test, h64, &mut hash_or);
        }
    }
    KUNIT_EXPECT_EQ_MSG(test, hash_or[0][0], u32::MAX, "OR of all __hash_32 results = %#x != %#x", hash_or[0][0], u32::MAX);
    for i in 1..=32 {
        let m = (2u32 << (i - 1)) - 1;
        KUNIT_EXPECT_EQ_MSG(test, hash_or[0][i], m, "OR of all hash_32(%d) results = %#x (%#x expected)", i, hash_or[0][i], m);
        KUNIT_EXPECT_EQ_MSG(test, hash_or[1][i], m, "OR of all hash_64(%d) results = %#x (%#x expected)", i, hash_or[1][i], m);
    }
}

/* KUnit case and suite registration declarations retained as external integration points. */
static mut hash_test_cases: [*const core::ffi::c_void; 3] = [core::ptr::null(); 3];
static mut hash_test_suite: *const core::ffi::c_void = core::ptr::null();

/* kunit_test_suite(hash_test_suite); */
/* MODULE_DESCRIPTION("Test cases for <linux/hash.h> and <linux/stringhash.h>"); */
/* MODULE_LICENSE("GPL"); */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
