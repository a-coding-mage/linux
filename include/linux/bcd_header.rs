// SPDX-License-Identifier: GPL-2.0
//! Pure binary-coded decimal helpers shared by the implementation and callers.
//!
//! C's `bcd2bin` and `bin2bcd` macros select different expressions for constant
//! and nonconstant arguments. Rust exposes those operations separately: the
//! unprefixed conversions reproduce the exported C functions, whereas the
//! `const_` conversions reproduce the C macros with an unsigned 32-bit argument.
//! All helpers are usable in constant expressions; their names specify their
//! arithmetic semantics, not whether a call is evaluated at compile time.
//!
//! Conversions do not validate their input. In particular, do not replace the
//! runtime-compatible [`bin2bcd`] algorithm with [`const_bin2bcd`]: their results
//! can differ outside the ordinary two-digit decimal range.

/// Converts a packed byte with the semantics of C's exported `_bcd2bin`.
///
/// Both nibbles participate even when they are not valid decimal digits. For
/// example, `0xff` converts to 165; use [`bcd_is_valid`] for validation.
#[inline]
pub const fn bcd2bin(val: u8) -> u32 {
    const_bcd2bin(val as u32)
}

/// Converts an integer with the semantics of C's exported `_bin2bcd`.
///
/// This retains the original unsigned, wrapping multiply-and-shift algorithm
/// and byte-sized result for every input, not just values in `0..=99`.
/// In particular, `bin2bcd(1024)` is `0xfa`, whereas [`const_bin2bcd`] returns
/// the full-width value `0x664` for that input.
#[inline]
pub const fn bin2bcd(val: u32) -> u8 {
    let tens = val.wrapping_mul(103) >> 10;
    ((tens << 4) | val.wrapping_sub(tens.wrapping_mul(10))) as u8
}

/// Evaluates C's `const_bcd2bin` expression for an unsigned 32-bit argument.
///
/// The input is not truncated to a byte: `const_bcd2bin(0x100)` is 160.
/// Arithmetic uses C's unsigned wrapping rules, and invalid digits are not
/// rejected.
#[inline]
pub const fn const_bcd2bin(val: u32) -> u32 {
    (val & 0x0f).wrapping_add((val >> 4).wrapping_mul(10))
}

/// Evaluates C's `const_bin2bcd` expression for an unsigned 32-bit argument.
///
/// Division and remainder use the entire argument. The shift and addition
/// wrap at 32 bits, and the result is not truncated to a byte. For example,
/// `const_bin2bcd(1024)` is `0x664`, not `0x64` or [`bin2bcd`]'s `0xfa`.
#[inline]
pub const fn const_bin2bcd(val: u32) -> u32 {
    ((val / 10) << 4).wrapping_add(val % 10)
}

/// Reports whether the complete argument encodes two decimal BCD digits.
///
/// No byte truncation is performed, so any nonzero bits above the low byte
/// make the argument invalid.
#[inline]
pub const fn bcd_is_valid(val: u32) -> bool {
    const_bcd_is_valid(val)
}

/// Evaluates C's `const_bcd_is_valid` expression for an unsigned argument.
///
/// This is identical to [`bcd_is_valid`], including its full-width check.
#[inline]
pub const fn const_bcd_is_valid(val: u32) -> bool {
    (val & 0x0f) < 10 && (val >> 4) < 10
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
