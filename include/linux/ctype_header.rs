// SPDX-License-Identifier: GPL-2.0
//! Safe counterparts of the kernel's byte-classification macros and inlines.
//!
//! This private component is re-exported by the table-owning `lib/ctype.rs`
//! module and by `kernel::ctype`. Its enclosing module supplies a private
//! byte lookup into the single immutable table, without duplicating helpers.
//! These are the kernel's historical byte/Latin-1 rules, not Unicode or
//! locale-dependent classification. There is no special handling of C EOF.
//! Except for [`isdigit`], integer arguments are truncated to one byte, as
//! they are by the original macros or the C helpers' parameter conversions.

use super::ctype_mask;

#[cfg(not(CONFIG_RUST))]
use core::ffi::c_char;
#[cfg(CONFIG_RUST)]
use kernel::ffi::c_char;

/// Uppercase-letter classification bit.
pub const _U: u8 = 0x01;
/// Lowercase-letter classification bit.
pub const _L: u8 = 0x02;
/// Decimal-digit classification bit.
pub const _D: u8 = 0x04;
/// Control-character classification bit.
pub const _C: u8 = 0x08;
/// Punctuation classification bit.
pub const _P: u8 = 0x10;
/// Whitespace classification bit, including ASCII controls and Latin-1 NBSP.
pub const _S: u8 = 0x20;
/// Hexadecimal-letter classification bit, for `A`–`F` and `a`–`f`.
pub const _X: u8 = 0x40;
/// Printable-space classification bit, for space and Latin-1 NBSP.
pub const _SP: u8 = 0x80;

/// Return the classification mask after C's unsigned-byte conversion.
#[inline]
pub const fn __ismask(c: i32) -> u8 {
    ctype_mask(c as u8)
}

/// Whether the low byte is a letter or decimal digit.
#[inline]
pub const fn isalnum(c: i32) -> bool {
    __ismask(c) & (_U | _L | _D) != 0
}

/// Whether the low byte is a letter under the kernel's Latin-1 rules.
#[inline]
pub const fn isalpha(c: i32) -> bool {
    __ismask(c) & (_U | _L) != 0
}

/// Whether the low byte is an ASCII control character or DEL.
#[inline]
pub const fn iscntrl(c: i32) -> bool {
    __ismask(c) & _C != 0
}

/// Whether the low byte is printable and is not a space.
#[inline]
pub const fn isgraph(c: i32) -> bool {
    __ismask(c) & (_P | _U | _L | _D) != 0
}

/// Whether the low byte is a lowercase letter.
#[inline]
pub const fn islower(c: i32) -> bool {
    __ismask(c) & _L != 0
}

/// Whether the low byte is printable, including space and Latin-1 NBSP.
#[inline]
pub const fn isprint(c: i32) -> bool {
    __ismask(c) & (_P | _U | _L | _D | _SP) != 0
}

/// Whether the low byte is punctuation under the kernel's Latin-1 rules.
#[inline]
pub const fn ispunct(c: i32) -> bool {
    __ismask(c) & _P != 0
}

/// Whether the low byte is whitespace; NUL is never whitespace.
#[inline]
pub const fn isspace(c: i32) -> bool {
    __ismask(c) & _S != 0
}

/// Whether the low byte is an uppercase letter.
#[inline]
pub const fn isupper(c: i32) -> bool {
    __ismask(c) & _U != 0
}

/// Whether the low byte is an ASCII hexadecimal digit.
#[inline]
pub const fn isxdigit(c: i32) -> bool {
    __ismask(c) & (_D | _X) != 0
}

/// Whether the low byte is in the ASCII range.
#[inline]
pub const fn isascii(c: i32) -> bool {
    (c as u8) <= 0x7f
}

/// Truncate to a byte and clear its high bit, as the C macro does.
#[inline]
pub const fn toascii(c: i32) -> u8 {
    (c as u8) & 0x7f
}

/// Whether the full integer is an ASCII decimal digit, without truncation.
#[inline]
pub const fn isdigit(c: i32) -> bool {
    b'0' as i32 <= c && c <= b'9' as i32
}

/// Convert an uppercase byte by the original fixed Latin-1 offset.
#[inline]
pub const fn __tolower(c: u8) -> u8 {
    if isupper(c as i32) {
        c.wrapping_add(b'a' - b'A')
    } else {
        c
    }
}

/// Convert a lowercase byte by the original fixed Latin-1 offset.
///
/// Historical results are retained even where this is not a Unicode case
/// mapping: for example, `0xdf` becomes `0xbf`, and `0xff` becomes `0xdf`.
#[inline]
pub const fn __toupper(c: u8) -> u8 {
    if islower(c as i32) {
        c.wrapping_sub(b'a' - b'A')
    } else {
        c
    }
}

/// Truncate the integer to a byte and apply [`__tolower`].
#[inline]
pub const fn tolower(c: i32) -> u8 {
    __tolower(c as u8)
}

/// Truncate the integer to a byte and apply [`__toupper`].
#[inline]
pub const fn toupper(c: i32) -> u8 {
    __toupper(c as u8)
}

/// Unconditionally set the case bit, preserving the original C `char` type.
///
/// This internal shortcut is not a general case conversion. Native kernel
/// `char` is unsigned; standalone users retain their platform's C signedness.
#[inline]
pub const fn _tolower(c: c_char) -> c_char {
    c | 0x20
}

/// Whether the C-`char`-truncated input is an ASCII octal digit.
///
/// Signed and unsigned C `char` give the same result for this ASCII range.
/// Unlike [`isdigit`], this retains the C helper's implicit byte conversion.
#[inline]
pub const fn isodigit(c: i32) -> bool {
    let c = c as u8;
    b'0' <= c && c <= b'7'
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
