// SPDX-License-Identifier: GPL-2.0-only

// Dependencies supplied by the surrounding kernel/KUnit environment:
// <kunit/test.h>, <linux/module.h>, <linux/prime_numbers.h>, and
// ../prime_numbers_private.h

use core::ffi::{c_char, c_int, c_ulong, c_void};

#[repr(C)]
pub struct kunit {
    _private: [u8; 0],
}

#[repr(C)]
pub struct kunit_suite {
    pub name: *const c_char,
    pub suite_exit: Option<unsafe extern "C" fn(*mut kunit_suite)>,
    pub test_cases: *mut kunit_case,
}

#[repr(C)]
pub struct kunit_case {
    pub run_case: Option<unsafe extern "C" fn(*mut kunit)>,
}

#[repr(C)]
pub struct primes {
    pub last: c_ulong,
    pub sz: c_ulong,
    pub primes: *mut c_ulong,
}

extern "C" {
    fn slow_is_prime_number(x: c_ulong) -> bool;
    fn is_prime_number(x: c_ulong) -> bool;
    fn next_prime_number(last: c_ulong) -> c_ulong;
    fn with_primes(
        suite: *mut kunit_suite,
        callback: unsafe extern "C" fn(*mut c_void, *const primes),
    );
    fn kunit_info(suite: *mut kunit_suite, fmt: *const c_char, ...);
    fn kunit_assert_eq_msg<T>(test: *mut kunit, left: T, right: T, fmt: *const c_char, ...);
}

unsafe extern "C" fn dump_primes(ctx: *mut c_void, p: *const primes) {
    let suite = ctx as *mut kunit_suite;
    let p = &*p;
    let last_prime = *p.primes.add(((p.sz + (usize::BITS as c_ulong) - 1)
        / (usize::BITS as c_ulong)) as usize - 1);

    kunit_info(
        suite,
        c"primes.{last=%lu, .sz=%lu, .primes[]=...x%lx} = %*pbl".as_ptr(),
        p.last,
        p.sz,
        last_prime,
        p.sz as c_int,
        p.primes,
    );
}

unsafe extern "C" fn prime_numbers_test(test: *mut kunit) {
    const MAX: c_ulong = 65536;
    let mut x: c_ulong;
    let mut last: c_ulong = 0;
    let mut next: c_ulong;

    x = 2;
    while x < MAX {
        let slow = slow_is_prime_number(x);
        let fast = is_prime_number(x);

        kunit_assert_eq_msg(test, slow, fast, c"is-prime(%lu)".as_ptr(), x);

        if !slow {
            x += 1;
            continue;
        }

        next = next_prime_number(last);
        kunit_assert_eq_msg(test, next, x, c"next-prime(%lu)".as_ptr(), last);
        last = next;
        x += 1;
    }
}

unsafe extern "C" fn kunit_suite_exit(suite: *mut kunit_suite) {
    with_primes(suite, dump_primes);
}

static mut prime_numbers_cases: [kunit_case; 2] = [
    kunit_case {
        run_case: Some(prime_numbers_test),
    },
    kunit_case { run_case: None },
];

static mut prime_numbers_suite: kunit_suite = kunit_suite {
    name: c"math-prime_numbers".as_ptr(),
    suite_exit: Some(kunit_suite_exit),
    test_cases: prime_numbers_cases.as_mut_ptr(),
};

// Equivalent of: kunit_test_suite(prime_numbers_suite);
// MODULE_AUTHOR("Intel Corporation");
// MODULE_DESCRIPTION("Prime number library");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
