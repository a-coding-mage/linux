/* SPDX-License-Identifier: GPL-2.0 */
/*
 *    EBCDIC -> ASCII, ASCII -> EBCDIC conversion routines.
 *
 *  S390 version
 *    Copyright IBM Corp. 1999
 *    Author(s): Martin Schwidefsky <schwidefsky@de.ibm.com>
 */

// Translated from the Linux s390 EBCDIC header.  C's __u8 is represented by
// Rust's u8; the externally supplied conversion tables retain their names and
// mutable global layout.

unsafe extern "C" {
    pub static mut _ascebc_500: [u8; 256]; // ASCII -> EBCDIC 500 conversion table
    pub static mut _ebcasc_500: [u8; 256]; // EBCDIC 500 -> ASCII conversion table
    pub static mut _ascebc: [u8; 256]; // ASCII -> EBCDIC conversion table
    pub static mut _ebcasc: [u8; 256]; // EBCDIC -> ASCII conversion table
    pub static mut _ebc_tolower: [u8; 256]; // EBCDIC -> lowercase
    pub static mut _ebc_toupper: [u8; 256]; // EBCDIC -> uppercase
}

/// Convert `nr` bytes at `addr` using the supplied 256-byte code page.
///
/// The original implementation uses s390 `tr` instructions through inline
/// assembly.  This preserves the same byte-wise volatile memory behavior.
#[inline]
pub unsafe fn codepage_convert(codepage: *const u8, addr: *mut core::ffi::c_char, nr: usize) {
    let mut i = 0usize;
    while i < nr {
        let p = addr.add(i);
        let value = core::ptr::read_volatile(p as *const u8);
        let converted = core::ptr::read_volatile(codepage.add(value as usize));
        core::ptr::write_volatile(p as *mut u8, converted);
        i = i.wrapping_add(1);
    }
}

#[macro_export]
macro_rules! ASCEBC {
    ($addr:expr, $nr:expr) => {
        unsafe { $crate::codepage_convert(core::ptr::addr_of!($crate::_ascebc) as *const u8, $addr, $nr) }
    };
}

#[macro_export]
macro_rules! EBCASC {
    ($addr:expr, $nr:expr) => {
        unsafe { $crate::codepage_convert(core::ptr::addr_of!($crate::_ebcasc) as *const u8, $addr, $nr) }
    };
}

#[macro_export]
macro_rules! ASCEBC_500 {
    ($addr:expr, $nr:expr) => {
        unsafe { $crate::codepage_convert(core::ptr::addr_of!($crate::_ascebc_500) as *const u8, $addr, $nr) }
    };
}

#[macro_export]
macro_rules! EBCASC_500 {
    ($addr:expr, $nr:expr) => {
        unsafe { $crate::codepage_convert(core::ptr::addr_of!($crate::_ebcasc_500) as *const u8, $addr, $nr) }
    };
}

#[macro_export]
macro_rules! EBC_TOLOWER {
    ($addr:expr, $nr:expr) => {
        unsafe { $crate::codepage_convert(core::ptr::addr_of!($crate::_ebc_tolower) as *const u8, $addr, $nr) }
    };
}

#[macro_export]
macro_rules! EBC_TOUPPER {
    ($addr:expr, $nr:expr) => {
        unsafe { $crate::codepage_convert(core::ptr::addr_of!($crate::_ebc_toupper) as *const u8, $addr, $nr) }
    };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
