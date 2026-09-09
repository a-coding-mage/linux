/*
 * This file is subject to the terms and conditions of the GNU General Public
 * License.  See the file "COPYING" in the main directory of this archive
 * for more details.
 *
 * Copyright (C) 2003 Ladislav Michl
 * Copyright (C) 2004 Ralf Baechle
 */

// The original C header guard was: __ASM_MACH_IP32_MANGLE_PORT_H

#[inline(always)]
pub const fn __swizzle_addr_b(port: usize) -> usize {
    port ^ 3
}

#[inline(always)]
pub const fn __swizzle_addr_w(port: usize) -> usize {
    port ^ 2
}

#[inline(always)]
pub const fn __swizzle_addr_l<T>(port: T) -> T {
    port
}

#[inline(always)]
pub const fn __swizzle_addr_q<T>(port: T) -> T {
    port
}

#[macro_export]
macro_rules! ioswabb {
    ($a:expr, $x:expr) => {{ $x }};
}

#[macro_export]
macro_rules! __mem_ioswabb {
    ($a:expr, $x:expr) => {{ $x }};
}

#[macro_export]
macro_rules! ioswabw {
    ($a:expr, $x:expr) => {{ $x }};
}

#[macro_export]
macro_rules! __mem_ioswabw {
    ($a:expr, $x:expr) => {{ cpu_to_le16($x) as u16 }};
}

#[macro_export]
macro_rules! ioswabl {
    ($a:expr, $x:expr) => {{ $x }};
}

#[macro_export]
macro_rules! __mem_ioswabl {
    ($a:expr, $x:expr) => {{ cpu_to_le32($x) as u32 }};
}

#[macro_export]
macro_rules! ioswabq {
    ($a:expr, $x:expr) => {{ $x }};
}

#[macro_export]
macro_rules! __mem_ioswabq {
    ($a:expr, $x:expr) => {{ cpu_to_le64($x) as u64 }};
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
