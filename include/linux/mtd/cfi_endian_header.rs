/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright © 2001-2010 David Woodhouse <dwmw2@infradead.org>
 */

// Dependency supplied by the target architecture's byte-order definitions.

pub const CFI_HOST_ENDIAN: i32 = 1;
pub const CFI_LITTLE_ENDIAN: i32 = 2;
pub const CFI_BIG_ENDIAN: i32 = 3;

// CONFIG_MTD_CFI_ADV_OPTIONS / CONFIG_MTD_CFI_NOSWAP and the byte-swap
// configuration are build-time conditions from the original header.
#[cfg(any(not(feature = "mtd_cfi_adv_options"), feature = "mtd_cfi_noswap"))]
pub const CFI_DEFAULT_ENDIAN: i32 = CFI_HOST_ENDIAN;
#[cfg(all(feature = "mtd_cfi_adv_options", not(feature = "mtd_cfi_noswap"), feature = "mtd_cfi_le_byte_swap"))]
pub const CFI_DEFAULT_ENDIAN: i32 = CFI_LITTLE_ENDIAN;
#[cfg(all(feature = "mtd_cfi_adv_options", not(feature = "mtd_cfi_noswap"), not(feature = "mtd_cfi_le_byte_swap"), feature = "mtd_cfi_be_byte_swap"))]
pub const CFI_DEFAULT_ENDIAN: i32 = CFI_BIG_ENDIAN;

#[inline]
pub const fn cfi_default(s: i32) -> i32 {
    if s != 0 { s } else { CFI_DEFAULT_ENDIAN }
}

#[inline]
pub const fn cfi_be(s: i32) -> bool { cfi_default(s) == CFI_BIG_ENDIAN }

#[inline]
pub const fn cfi_le(s: i32) -> bool { cfi_default(s) == CFI_LITTLE_ENDIAN }

#[inline]
pub const fn cfi_host(s: i32) -> bool { cfi_default(s) == CFI_HOST_ENDIAN }

#[macro_export]
macro_rules! cpu_to_cfi8 { ($map:expr, $x:expr) => { $x }; }
#[macro_export]
macro_rules! cfi8_to_cpu { ($map:expr, $x:expr) => { $x }; }

#[macro_export]
macro_rules! cpu_to_cfi16 { ($map:expr, $x:expr) => { $crate::_cpu_to_cfi!(16, ($map).swap, $x) }; }
#[macro_export]
macro_rules! cpu_to_cfi32 { ($map:expr, $x:expr) => { $crate::_cpu_to_cfi!(32, ($map).swap, $x) }; }
#[macro_export]
macro_rules! cpu_to_cfi64 { ($map:expr, $x:expr) => { $crate::_cpu_to_cfi!(64, ($map).swap, $x) }; }
#[macro_export]
macro_rules! cfi16_to_cpu { ($map:expr, $x:expr) => { $crate::_cfi_to_cpu!(16, ($map).swap, $x) }; }
#[macro_export]
macro_rules! cfi32_to_cpu { ($map:expr, $x:expr) => { $crate::_cfi_to_cpu!(32, ($map).swap, $x) }; }
#[macro_export]
macro_rules! cfi64_to_cpu { ($map:expr, $x:expr) => { $crate::_cfi_to_cpu!(64, ($map).swap, $x) }; }

#[macro_export]
macro_rules! _cpu_to_cfi {
    ($w:literal, $s:expr, $x:expr) => {
        if $crate::cfi_host($s) { $x } else { $crate::_swap_to_cfi!($w, $s, $x) }
    };
}
#[macro_export]
macro_rules! _cfi_to_cpu {
    ($w:literal, $s:expr, $x:expr) => {
        if $crate::cfi_host($s) { $x } else { $crate::_swap_to_cpu!($w, $s, $x) }
    };
}
#[macro_export]
macro_rules! _swap_to_cfi {
    (16, $s:expr, $x:expr) => { if $crate::cfi_be($s) { cpu_to_be16($x) } else { cpu_to_le16($x) } };
    (32, $s:expr, $x:expr) => { if $crate::cfi_be($s) { cpu_to_be32($x) } else { cpu_to_le32($x) } };
    (64, $s:expr, $x:expr) => { if $crate::cfi_be($s) { cpu_to_be64($x) } else { cpu_to_le64($x) } };
}
#[macro_export]
macro_rules! _swap_to_cpu {
    (16, $s:expr, $x:expr) => { if $crate::cfi_be($s) { be16_to_cpu($x) } else { le16_to_cpu($x) } };
    (32, $s:expr, $x:expr) => { if $crate::cfi_be($s) { be32_to_cpu($x) } else { le32_to_cpu($x) } };
    (64, $s:expr, $x:expr) => { if $crate::cfi_be($s) { be64_to_cpu($x) } else { le64_to_cpu($x) } };
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
