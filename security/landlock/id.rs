// SPDX-License-Identifier: GPL-2.0-only
/*
 * Landlock - Unique identification number generator
 *
 * Copyright (c) 2024-2025 Microsoft Corporation
 */

// Rust translation of dependencies from:
// #include <kunit/test.h>
// #include <linux/atomic.h>
// #include <linux/bitops.h>
// #include <linux/random.h>
// #include <linux/spinlock.h>
// #include "common.h"
// #include "id.h"

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]

type size_t = usize;
type u8 = core::ffi::c_uchar;
type u32 = core::ffi::c_uint;
type u64 = core::ffi::c_ulonglong;

const COUNTER_PRE_INIT: u64 = 0;
const U32_MAX: u64 = u32::MAX as u64;

#[repr(C)]
pub struct atomic64_t {
    counter: i64,
}

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_case {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_suite {
    name: *const core::ffi::c_char,
    test_cases: *mut kunit_case,
}

unsafe extern "C" {
    fn atomic64_cmpxchg(v: *mut atomic64_t, old: i64, new: i64) -> i64;
    fn atomic64_read(v: *const atomic64_t) -> i64;
    fn atomic64_fetch_add(i: i64, v: *mut atomic64_t) -> i64;
    fn atomic64_set(v: *mut atomic64_t, i: i64);
    fn get_random_u32() -> u32;
    fn get_random_u8() -> u8;
    fn WARN_ON_ONCE(condition: bool) -> bool;

    #[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
    fn KUNIT_EXPECT_EQ(test: *mut kunit, left: u64, right: u64);
    #[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
    fn KUNIT_EXPECT_EQ_MSG(
        test: *mut kunit,
        left: u64,
        right: u64,
        message: *const core::ffi::c_char,
    );
}

const fn ATOMIC64_INIT(i: u64) -> atomic64_t {
    atomic64_t { counter: i as i64 }
}

const fn BIT_ULL(nr: u32) -> u64 {
    1u64 << nr
}

static mut next_id: atomic64_t = ATOMIC64_INIT(COUNTER_PRE_INIT);

unsafe fn init_id(counter: *mut atomic64_t, random_32bits: u32) {
    let mut init: u64;

    /*
     * Ensures sure 64-bit values are always used by user space (or may
     * fail with -EOVERFLOW), and makes this testable.
     */
    init = BIT_ULL(32);

    /*
     * Makes a large (2^32) boot-time value to limit ID collision in logs
     * from different boots, and to limit info leak about the number of
     * initially (relative to the reader) created elements (e.g. domains).
     */
    init = init.wrapping_add(random_32bits as u64);

    /* Sets first or ignores.  This will be the first ID. */
    unsafe {
        atomic64_cmpxchg(counter, COUNTER_PRE_INIT as i64, init as i64);
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_init_min(test: *mut kunit) {
    let mut counter: atomic64_t = ATOMIC64_INIT(COUNTER_PRE_INIT);

    unsafe {
        init_id(&mut counter, 0);
        KUNIT_EXPECT_EQ(test, atomic64_read(&counter) as u64, 1u64 + U32_MAX);
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_init_max(test: *mut kunit) {
    let mut counter: atomic64_t = ATOMIC64_INIT(COUNTER_PRE_INIT);

    unsafe {
        init_id(&mut counter, !0);
        KUNIT_EXPECT_EQ(test, atomic64_read(&counter) as u64, 1 + (2u64 * U32_MAX));
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_init_once(test: *mut kunit) {
    let first_init: u64 = 1u64 + U32_MAX;
    let mut counter: atomic64_t = ATOMIC64_INIT(COUNTER_PRE_INIT);

    unsafe {
        init_id(&mut counter, 0);
        KUNIT_EXPECT_EQ(test, atomic64_read(&counter) as u64, first_init);

        init_id(&mut counter, !0);
        KUNIT_EXPECT_EQ_MSG(
            test,
            atomic64_read(&counter) as u64,
            first_init,
            c"Should still have the same value after the subsequent init_id()".as_ptr(),
        );
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn landlock_init_id() {
    unsafe {
        return init_id(&raw mut next_id, get_random_u32());
    }
}

/*
 * It's not worth it to try to hide the monotonic counter because it can still
 * be inferred (with N counter ranges), and if we are allowed to read the inode
 * number we should also be allowed to read the time creation anyway, and it
 * can be handy to store and sort domain IDs for user space.
 *
 * Returns the value of next_id and increment it to let some space for the next
 * one.
 */
unsafe fn get_id_range(
    mut number_of_ids: size_t,
    counter: *mut atomic64_t,
    mut random_4bits: u8,
) -> u64 {
    let id: u64;
    let step: u64;

    /*
     * We should return at least 1 ID, and we may need a set of consecutive
     * ones (e.g. to generate a set of inodes).
     */
    if unsafe { WARN_ON_ONCE(number_of_ids <= 0) } {
        number_of_ids = 1;
    }

    /*
     * Blurs the next ID guess with 1/16 ratio.  We get 2^(64 - 4) -
     * (2 * 2^32), so a bit less than 2^60 available IDs, which should be
     * much more than enough considering the number of CPU cycles required
     * to get a new ID (e.g. a full landlock_restrict_self() call), and the
     * cost of draining all available IDs during the system's uptime.
     */
    random_4bits &= 0b1111;
    step = (number_of_ids as u64).wrapping_add(random_4bits as u64);

    /* It is safe to cast a signed atomic to an unsigned value. */
    id = unsafe { atomic64_fetch_add(step as i64, counter) as u64 };

    /* Warns if landlock_init_id() was not called. */
    unsafe {
        WARN_ON_ONCE(id == COUNTER_PRE_INIT);
    }
    return id;
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn get_random_u8_positive() -> u8 {
    /* max() evaluates its arguments once. */
    let random = unsafe { get_random_u8() };
    if 1 > random {
        1
    } else {
        random
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_range1_rand0(test: *mut kunit) {
    let mut counter: atomic64_t = ATOMIC64_INIT(0);
    let init: u64;

    unsafe {
        init = get_random_u32() as u64;
        atomic64_set(&mut counter, init as i64);
        KUNIT_EXPECT_EQ(test, get_id_range(1, &mut counter, 0), init);
        KUNIT_EXPECT_EQ(
            test,
            get_id_range(
                get_random_u8_positive() as size_t,
                &mut counter,
                get_random_u8(),
            ),
            init.wrapping_add(1),
        );
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_range1_rand1(test: *mut kunit) {
    let mut counter: atomic64_t = ATOMIC64_INIT(0);
    let init: u64;

    unsafe {
        init = get_random_u32() as u64;
        atomic64_set(&mut counter, init as i64);
        KUNIT_EXPECT_EQ(test, get_id_range(1, &mut counter, 1), init);
        KUNIT_EXPECT_EQ(
            test,
            get_id_range(
                get_random_u8_positive() as size_t,
                &mut counter,
                get_random_u8(),
            ),
            init.wrapping_add(2),
        );
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_range1_rand15(test: *mut kunit) {
    let mut counter: atomic64_t = ATOMIC64_INIT(0);
    let init: u64;

    unsafe {
        init = get_random_u32() as u64;
        atomic64_set(&mut counter, init as i64);
        KUNIT_EXPECT_EQ(test, get_id_range(1, &mut counter, 15), init);
        KUNIT_EXPECT_EQ(
            test,
            get_id_range(
                get_random_u8_positive() as size_t,
                &mut counter,
                get_random_u8(),
            ),
            init.wrapping_add(16),
        );
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_range1_rand16(test: *mut kunit) {
    let mut counter: atomic64_t = ATOMIC64_INIT(0);
    let init: u64;

    unsafe {
        init = get_random_u32() as u64;
        atomic64_set(&mut counter, init as i64);
        KUNIT_EXPECT_EQ(test, get_id_range(1, &mut counter, 16), init);
        KUNIT_EXPECT_EQ(
            test,
            get_id_range(
                get_random_u8_positive() as size_t,
                &mut counter,
                get_random_u8(),
            ),
            init.wrapping_add(1),
        );
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_range2_rand0(test: *mut kunit) {
    let mut counter: atomic64_t = ATOMIC64_INIT(0);
    let init: u64;

    unsafe {
        init = get_random_u32() as u64;
        atomic64_set(&mut counter, init as i64);
        KUNIT_EXPECT_EQ(test, get_id_range(2, &mut counter, 0), init);
        KUNIT_EXPECT_EQ(
            test,
            get_id_range(
                get_random_u8_positive() as size_t,
                &mut counter,
                get_random_u8(),
            ),
            init.wrapping_add(2),
        );
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_range2_rand1(test: *mut kunit) {
    let mut counter: atomic64_t = ATOMIC64_INIT(0);
    let init: u64;

    unsafe {
        init = get_random_u32() as u64;
        atomic64_set(&mut counter, init as i64);
        KUNIT_EXPECT_EQ(test, get_id_range(2, &mut counter, 1), init);
        KUNIT_EXPECT_EQ(
            test,
            get_id_range(
                get_random_u8_positive() as size_t,
                &mut counter,
                get_random_u8(),
            ),
            init.wrapping_add(3),
        );
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_range2_rand2(test: *mut kunit) {
    let mut counter: atomic64_t = ATOMIC64_INIT(0);
    let init: u64;

    unsafe {
        init = get_random_u32() as u64;
        atomic64_set(&mut counter, init as i64);
        KUNIT_EXPECT_EQ(test, get_id_range(2, &mut counter, 2), init);
        KUNIT_EXPECT_EQ(
            test,
            get_id_range(
                get_random_u8_positive() as size_t,
                &mut counter,
                get_random_u8(),
            ),
            init.wrapping_add(4),
        );
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_range2_rand15(test: *mut kunit) {
    let mut counter: atomic64_t = ATOMIC64_INIT(0);
    let init: u64;

    unsafe {
        init = get_random_u32() as u64;
        atomic64_set(&mut counter, init as i64);
        KUNIT_EXPECT_EQ(test, get_id_range(2, &mut counter, 15), init);
        KUNIT_EXPECT_EQ(
            test,
            get_id_range(
                get_random_u8_positive() as size_t,
                &mut counter,
                get_random_u8(),
            ),
            init.wrapping_add(17),
        );
    }
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
unsafe fn test_range2_rand16(test: *mut kunit) {
    let mut counter: atomic64_t = ATOMIC64_INIT(0);
    let init: u64;

    unsafe {
        init = get_random_u32() as u64;
        atomic64_set(&mut counter, init as i64);
        KUNIT_EXPECT_EQ(test, get_id_range(2, &mut counter, 16), init);
        KUNIT_EXPECT_EQ(
            test,
            get_id_range(
                get_random_u8_positive() as size_t,
                &mut counter,
                get_random_u8(),
            ),
            init.wrapping_add(2),
        );
    }
}

/**
 * landlock_get_id_range - Get a range of unique IDs
 *
 * @number_of_ids: Number of IDs to hold.  Must be greater than one.
 *
 * Return: The first ID in the range.
 */
#[unsafe(no_mangle)]
pub unsafe extern "C" fn landlock_get_id_range(number_of_ids: size_t) -> u64 {
    unsafe { return get_id_range(number_of_ids, &raw mut next_id, get_random_u8()) };
}

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
static mut test_cases: [kunit_case; 13] = [
    /* clang-format off */
    // KUNIT_CASE(test_init_min),
    // KUNIT_CASE(test_init_max),
    // KUNIT_CASE(test_init_once),
    // KUNIT_CASE(test_range1_rand0),
    // KUNIT_CASE(test_range1_rand1),
    // KUNIT_CASE(test_range1_rand15),
    // KUNIT_CASE(test_range1_rand16),
    // KUNIT_CASE(test_range2_rand0),
    // KUNIT_CASE(test_range2_rand1),
    // KUNIT_CASE(test_range2_rand2),
    // KUNIT_CASE(test_range2_rand15),
    // KUNIT_CASE(test_range2_rand16),
    // {}
    /* clang-format on */
    kunit_case { _private: [] },
    kunit_case { _private: [] },
    kunit_case { _private: [] },
    kunit_case { _private: [] },
    kunit_case { _private: [] },
    kunit_case { _private: [] },
    kunit_case { _private: [] },
    kunit_case { _private: [] },
    kunit_case { _private: [] },
    kunit_case { _private: [] },
    kunit_case { _private: [] },
    kunit_case { _private: [] },
    kunit_case { _private: [] },
];

#[cfg(CONFIG_SECURITY_LANDLOCK_KUNIT_TEST)]
static mut test_suite: kunit_suite = kunit_suite {
    name: c"landlock_id".as_ptr(),
    test_cases: unsafe { &raw mut test_cases as *mut kunit_case },
};

// CONFIG_SECURITY_LANDLOCK_KUNIT_TEST: kunit_test_init_section_suite(test_suite);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
