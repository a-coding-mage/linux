// SPDX-License-Identifier: GPL-2.0-only
//! Native C ABI owner for greatest common divisors and least common multiples.
//!
//! Pure Rust callers use `kernel::math`. Only this crate owns the unmangled
//! functions and the real mutable key shared with architecture setup code.

#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

// The native entry points use explicit runtime strategies; the same sources
// also provide configured, constant-evaluable APIs for pure Rust consumers.
#[allow(dead_code, unreachable_pub)]
#[path = "gcd.rs"]
mod greatest_common_divisor;
#[allow(dead_code, unreachable_pub)]
#[path = "lcm.rs"]
mod least_common_multiple;

/// Initially enabled key matching C's `DEFINE_STATIC_KEY_TRUE` storage.
///
/// This is deliberately not exported to modules. For example, RISC-V setup
/// disables this same key when the Zbb extension is unavailable.
#[no_mangle]
pub static efficient_ffs_key: kernel::jump_label::StaticKeyTrue =
    kernel::jump_label::StaticKeyTrue::new();

fn efficient_ffs() -> bool {
    #[cfg(not(CONFIG_CPU_NO_EFFICIENT_FFS))]
    {
        kernel::static_branch_likely!(efficient_ffs_key)
    }
    #[cfg(CONFIG_CPU_NO_EFFICIENT_FFS)]
    {
        false
    }
}

/// Returns the greatest common divisor with C's native unsigned-long ABI.
#[no_mangle]
pub extern "C" fn gcd(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return a | b;
    }
    greatest_common_divisor::gcd_with_ffs(a, b, efficient_ffs())
}

/// Returns the least common multiple with unsigned-long wrapping arithmetic.
#[no_mangle]
pub extern "C" fn lcm(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return 0;
    }
    least_common_multiple::lcm_with_ffs(a, b, efficient_ffs())
}

/// Returns the LCM, or the nonzero input when the LCM is zero.
#[no_mangle]
pub extern "C" fn lcm_not_zero(a: usize, b: usize) -> usize {
    if a == 0 || b == 0 {
        return a | b;
    }
    least_common_multiple::lcm_not_zero_with_ffs(a, b, efficient_ffs())
}

ffi_export::export_symbol!(gcd, gcd, "GPL", "");
ffi_export::export_symbol!(lcm, lcm, "GPL", "");
ffi_export::export_symbol!(lcm_not_zero, lcm_not_zero, "GPL", "");
