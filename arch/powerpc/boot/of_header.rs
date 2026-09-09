/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the translated `swab.h` equivalent.

pub type Phandle = *mut core::ffi::c_void;
pub type Ihandle = u32;

unsafe extern "C" {
    pub fn of_init(promptr: *mut core::ffi::c_void);
    pub fn of_call_prom(
        service: *const core::ffi::c_char,
        nargs: core::ffi::c_int,
        nret: core::ffi::c_int,
        ...,
    ) -> core::ffi::c_int;
    pub fn of_claim(
        virt: core::ffi::c_ulong,
        size: core::ffi::c_ulong,
        align: core::ffi::c_ulong,
    ) -> core::ffi::c_uint;
    pub fn of_vmlinux_alloc(size: core::ffi::c_ulong) -> *mut core::ffi::c_void;
    pub fn of_exit();
    pub fn of_finddevice(name: *const core::ffi::c_char) -> *mut core::ffi::c_void;
    pub fn of_getprop(
        phandle: *const core::ffi::c_void,
        name: *const core::ffi::c_char,
        buf: *mut core::ffi::c_void,
        buflen: core::ffi::c_int,
    ) -> core::ffi::c_int;
    pub fn of_setprop(
        phandle: *const core::ffi::c_void,
        name: *const core::ffi::c_char,
        buf: *const core::ffi::c_void,
        buflen: core::ffi::c_int,
    ) -> core::ffi::c_int;

    /* Console functions */
    pub fn of_console_init();
}

pub type __be16 = u16;
pub type __be32 = u32;
pub type __be64 = u64;

#[cfg(target_endian = "little")]
#[inline]
pub const fn cpu_to_be16(x: u16) -> u16 { x.swap_bytes() }
#[cfg(target_endian = "little")]
#[inline]
pub const fn be16_to_cpu(x: u16) -> u16 { x.swap_bytes() }
#[cfg(target_endian = "little")]
#[inline]
pub const fn cpu_to_be32(x: u32) -> u32 { x.swap_bytes() }
#[cfg(target_endian = "little")]
#[inline]
pub const fn be32_to_cpu(x: u32) -> u32 { x.swap_bytes() }
#[cfg(target_endian = "little")]
#[inline]
pub const fn cpu_to_be64(x: u64) -> u64 { x.swap_bytes() }
#[cfg(target_endian = "little")]
#[inline]
pub const fn be64_to_cpu(x: u64) -> u64 { x.swap_bytes() }

#[cfg(target_endian = "big")]
#[inline]
pub const fn cpu_to_be16(x: u16) -> u16 { x }
#[cfg(target_endian = "big")]
#[inline]
pub const fn be16_to_cpu(x: u16) -> u16 { x }
#[cfg(target_endian = "big")]
#[inline]
pub const fn cpu_to_be32(x: u32) -> u32 { x }
#[cfg(target_endian = "big")]
#[inline]
pub const fn be32_to_cpu(x: u32) -> u32 { x }
#[cfg(target_endian = "big")]
#[inline]
pub const fn cpu_to_be64(x: u64) -> u64 { x }
#[cfg(target_endian = "big")]
#[inline]
pub const fn be64_to_cpu(x: u64) -> u64 { x }

pub const PROM_ERROR: u32 = (-1i32) as u32;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
