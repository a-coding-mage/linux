/* SPDX-License-Identifier: GPL-2.0 */
//! Original prime-number interface and inclusive iterator semantics.

use kernel::{bindings, ffi::c_ulong};

/// Test primality using the selected native provider. May sleep when growing.
#[inline(always)]
pub fn is_prime_number(x: c_ulong) -> bool {
    // SAFETY: The original ABI has no value or pointer preconditions.
    unsafe { bindings::is_prime_number(x) }
}

/// Return a strictly later prime, or ULONG_MAX as the original sentinel.
/// Cache growth may sleep; allocation failure uses the original slow fallback.
#[inline(always)]
pub fn next_prime_number(x: c_ulong) -> c_ulong {
    // SAFETY: The original ABI accepts every native unsigned long value.
    unsafe { bindings::next_prime_number(x) }
}

/// Iteration corresponding to the two original C for_each macros.
///
/// The first value is the supplied starting point unchanged, even for zero,
/// one or composites. Subsequent values use the selected native provider.
/// Work is deferred until next() so breaking after a value never grows cache.
pub struct PrimeNumbers {
    current: c_ulong,
    maximum: c_ulong,
    advance: bool,
    finished: bool,
}

impl Iterator for PrimeNumbers {
    type Item = c_ulong;

    #[inline(always)]
    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        if self.advance {
            if self.current == c_ulong::MAX {
                // The C macros require max < ULONG_MAX for termination. For
                // the otherwise nonterminating bound, yield its sentinel once.
                self.finished = true;
                return None;
            }
            self.current = next_prime_number(self.current);
        }
        if self.current > self.maximum {
            self.finished = true;
            return None;
        }
        self.advance = true;
        Some(self.current)
    }
}

impl core::iter::FusedIterator for PrimeNumbers {}

/// Iterate inclusively from the original first prime, two, through `maximum`.
#[inline(always)]
pub const fn for_each_prime_number(maximum: c_ulong) -> PrimeNumbers {
    for_each_prime_number_from(2, maximum)
}

/// Yield `from` unchanged first, then successive primes through `maximum`.
/// Prefer `maximum < ULONG_MAX`, as required by the original C macro contract.
#[inline(always)]
pub const fn for_each_prime_number_from(from: c_ulong, maximum: c_ulong) -> PrimeNumbers {
    PrimeNumbers {
        current: from,
        maximum,
        advance: false,
        finished: false,
    }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
