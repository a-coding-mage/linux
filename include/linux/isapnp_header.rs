/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  ISA Plug & Play support
 *  Copyright (c) by Jaroslav Kysela <perex@suse.cz>
 */

// Dependencies: linux/errno.h, linux/pnp.h, and linux/device-id/isapnp.h.

pub const fn isapnp_vendor(a: u8, b: u8, c: u8) -> u16 {
    ((((a as i32 - b'A' as i32 + 1) & 0x3f) << 2)
        | (((b as i32 - b'A' as i32 + 1) & 0x18) >> 3)
        | (((b as i32 - b'A' as i32 + 1) & 7) << 13)
        | (((c as i32 - b'A' as i32 + 1) & 0x1f) << 8)) as u16
}

pub const fn isapnp_device(x: u16) -> u16 {
    (((x & 0xf000) >> 8)
        | ((x & 0x0f00) >> 8)
        | ((x & 0x00f0) << 8)
        | ((x & 0x000f) << 8)) as u16
}

pub const fn isapnp_function(x: u16) -> u16 {
    isapnp_device(x)
}

pub const DEVICE_COUNT_COMPATIBLE: usize = 4;
pub const ISAPNP_CARD_DEVS: usize = 8;

#[repr(C)]
pub struct IsapnpCardIdDev {
    pub vendor: u16,
    pub function: u16,
}

#[repr(C)]
pub struct IsapnpCardId {
    pub driver_data: ::core::ffi::c_ulong,
    pub card_vendor: u16,
    pub card_device: u16,
    pub devs: [IsapnpCardIdDev; ISAPNP_CARD_DEVS],
}

// ISAPNP_CARD_ID(_va, _vb, _vc, _device):
// card_vendor = ISAPNP_VENDOR(_va, _vb, _vc), card_device = ISAPNP_DEVICE(_device)
// ISAPNP_CARD_END: card_vendor = 0, card_device = 0
// ISAPNP_DEVICE_ID(_va, _vb, _vc, _function):
// { vendor = ISAPNP_VENDOR(_va, _vb, _vc), function = ISAPNP_FUNCTION(_function) }
// ISAPNP_DEVICE_SINGLE and ISAPNP_DEVICE_SINGLE_END are designated initializers
// corresponding to the fields documented by their C definitions.

// The following declarations are active when CONFIG_ISAPNP (or its module form)
// is enabled; otherwise the inline fallbacks below apply.
#[cfg(any(feature = "CONFIG_ISAPNP", feature = "CONFIG_ISAPNP_MODULE"))]
extern "C" {
    pub fn isapnp_present() -> ::core::ffi::c_int;
    pub fn isapnp_cfg_begin(csn: ::core::ffi::c_int, device: ::core::ffi::c_int) -> ::core::ffi::c_int;
    pub fn isapnp_cfg_end() -> ::core::ffi::c_int;
    pub fn isapnp_read_byte(idx: u8) -> u8;
    pub fn isapnp_write_byte(idx: u8, val: u8);
}

#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub const fn isapnp_proc_init() -> ::core::ffi::c_int { 0 }

#[cfg(not(feature = "CONFIG_PROC_FS"))]
#[inline]
pub const fn isapnp_proc_done() -> ::core::ffi::c_int { 0 }

#[cfg(feature = "CONFIG_PROC_FS")]
extern "C" {
    pub fn isapnp_proc_init() -> ::core::ffi::c_int;
    pub fn isapnp_proc_done() -> ::core::ffi::c_int;
}

// compat: pnp_find_dev is supplied by linux/pnp.h.
extern "C" {
    pub fn pnp_find_dev(
        card: *mut crate::pnp_card,
        vendor: u16,
        function: u16,
        from: *mut crate::pnp_dev,
    ) -> *mut crate::pnp_dev;
}

// When CONFIG_ISAPNP is disabled, the C header instead provides these inline
// fallbacks.  ENODEV is supplied by linux/errno.h.
#[cfg(not(any(feature = "CONFIG_ISAPNP", feature = "CONFIG_ISAPNP_MODULE")))]
#[inline]
pub const fn isapnp_present() -> ::core::ffi::c_int { 0 }

#[cfg(not(any(feature = "CONFIG_ISAPNP", feature = "CONFIG_ISAPNP_MODULE")))]
#[inline]
pub const fn isapnp_cfg_begin(_csn: ::core::ffi::c_int, _device: ::core::ffi::c_int) -> ::core::ffi::c_int { -crate::ENODEV }

#[cfg(not(any(feature = "CONFIG_ISAPNP", feature = "CONFIG_ISAPNP_MODULE")))]
#[inline]
pub const fn isapnp_cfg_end() -> ::core::ffi::c_int { -crate::ENODEV }

#[cfg(not(any(feature = "CONFIG_ISAPNP", feature = "CONFIG_ISAPNP_MODULE")))]
#[inline]
pub const fn isapnp_read_byte(_idx: u8) -> u8 { 0xff }

#[cfg(not(any(feature = "CONFIG_ISAPNP", feature = "CONFIG_ISAPNP_MODULE")))]
#[inline]
pub fn isapnp_write_byte(_idx: u8, _val: u8) {}

#[cfg(not(any(feature = "CONFIG_ISAPNP", feature = "CONFIG_ISAPNP_MODULE")))]
#[inline]
pub fn pnp_find_dev(
    _card: *mut crate::pnp_card,
    _vendor: u16,
    _function: u16,
    _from: *mut crate::pnp_dev,
) -> *mut crate::pnp_dev { ::core::ptr::null_mut() }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
