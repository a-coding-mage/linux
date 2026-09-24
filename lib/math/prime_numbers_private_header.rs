/* SPDX-License-Identifier: GPL-2.0 */
//! Private prime-cache ABI. Layout and callback identity come from C bindings.

/// The original C header with its real RCU head and inline flexible array.
#[allow(non_camel_case_types)]
pub type primes = kernel::bindings::primes;

/// Original non-null C callback contract, available for built-in or modular tests.
#[cfg(CONFIG_PRIME_NUMBERS_KUNIT_TEST)]
#[allow(non_camel_case_types)]
pub type primes_fn = kernel::bindings::primes_fn;

/// Invoke a callback while the actual published prime cache is RCU-protected.
///
/// # Safety
/// The callback must be non-null, valid for `ctx`, must not mutate/retain the
/// supplied snapshot, and must obey RCU read-side restrictions.
#[cfg(CONFIG_PRIME_NUMBERS_KUNIT_TEST)]
#[inline(always)]
pub unsafe fn with_primes(ctx: *mut kernel::ffi::c_void, callback: primes_fn) {
    // SAFETY: The caller guarantees the original callback contract.
    unsafe { kernel::bindings::with_primes(ctx, callback) };
}

/// Original trial-division reference used by the unchanged C KUnit suite.
#[cfg(CONFIG_PRIME_NUMBERS_KUNIT_TEST)]
#[inline(always)]
pub fn slow_is_prime_number(x: kernel::ffi::c_ulong) -> bool {
    // SAFETY: The original function accepts every native unsigned-long value.
    unsafe { kernel::bindings::slow_is_prime_number(x) }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
