// SPDX-License-Identifier: GPL-2.0
//! Native C ABI owner for integer powers and square roots.
//!
//! Pure Rust consumers import `kernel::math`; only this crate defines the
//! unmangled C entry points and their native export metadata.

#[path = "../../rust/ffi_export.rs"]
mod ffi_export;

// The canonical functions are public for pure consumers, but this C ABI crate
// deliberately keeps its imports private because the wrappers reuse their names.
#[allow(unreachable_pub)]
#[path = "int_pow.rs"]
mod power;
#[allow(unreachable_pub)]
#[path = "int_sqrt.rs"]
mod square_root;

// This additional width-specific helper remains a normal Rust symbol.
pub use square_root::int_sqrt32;

/// Returns `base` raised to `exp` with unsigned 64-bit wrapping arithmetic.
#[no_mangle]
pub extern "C" fn int_pow(base: u64, exp: u32) -> u64 {
    power::int_pow(base, exp)
}

/// Returns the floor of the square root with C's native `unsigned long` ABI.
#[no_mangle]
pub extern "C" fn int_sqrt(x: usize) -> usize {
    square_root::int_sqrt(x)
}

/// Returns a full-width square root through the 32-bit kernel's C entry point.
#[cfg(target_pointer_width = "32")]
#[no_mangle]
pub extern "C" fn int_sqrt64(x: u64) -> u32 {
    square_root::int_sqrt64(x)
}

// On 64-bit kernels the C header supplies its own inline function. Keep the
// shared safe helper available here without creating an extra C ABI symbol.
#[cfg(not(target_pointer_width = "32"))]
pub use square_root::int_sqrt64;

ffi_export::export_symbol!(int_pow, int_pow, "GPL", "");
ffi_export::export_symbol!(int_sqrt, int_sqrt, "", "");
#[cfg(target_pointer_width = "32")]
ffi_export::export_symbol!(int_sqrt64, int_sqrt64, "", "");
