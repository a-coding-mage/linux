/*
 * Interface to the TURBOchannel related routines.
 *
 * Copyright (c) 1998 Harald Koerfgen
 * Copyright (c) 2005 James Simmons
 * Copyright (c) 2006 Maciej W. Rozycki
 *
 * Based on: "TURBOchannel Firmware Specification", EK-TCAAD-FS-004,
 * from Digital Equipment Corporation.
 *
 * This file is subject to the terms and conditions of the GNU General
 * Public License. See the file "COPYING" in the main directory of this
 * archive for more details.
 */

pub const TC_OLDCARD: u32 = 0x3c0000;
pub const TC_NEWCARD: u32 = 0x000000;

pub const TC_ROM_WIDTH: u32 = 0x3e0;
pub const TC_ROM_STRIDE: u32 = 0x3e4;
pub const TC_ROM_SIZE: u32 = 0x3e8;
pub const TC_SLOT_SIZE: u32 = 0x3ec;
pub const TC_PATTERN0: u32 = 0x3f0;
pub const TC_PATTERN1: u32 = 0x3f4;
pub const TC_PATTERN2: u32 = 0x3f8;
pub const TC_PATTERN3: u32 = 0x3fc;
pub const TC_FIRM_VER: u32 = 0x400;
pub const TC_VENDOR: u32 = 0x420;
pub const TC_MODULE: u32 = 0x440;
pub const TC_FIRM_TYPE: u32 = 0x460;
pub const TC_FLAGS: u32 = 0x470;
pub const TC_ROM_OBJECTS: u32 = 0x480;

#[repr(C)]
pub struct tcinfo {
    pub revision: i32,
    pub clk_period: i32,
    pub slot_size: i32,
    pub io_timeout: i32,
    pub dma_range: i32,
    pub max_dma_burst: i32,
    pub parity: i32,
    pub reserved: [i32; 4],
}

#[repr(C)]
pub struct tc_bus {
    pub devices: list_head,
    pub resource: [resource; 2],
    pub dev: device,
    pub name: [core::ffi::c_char; 13],
    pub slot_base: resource_size_t,
    pub ext_slot_base: resource_size_t,
    pub ext_slot_size: resource_size_t,
    pub num_tcslots: core::ffi::c_int,
    pub info: tcinfo,
}

#[repr(C)]
pub struct tc_dev {
    pub node: list_head,
    pub bus: *mut tc_bus,
    pub driver: *mut tc_driver,
    pub dev: device,
    pub resource: resource,
    pub dma_mask: u64,
    pub vendor: [core::ffi::c_char; 9],
    pub name: [core::ffi::c_char; 9],
    pub firmware: [core::ffi::c_char; 9],
    pub interrupt: core::ffi::c_int,
    pub slot: core::ffi::c_int,
}

#[macro_export]
macro_rules! to_tc_dev {
    ($n:expr) => {
        container_of!($n, tc_dev, dev)
    };
}

#[repr(C)]
pub struct tc_device_id {
    pub vendor: [core::ffi::c_char; 9],
    pub name: [core::ffi::c_char; 9],
}

#[repr(C)]
pub struct tc_driver {
    pub node: list_head,
    pub id_table: *const tc_device_id,
    pub driver: device_driver,
}

#[macro_export]
macro_rules! to_tc_driver {
    ($drv:expr) => {
        container_of_const!($drv, tc_driver, driver)
    };
}

#[inline]
pub unsafe fn tc_get_speed(tbus: *mut tc_bus) -> core::ffi::c_ulong {
    100000u64
        .wrapping_mul(10000u64 / (*tbus).info.clk_period as u64)
        as core::ffi::c_ulong
}

// Under CONFIG_TC these are provided by the TURBOchannel bus implementation.
#[cfg(CONFIG_TC)]
extern "C" {
    pub static tc_bus_type: bus_type;
    pub fn tc_register_driver(tdrv: *mut tc_driver) -> core::ffi::c_int;
    pub fn tc_unregister_driver(tdrv: *mut tc_driver);
}

// !CONFIG_TC: registration is a no-op.
#[cfg(not(CONFIG_TC))]
#[inline]
pub unsafe fn tc_register_driver(_tdrv: *mut tc_driver) -> core::ffi::c_int {
    0
}

#[cfg(not(CONFIG_TC))]
#[inline]
pub unsafe fn tc_unregister_driver(_tdrv: *mut tc_driver) {}

extern "C" {
    pub fn tc_preadb(valp: *mut u8, addr: *mut core::ffi::c_void) -> core::ffi::c_int;
    pub fn tc_bus_get_info(tbus: *mut tc_bus) -> core::ffi::c_int;
    pub fn tc_device_get_irq(tdev: *mut tc_dev);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
