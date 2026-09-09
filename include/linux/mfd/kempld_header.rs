/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Kontron PLD driver definitions
 *
 * Copyright (c) 2010-2012 Kontron Europe GmbH
 * Author: Michael Brunner <michael.brunner@kontron.com>
 */

/* kempld register definitions */
pub const KEMPLD_IOINDEX: u16 = 0xa80;
pub const KEMPLD_IODATA: u16 = 0xa81;
pub const KEMPLD_MUTEX_KEY: u8 = 0x80;
pub const KEMPLD_VERSION: u8 = 0x00;
pub const KEMPLD_VERSION_LSB: u8 = 0x00;
pub const KEMPLD_VERSION_MSB: u8 = 0x01;
#[inline]
pub const fn KEMPLD_VERSION_GET_MINOR(x: u16) -> u16 { x & 0x1f }
#[inline]
pub const fn KEMPLD_VERSION_GET_MAJOR(x: u16) -> u16 { (x >> 5) & 0x1f }
#[inline]
pub const fn KEMPLD_VERSION_GET_NUMBER(x: u16) -> u16 { (x >> 10) & 0xf }
#[inline]
pub const fn KEMPLD_VERSION_GET_TYPE(x: u16) -> u16 { (x >> 14) & 0x3 }
pub const KEMPLD_BUILDNR: u8 = 0x02;
pub const KEMPLD_BUILDNR_LSB: u8 = 0x02;
pub const KEMPLD_BUILDNR_MSB: u8 = 0x03;
pub const KEMPLD_FEATURE: u8 = 0x04;
pub const KEMPLD_FEATURE_LSB: u8 = 0x04;
pub const KEMPLD_FEATURE_MSB: u8 = 0x05;
pub const KEMPLD_FEATURE_BIT_I2C: u32 = 1 << 0;
pub const KEMPLD_FEATURE_BIT_WATCHDOG: u32 = 1 << 1;
pub const KEMPLD_FEATURE_BIT_GPIO: u32 = 1 << 2;
pub const KEMPLD_FEATURE_MASK_UART: u32 = 7 << 3;
pub const KEMPLD_FEATURE_BIT_NMI: u32 = 1 << 8;
pub const KEMPLD_FEATURE_BIT_SMI: u32 = 1 << 9;
pub const KEMPLD_FEATURE_BIT_SCI: u32 = 1 << 10;
pub const KEMPLD_SPEC: u8 = 0x06;
#[inline]
pub const fn KEMPLD_SPEC_GET_MINOR(x: u8) -> u8 { x & 0x0f }
#[inline]
pub const fn KEMPLD_SPEC_GET_MAJOR(x: u8) -> u8 { (x >> 4) & 0x0f }
pub const KEMPLD_IRQ_GPIO: u8 = 0x35;
pub const KEMPLD_IRQ_GPIO_MASK: u8 = 0x0f;
pub const KEMPLD_IRQ_I2C: u8 = 0x36;
pub const KEMPLD_CFG: u8 = 0x37;
pub const KEMPLD_CFG_GPIO_I2C_MUX: u8 = 1 << 0;
pub const KEMPLD_CFG_BIOS_WP: u8 = 1 << 7;

pub const KEMPLD_CLK: u32 = 33333333;
pub const KEMPLD_TYPE_RELEASE: u32 = 0x0;
pub const KEMPLD_TYPE_DEBUG: u32 = 0x1;
pub const KEMPLD_TYPE_CUSTOM: u32 = 0x2;
pub const KEMPLD_VERSION_LEN: usize = 10;

#[repr(C)]
pub struct kempld_info {
    pub major: ::core::ffi::c_uint,
    pub minor: ::core::ffi::c_uint,
    pub buildnr: ::core::ffi::c_uint,
    pub number: ::core::ffi::c_uint,
    pub type_: ::core::ffi::c_uint,
    pub spec_major: ::core::ffi::c_uint,
    pub spec_minor: ::core::ffi::c_uint,
    pub version: [::core::ffi::c_char; KEMPLD_VERSION_LEN],
}

#[repr(C)]
pub struct kempld_device_data {
    pub io_base: *mut ::core::ffi::c_void,
    pub io_index: *mut ::core::ffi::c_void,
    pub io_data: *mut ::core::ffi::c_void,
    pub pld_clock: u32,
    pub feature_mask: u32,
    pub dev: *mut device,
    pub info: kempld_info,
    pub lock: mutex,
}

#[repr(C)]
pub struct kempld_platform_data {
    pub pld_clock: u32,
    pub gpio_base: ::core::ffi::c_int,
    pub ioresource: *mut resource,
    pub get_hardware_mutex: Option<unsafe extern "C" fn(*mut kempld_device_data)>,
    pub release_hardware_mutex: Option<unsafe extern "C" fn(*mut kempld_device_data)>,
    pub get_info: Option<unsafe extern "C" fn(*mut kempld_device_data) -> ::core::ffi::c_int>,
    pub register_cells: Option<unsafe extern "C" fn(*mut kempld_device_data) -> ::core::ffi::c_int>,
}

pub enum device {}
pub enum resource {}
pub enum mutex {}

extern "C" {
    pub fn kempld_get_mutex(pld: *mut kempld_device_data);
    pub fn kempld_release_mutex(pld: *mut kempld_device_data);
    pub fn kempld_read8(pld: *mut kempld_device_data, index: u8) -> u8;
    pub fn kempld_write8(pld: *mut kempld_device_data, index: u8, data: u8);
    pub fn kempld_read16(pld: *mut kempld_device_data, index: u8) -> u16;
    pub fn kempld_write16(pld: *mut kempld_device_data, index: u8, data: u16);
    pub fn kempld_read32(pld: *mut kempld_device_data, index: u8) -> u32;
    pub fn kempld_write32(pld: *mut kempld_device_data, index: u8, data: u32);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
