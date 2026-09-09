/* SPDX-License-Identifier: GPL-2.0 */

// NOTE! This ctype does not handle EOF like the standard C library is
// required to.

pub const _U: u8 = 0x01; // upper
pub const _L: u8 = 0x02; // lower
pub const _D: u8 = 0x04; // digit
pub const _C: u8 = 0x08; // cntrl
pub const _P: u8 = 0x10; // punct
pub const _S: u8 = 0x20; // white space (space/lf/tab)
pub const _X: u8 = 0x40; // hex digit
pub const _SP: u8 = 0x80; // hard space (0x20)

extern "C" {
    pub static _ctype: [u8; 256];
}

#[macro_export]
macro_rules! __ismask {
    ($x:expr) => {
        unsafe { $crate::_ctype[($x as u8) as usize] }
    };
}

#[macro_export]
macro_rules! isalnum { ($c:expr) => { ($crate::__ismask!($c) & ($crate::_U | $crate::_L | $crate::_D)) != 0 }; }
#[macro_export]
macro_rules! isalpha { ($c:expr) => { ($crate::__ismask!($c) & ($crate::_U | $crate::_L)) != 0 }; }
#[macro_export]
macro_rules! iscntrl { ($c:expr) => { ($crate::__ismask!($c) & $crate::_C) != 0 }; }
#[macro_export]
macro_rules! isgraph { ($c:expr) => { ($crate::__ismask!($c) & ($crate::_P | $crate::_U | $crate::_L | $crate::_D)) != 0 }; }
#[macro_export]
macro_rules! islower { ($c:expr) => { ($crate::__ismask!($c) & $crate::_L) != 0 }; }
#[macro_export]
macro_rules! isprint { ($c:expr) => { ($crate::__ismask!($c) & ($crate::_P | $crate::_U | $crate::_L | $crate::_D | $crate::_SP)) != 0 }; }
#[macro_export]
macro_rules! ispunct { ($c:expr) => { ($crate::__ismask!($c) & $crate::_P) != 0 }; }
// Note: isspace() must return false for %NUL-terminator
#[macro_export]
macro_rules! isspace { ($c:expr) => { ($crate::__ismask!($c) & $crate::_S) != 0 }; }
#[macro_export]
macro_rules! isupper { ($c:expr) => { ($crate::__ismask!($c) & $crate::_U) != 0 }; }
#[macro_export]
macro_rules! isxdigit { ($c:expr) => { ($crate::__ismask!($c) & ($crate::_D | $crate::_X)) != 0 }; }

#[macro_export]
macro_rules! isascii { ($c:expr) => { (($c as u8) <= 0x7f) }; }
#[macro_export]
macro_rules! toascii { ($c:expr) => { (($c as u8) & 0x7f) }; }

pub unsafe fn isdigit(c: i32) -> i32 {
    if b'0' as i32 <= c && c <= b'9' as i32 { 1 } else { 0 }
}

pub unsafe fn __tolower(mut c: u8) -> u8 {
    if isupper!(c) { c = c.wrapping_sub(b'A'.wrapping_sub(b'a')); }
    c
}

pub unsafe fn __toupper(mut c: u8) -> u8 {
    if islower!(c) { c = c.wrapping_sub(b'a'.wrapping_sub(b'A')); }
    c
}

#[macro_export]
macro_rules! tolower { ($c:expr) => { $crate::__tolower($c) }; }
#[macro_export]
macro_rules! toupper { ($c:expr) => { $crate::__toupper($c) }; }

// Fast implementation of tolower() for internal usage. Do not use in your
// code.
pub unsafe fn _tolower(c: i8) -> i8 { c | 0x20 }

// Fast check for octal digit
pub unsafe fn isodigit(c: i8) -> i32 {
    if c >= b'0' as i8 && c <= b'7' as i8 { 1 } else { 0 }
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
