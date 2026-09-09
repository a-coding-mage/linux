/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the Linux types/environment.
use core::ffi::c_void;
use core::ffi::c_ulong;

// The declarations below are enabled in C when
// IS_ENABLED(CONFIG_PRIME_NUMBERS_KUNIT_TEST) is true.  The build-time
// condition is preserved here as intent; its configuration is supplied by
// the surrounding build.

#[repr(C)]
pub struct primes {
    pub rcu: rcu_head,
    pub last: c_ulong,
    pub sz: c_ulong,
    pub primes: [c_ulong; 0],
}

pub type primes_fn = Option<unsafe extern "C" fn(*mut c_void, *const primes)>;

unsafe extern "C" {
    pub fn with_primes(ctx: *mut c_void, fn_: primes_fn);
    pub fn slow_is_prime_number(x: c_ulong) -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
