// SPDX-License-Identifier: GPL-2.0
//
// CS35L41 ALSA HDA Property driver
//
// Copyright 2023 Cirrus Logic, Inc.
//
// Author: Stefan Binding <sbinding@opensource.cirrus.com>

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(dead_code)]
#![allow(improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_void};
use core::mem::size_of;
use core::ptr;

const MAX_AMPS: usize = 4;

const ENOENT: c_int = 2;
const ENODEV: c_int = 19;
const ENOMEM: c_int = 12;
const GFP_KERNEL: c_uint = 0;
const GPIOD_OUT_LOW: c_int = 0;
const GPIOD_OUT_HIGH: c_int = 1;
const SPI: c_int = 1;
const CONFIG_SPI: bool = true;

type u8 = core::ffi::c_uchar;

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_device {
    pub driver_gpios: *mut c_void,
}

#[repr(C)]
pub struct gpio_desc {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct fwnode_handle {
    _private: [u8; 0],
}

#[repr(C)]
pub struct acpi_gpio_params {
    pub crs_entry_index: c_uint,
    pub line_index: c_uint,
    pub active_low: bool,
}

#[repr(C)]
pub struct acpi_gpio_mapping {
    pub name: *const c_char,
    pub data: *const acpi_gpio_params,
    pub size: c_uint,
    pub quirks: c_uint,
}

#[repr(C)]
pub struct cs35l41_gpio_cfg {
    pub func: c_int,
    pub valid: bool,
}

#[repr(C)]
pub struct cs35l41_hw_cfg {
    pub spk_pos: u8,
    pub bst_type: c_int,
    pub bst_ind: c_int,
    pub bst_ipk: c_int,
    pub bst_cap: c_int,
    pub gpio1: cs35l41_gpio_cfg,
    pub gpio2: cs35l41_gpio_cfg,
    pub valid: bool,
}

#[repr(C)]
pub struct cs35l41_hda {
    pub dacpi: *mut acpi_device,
    pub hw_cfg: cs35l41_hw_cfg,
    pub acpi_subsystem_id: *const c_char,
    pub dev: *mut device,
    pub control_bus: c_int,
    pub index: c_int,
    pub reset_gpio: *mut gpio_desc,
    pub speaker_id: c_int,
    pub channel_index: c_int,
    pub cs_gpio: *mut gpio_desc,
}

extern "C" {
    static CS35L41_LEFT: u8;
    static CS35L41_RIGHT: u8;
    static CS35L41_INT_BOOST: c_int;
    static CS35L41_EXT_BOOST: c_int;
    static CS35L41_EXT_BOOST_NO_VSPK_SWITCH: c_int;
    static CS35L41_NOT_USED: c_int;
    static CS35l41_VSPK_SWITCH: c_int;
    static CS35L41_INTERRUPT: c_int;

    fn strcasecmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn strcmp(s1: *const c_char, s2: *const c_char) -> c_int;
    fn devm_kcalloc(dev: *mut device, n: usize, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn devm_kfree(dev: *mut device, p: *mut c_void);
    fn devm_acpi_dev_add_driver_gpios(
        dev: *mut device,
        mapping: *mut acpi_gpio_mapping,
    ) -> c_int;
    fn ACPI_COMPANION(dev: *mut device) -> *mut acpi_device;
    fn acpi_dev_has_props(adev: *mut acpi_device) -> bool;
    fn dev_err(dev: *mut device, fmt: *const c_char, ...);
    fn dev_info(dev: *mut device, fmt: *const c_char, ...);
    fn dev_warn(dev: *mut device, fmt: *const c_char, ...);
    fn to_spi_device(dev: *mut device) -> *mut spi_device;
    fn gpiod_get_index(
        dev: *mut device,
        con_id: *const c_char,
        idx: c_uint,
        flags: c_int,
    ) -> *mut gpio_desc;
    fn IS_ERR(ptr: *const c_void) -> bool;
    fn PTR_ERR(ptr: *const c_void) -> c_int;
    fn spi_set_csgpiod(spi: *mut spi_device, idx: c_uint, desc: *mut gpio_desc);
    fn gpiod_set_value_cansleep(desc: *mut gpio_desc, value: bool);
    fn gpiod_put(desc: *mut gpio_desc);
    fn spi_setup(spi: *mut spi_device) -> c_int;
    fn acpi_fwnode_handle(adev: *mut acpi_device) -> *mut fwnode_handle;
    fn fwnode_gpiod_get_index(
        fwnode: *mut fwnode_handle,
        con_id: *const c_char,
        index: c_uint,
        flags: c_int,
        label: *const c_char,
    ) -> *mut gpio_desc;
    fn cs35l41_get_speaker_id(
        physdev: *mut device,
        amp_index: c_int,
        num_amps: c_int,
        fixed_gpio_id: c_int,
    ) -> c_int;
    fn cs35l41_hda_parse_acpi(
        cs35l41: *mut cs35l41_hda,
        physdev: *mut device,
        id: c_int,
    ) -> c_int;
}

macro_rules! cstr {
    ($s:literal) => {
        concat!($s, "\0").as_ptr() as *const c_char
    };
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum cs35l41_boost_type {
    INTERNAL,
    EXTERNAL,
}

#[repr(C)]
#[derive(Copy, Clone)]
struct cs35l41_config {
    ssid: *const c_char,
    num_amps: c_int,
    boost_type: cs35l41_boost_type,
    channel: [u8; MAX_AMPS],
    reset_gpio_index: c_int,     /* -1 if no reset gpio */
    spkid_gpio_index: c_int,     /* -1 if no spkid gpio */
    cs_gpio_index: c_int, /* -1 if no cs gpio, or cs-gpios already exists, max num amps == 2 */
    boost_ind_nanohenry: c_int, /* Required if boost_type == Internal */
    boost_peak_milliamp: c_int, /* Required if boost_type == Internal */
    boost_cap_microfarad: c_int, /* Required if boost_type == Internal */
}

const unsafe fn ch(v: *const u8) -> u8 {
    *v
}

macro_rules! cfg {
    ($ssid:literal, $num:expr, $boost:ident, [$a:expr, $b:expr, $c:expr, $d:expr], $rst:expr, $spkid:expr, $cs:expr, $ind:expr, $ipk:expr, $cap:expr) => {
        cs35l41_config {
            ssid: cstr!($ssid),
            num_amps: $num,
            boost_type: cs35l41_boost_type::$boost,
            channel: [$a, $b, $c, $d],
            reset_gpio_index: $rst,
            spkid_gpio_index: $spkid,
            cs_gpio_index: $cs,
            boost_ind_nanohenry: $ind,
            boost_peak_milliamp: $ipk,
            boost_cap_microfarad: $cap,
        }
    };
}

static cs35l41_config_table: &[cs35l41_config] = unsafe { &[
    cfg!("10251826", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, -1, -1, 0, 0, 0),
    cfg!("1025182C", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, -1, -1, 0, 0, 0),
    cfg!("10251844", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, -1, -1, 0, 0, 0),
    cfg!("10280B27", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10280B28", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10280BEB", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, -1, 0, 0, 0, 0),
    cfg!("10280C4D", 4, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT)], 0, 1, -1, 1000, 4500, 24),
/*
 * Device 103C89C6 does have _DSD, however it is setup to use the wrong boost type.
 * We can override the _DSD to correct the boost type here.
 * Since this laptop has valid ACPI, we do not need to handle cs-gpios, since that already exists
 * in the ACPI. The Reset GPIO is also valid, so we can use the Reset defined in _DSD.
 */
    cfg!("103C89C6", 2, INTERNAL, [ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT), 0, 0], -1, -1, -1, 1000, 4500, 24),
    cfg!("103C8A28", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8A29", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8A2A", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8A2B", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8A2C", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8A2D", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8A2E", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8A30", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8A31", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8A6E", 4, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), ch(&CS35L41_RIGHT)], 0, -1, -1, 0, 0, 0),
/*
 * Device 103C8B63 has _DSD with valid reset-gpios and cs-gpios, however the
 * boost type is incorrectly set to Internal. Override to External Boost.
 */
    cfg!("103C8B63", 4, EXTERNAL, [ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT)], -1, -1, -1, 0, 0, 0),
    cfg!("103C8BB3", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BB4", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BDD", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BDE", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BDF", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BE0", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BE1", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BE2", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BE3", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BE5", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BE6", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BE7", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BE8", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8BE9", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8B3A", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8C15", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4000, 24),
    cfg!("103C8C16", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4000, 24),
    cfg!("103C8C17", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4000, 24),
    cfg!("103C8C4D", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8C4E", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8C4F", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8C50", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8C51", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8CDD", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4100, 24),
    cfg!("103C8CDE", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 3900, 24),
    cfg!("104312AF", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10431433", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4500, 24),
    cfg!("10431463", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4500, 24),
    cfg!("10431473", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, -1, 0, 1000, 4500, 24),
    cfg!("10431483", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, -1, 0, 1000, 4500, 24),
    cfg!("10431493", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("104314D3", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("104314E3", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4500, 24),
    cfg!("10431503", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4500, 24),
    cfg!("10431533", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4500, 24),
    cfg!("10431573", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10431663", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, -1, 0, 1000, 4500, 24),
    cfg!("10431683", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 0, 0, 0),
    cfg!("104316A3", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 0, 0, 0),
    cfg!("104316D3", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 0, 0, 0),
    cfg!("104316F3", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 0, 0, 0),
    cfg!("104317F3", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4500, 24),
    cfg!("10431863", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("104318D3", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 0, 0, 0),
    cfg!("10431A83", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4500, 24),
    cfg!("10431B93", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10431C9F", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10431CAF", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10431CCF", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10431CDF", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10431CEF", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10431D1F", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4500, 24),
    cfg!("10431DA2", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 0, 0, 0),
    cfg!("10431E02", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 0, 0, 0),
    cfg!("10431E12", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 0, 0, 0),
    cfg!("10431EE2", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, -1, -1, 0, 0, 0),
    cfg!("10431F12", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 1000, 4500, 24),
    cfg!("10431F1F", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, -1, 0, 0, 0, 0),
    cfg!("10431F62", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 0, 0, 0),
    cfg!("10433A20", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10433A30", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10433A40", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10433A50", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("10433A60", 2, INTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 1, 2, 0, 1000, 4500, 24),
    cfg!("17AA3865", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, -1, -1, 0, 0, 0),
    cfg!("17AA3866", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, -1, -1, 0, 0, 0),
    cfg!("17AA386E", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 2, -1, 0, 0, 0),
    cfg!("17AA386F", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, -1, -1, 0, 0, 0),
    cfg!("17AA3874", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, -1, -1, 0, 0, 0),
    cfg!("17AA3877", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, -1, -1, 0, 0, 0),
    cfg!("17AA3878", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, -1, -1, 0, 0, 0),
    cfg!("17AA38A9", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 2, -1, 0, 0, 0),
    cfg!("17AA38AB", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 2, -1, 0, 0, 0),
    cfg!("17AA38B4", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 0, 0, 0),
    cfg!("17AA38B5", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 0, 0, 0),
    cfg!("17AA38B6", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 0, 0, 0),
    cfg!("17AA38B7", 2, EXTERNAL, [ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), 0, 0], 0, 1, -1, 0, 0, 0),
    cfg!("17AA38C7", 4, INTERNAL, [ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT)], 0, 2, -1, 1000, 4500, 24),
    cfg!("17AA38C8", 4, INTERNAL, [ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT)], 0, 2, -1, 1000, 4500, 24),
    cfg!("17AA38F9", 2, EXTERNAL, [ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT), 0, 0], 0, 2, -1, 0, 0, 0),
    cfg!("17AA38FA", 2, EXTERNAL, [ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT), 0, 0], 0, 2, -1, 0, 0, 0),
    cfg!("17AA3929", 4, INTERNAL, [ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT)], 0, 2, -1, 1000, 4500, 24),
    cfg!("17AA392B", 4, INTERNAL, [ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT), ch(&CS35L41_RIGHT), ch(&CS35L41_LEFT)], 0, 2, -1, 1000, 4500, 24),
    cs35l41_config {
        ssid: ptr::null(),
        num_amps: 0,
        boost_type: cs35l41_boost_type::INTERNAL,
        channel: [0; MAX_AMPS],
        reset_gpio_index: 0,
        spkid_gpio_index: 0,
        cs_gpio_index: 0,
        boost_ind_nanohenry: 0,
        boost_peak_milliamp: 0,
        boost_cap_microfarad: 0,
    },
] };

unsafe fn cs35l41_add_gpios(
    cs35l41: *mut cs35l41_hda,
    physdev: *mut device,
    reset_gpio: c_int,
    spkid_gpio: c_int,
    cs_gpio_index: c_int,
    num_amps: c_int,
) -> c_int {
    let mut gpio_mapping: *mut acpi_gpio_mapping = ptr::null_mut();
    let mut reset_gpio_params: *mut acpi_gpio_params = ptr::null_mut();
    let mut spkid_gpio_params: *mut acpi_gpio_params = ptr::null_mut();
    let mut cs_gpio_params: *mut acpi_gpio_params = ptr::null_mut();
    let mut num_entries: c_uint = 0;
    let mut reset_index: c_uint = 0;
    let mut spkid_index: c_uint = 0;
    let mut csgpio_index: c_uint = 0;
    let mut i: c_int;

    /*
     * GPIO Mapping only needs to be done once, since it would be available for subsequent amps
     */
    if !(*(*cs35l41).dacpi).driver_gpios.is_null() {
        return 0;
    }

    if reset_gpio >= 0 {
        reset_index = num_entries;
        num_entries += 1;
    }

    if spkid_gpio >= 0 {
        spkid_index = num_entries;
        num_entries += 1;
    }

    if (cs_gpio_index >= 0) && (num_amps == 2) {
        csgpio_index = num_entries;
        num_entries += 1;
    }

    if num_entries == 0 {
        return 0;
    }

    /* must include termination entry */
    num_entries += 1;

    gpio_mapping = devm_kcalloc(
        physdev,
        num_entries as usize,
        size_of::<acpi_gpio_mapping>(),
        GFP_KERNEL,
    ) as *mut acpi_gpio_mapping;

    if gpio_mapping.is_null() {
        goto_err(physdev, gpio_mapping, reset_gpio_params, spkid_gpio_params, cs_gpio_params);
        return -ENOMEM;
    }

    if reset_gpio >= 0 {
        (*gpio_mapping.add(reset_index as usize)).name = cstr!("reset-gpios");
        reset_gpio_params = devm_kcalloc(
            physdev,
            num_amps as usize,
            size_of::<acpi_gpio_params>(),
            GFP_KERNEL,
        ) as *mut acpi_gpio_params;
        if reset_gpio_params.is_null() {
            goto_err(physdev, gpio_mapping, reset_gpio_params, spkid_gpio_params, cs_gpio_params);
            return -ENOMEM;
        }

        i = 0;
        while i < num_amps {
            (*reset_gpio_params.add(i as usize)).crs_entry_index = reset_gpio as c_uint;
            i += 1;
        }

        (*gpio_mapping.add(reset_index as usize)).data = reset_gpio_params;
        (*gpio_mapping.add(reset_index as usize)).size = num_amps as c_uint;
    }

    if spkid_gpio >= 0 {
        (*gpio_mapping.add(spkid_index as usize)).name = cstr!("spk-id-gpios");
        spkid_gpio_params = devm_kcalloc(
            physdev,
            num_amps as usize,
            size_of::<acpi_gpio_params>(),
            GFP_KERNEL,
        ) as *mut acpi_gpio_params;
        if spkid_gpio_params.is_null() {
            goto_err(physdev, gpio_mapping, reset_gpio_params, spkid_gpio_params, cs_gpio_params);
            return -ENOMEM;
        }

        i = 0;
        while i < num_amps {
            (*spkid_gpio_params.add(i as usize)).crs_entry_index = spkid_gpio as c_uint;
            i += 1;
        }

        (*gpio_mapping.add(spkid_index as usize)).data = spkid_gpio_params;
        (*gpio_mapping.add(spkid_index as usize)).size = num_amps as c_uint;
    }

    if (cs_gpio_index >= 0) && (num_amps == 2) {
        (*gpio_mapping.add(csgpio_index as usize)).name = cstr!("cs-gpios");
        /* only one GPIO CS is supported without using _DSD, obtained using index 0 */
        cs_gpio_params =
            devm_kzalloc(physdev, size_of::<acpi_gpio_params>(), GFP_KERNEL) as *mut acpi_gpio_params;
        if cs_gpio_params.is_null() {
            goto_err(physdev, gpio_mapping, reset_gpio_params, spkid_gpio_params, cs_gpio_params);
            return -ENOMEM;
        }

        (*cs_gpio_params).crs_entry_index = cs_gpio_index as c_uint;

        (*gpio_mapping.add(csgpio_index as usize)).data = cs_gpio_params;
        (*gpio_mapping.add(csgpio_index as usize)).size = 1;
    }

    devm_acpi_dev_add_driver_gpios(physdev, gpio_mapping)
}

unsafe fn goto_err(
    physdev: *mut device,
    gpio_mapping: *mut acpi_gpio_mapping,
    reset_gpio_params: *mut acpi_gpio_params,
    spkid_gpio_params: *mut acpi_gpio_params,
    cs_gpio_params: *mut acpi_gpio_params,
) {
    devm_kfree(physdev, gpio_mapping as *mut c_void);
    devm_kfree(physdev, reset_gpio_params as *mut c_void);
    devm_kfree(physdev, spkid_gpio_params as *mut c_void);
    devm_kfree(physdev, cs_gpio_params as *mut c_void);
}

unsafe fn generic_dsd_config(
    cs35l41: *mut cs35l41_hda,
    physdev: *mut device,
    id: c_int,
    _hid: *const c_char,
) -> c_int {
    let hw_cfg: *mut cs35l41_hw_cfg = &mut (*cs35l41).hw_cfg;
    let mut cfg: *const cs35l41_config = cs35l41_config_table.as_ptr();
    let mut cs_gpiod: *mut gpio_desc;
    let mut spi: *mut spi_device;
    let dsd_found: bool;
    let mut ret: c_int;
    let mut i: c_int;

    while !(*cfg).ssid.is_null() {
        if strcasecmp((*cfg).ssid, (*cs35l41).acpi_subsystem_id) == 0 {
            break;
        }
        cfg = cfg.add(1);
    }

    if (*cfg).ssid.is_null() {
        return -ENOENT;
    }

    if (*cs35l41).dacpi.is_null() || (*cs35l41).dacpi != ACPI_COMPANION(physdev) {
        dev_err(
            (*cs35l41).dev,
            cstr!("ACPI Device does not match, cannot override _DSD.\n"),
        );
        return -ENODEV;
    }

    dev_info(
        (*cs35l41).dev,
        cstr!("Adding DSD properties for %s\n"),
        (*cs35l41).acpi_subsystem_id,
    );

    dsd_found = acpi_dev_has_props((*cs35l41).dacpi);

    if !dsd_found {
        ret = cs35l41_add_gpios(
            cs35l41,
            physdev,
            (*cfg).reset_gpio_index,
            (*cfg).spkid_gpio_index,
            (*cfg).cs_gpio_index,
            (*cfg).num_amps,
        );
        if ret != 0 {
            dev_err((*cs35l41).dev, cstr!("Error adding GPIO mapping: %d\n"), ret);
            return ret;
        }
    } else if (*cfg).reset_gpio_index >= 0 || (*cfg).spkid_gpio_index >= 0 {
        dev_warn(
            (*cs35l41).dev,
            cstr!("Cannot add Reset/Speaker ID/SPI CS GPIO Mapping, _DSD already exists.\n"),
        );
    }

    if (*cs35l41).control_bus == SPI {
        (*cs35l41).index = id;

        /*
         * Manually set the Chip Select for the second amp <cs_gpio_index> in the node.
         * This is only supported for systems with 2 amps, since we cannot expand the
         * default number of chip selects without using cs-gpios
         * The CS GPIO must be set high prior to communicating with the first amp (which
         * uses a native chip select), to ensure the second amp does not clash with the
         * first.
         */
        if CONFIG_SPI && (*cfg).cs_gpio_index >= 0 {
            spi = to_spi_device((*cs35l41).dev);

            if (*cfg).num_amps != 2 {
                dev_warn(
                    (*cs35l41).dev,
                    cstr!("Cannot update SPI CS, Number of Amps (%d) != 2\n"),
                    (*cfg).num_amps,
                );
            } else if dsd_found {
                dev_warn(
                    (*cs35l41).dev,
                    cstr!("Cannot update SPI CS, _DSD already exists.\n"),
                );
            } else {
                /*
                 * This is obtained using driver_gpios, since only one GPIO for CS
                 * exists, this can be obtained using index 0.
                 */
                cs_gpiod = gpiod_get_index(physdev, cstr!("cs"), 0, GPIOD_OUT_LOW);
                if IS_ERR(cs_gpiod as *const c_void) {
                    dev_err(
                        (*cs35l41).dev,
                        cstr!("Unable to get Chip Select GPIO descriptor\n"),
                    );
                    return PTR_ERR(cs_gpiod as *const c_void);
                }
                if id == 1 {
                    spi_set_csgpiod(spi, 0, cs_gpiod);
                    (*cs35l41).cs_gpio = cs_gpiod;
                } else {
                    gpiod_set_value_cansleep(cs_gpiod, true);
                    gpiod_put(cs_gpiod);
                }
                spi_setup(spi);
            }
        }
    } else if (*cfg).num_amps > 2 {
        /*
         * i2c addresses for 3/4 amps are used in order: 0x40, 0x41, 0x42, 0x43,
         * subtracting 0x40 would give zero-based index
         */
        (*cs35l41).index = id - 0x40;
    } else {
        /* i2c addr 0x40 for first amp (always), 0x41/0x42 for 2nd amp */
        (*cs35l41).index = if id == 0x40 { 0 } else { 1 };
    }

    (*cs35l41).reset_gpio = fwnode_gpiod_get_index(
        acpi_fwnode_handle((*cs35l41).dacpi),
        cstr!("reset"),
        (*cs35l41).index as c_uint,
        GPIOD_OUT_LOW,
        cstr!("cs35l41-reset"),
    );
    (*cs35l41).speaker_id =
        cs35l41_get_speaker_id(physdev, (*cs35l41).index, (*cfg).num_amps, -1);

    (*hw_cfg).spk_pos = (*cfg).channel[(*cs35l41).index as usize];

    (*cs35l41).channel_index = 0;
    i = 0;
    while i < (*cs35l41).index {
        if (*cfg).channel[i as usize] == (*hw_cfg).spk_pos {
            (*cs35l41).channel_index += 1;
        }
        i += 1;
    }

    if (*cfg).boost_type == cs35l41_boost_type::INTERNAL {
        (*hw_cfg).bst_type = CS35L41_INT_BOOST;
        (*hw_cfg).bst_ind = (*cfg).boost_ind_nanohenry;
        (*hw_cfg).bst_ipk = (*cfg).boost_peak_milliamp;
        (*hw_cfg).bst_cap = (*cfg).boost_cap_microfarad;
        (*hw_cfg).gpio1.func = CS35L41_NOT_USED;
        (*hw_cfg).gpio1.valid = true;
    } else {
        (*hw_cfg).bst_type = CS35L41_EXT_BOOST;
        (*hw_cfg).bst_ind = -1;
        (*hw_cfg).bst_ipk = -1;
        (*hw_cfg).bst_cap = -1;
        (*hw_cfg).gpio1.func = CS35l41_VSPK_SWITCH;
        (*hw_cfg).gpio1.valid = true;
    }

    (*hw_cfg).gpio2.func = CS35L41_INTERRUPT;
    (*hw_cfg).gpio2.valid = true;
    (*hw_cfg).valid = true;

    0
}

/*
 * Systems 103C8C66, 103C8C67, 103C8C68, 103C8C6A use a dual speaker id system - each speaker has
 * its own speaker id.
 */
unsafe fn hp_i2c_int_2amp_dual_spkid(
    cs35l41: *mut cs35l41_hda,
    physdev: *mut device,
    id: c_int,
    _hid: *const c_char,
) -> c_int {
    let hw_cfg: *mut cs35l41_hw_cfg = &mut (*cs35l41).hw_cfg;

    /* If _DSD exists for this laptop, we cannot support it through here */
    if acpi_dev_has_props((*cs35l41).dacpi) {
        return -ENOENT;
    }

    /* check I2C address to assign the index */
    (*cs35l41).index = if id == 0x40 { 0 } else { 1 };
    (*cs35l41).channel_index = 0;
    (*cs35l41).reset_gpio = gpiod_get_index(physdev, ptr::null(), 0, GPIOD_OUT_HIGH);
    if (*cs35l41).index == 0 {
        (*cs35l41).speaker_id = cs35l41_get_speaker_id(physdev, 0, 0, 1);
    } else {
        (*cs35l41).speaker_id = cs35l41_get_speaker_id(physdev, 0, 0, 2);
    }
    (*hw_cfg).spk_pos = (*cs35l41).index as u8;
    (*hw_cfg).gpio2.func = CS35L41_INTERRUPT;
    (*hw_cfg).gpio2.valid = true;
    (*hw_cfg).valid = true;

    (*hw_cfg).bst_type = CS35L41_INT_BOOST;
    (*hw_cfg).bst_ind = 1000;
    (*hw_cfg).bst_ipk = 4100;
    (*hw_cfg).bst_cap = 24;
    (*hw_cfg).gpio1.func = CS35L41_NOT_USED;
    (*hw_cfg).gpio1.valid = true;

    0
}

/*
 * Device CLSA010(0/1) doesn't have _DSD so a gpiod_get by the label reset won't work.
 * And devices created by serial-multi-instantiate don't have their device struct
 * pointing to the correct fwnode, so acpi_dev must be used here.
 * And devm functions expect that the device requesting the resource has the correct
 * fwnode.
 */
unsafe fn lenovo_legion_no_acpi(
    cs35l41: *mut cs35l41_hda,
    physdev: *mut device,
    id: c_int,
    hid: *const c_char,
) -> c_int {
    let hw_cfg: *mut cs35l41_hw_cfg = &mut (*cs35l41).hw_cfg;

    /* check I2C address to assign the index */
    (*cs35l41).index = if id == 0x40 { 0 } else { 1 };
    (*cs35l41).channel_index = 0;
    (*cs35l41).reset_gpio = gpiod_get_index(physdev, ptr::null(), 0, GPIOD_OUT_HIGH);
    (*cs35l41).speaker_id = cs35l41_get_speaker_id(physdev, 0, 0, 2);
    (*hw_cfg).spk_pos = (*cs35l41).index as u8;
    (*hw_cfg).gpio2.func = CS35L41_INTERRUPT;
    (*hw_cfg).gpio2.valid = true;
    (*hw_cfg).valid = true;

    if strcmp(hid, cstr!("CLSA0100")) == 0 {
        (*hw_cfg).bst_type = CS35L41_EXT_BOOST_NO_VSPK_SWITCH;
    } else if strcmp(hid, cstr!("CLSA0101")) == 0 {
        (*hw_cfg).bst_type = CS35L41_EXT_BOOST;
        (*hw_cfg).gpio1.func = CS35l41_VSPK_SWITCH;
        (*hw_cfg).gpio1.valid = true;
    }

    0
}

unsafe fn missing_speaker_id_gpio2(
    cs35l41: *mut cs35l41_hda,
    physdev: *mut device,
    id: c_int,
    _hid: *const c_char,
) -> c_int {
    let ret: c_int;

    ret = cs35l41_add_gpios(cs35l41, physdev, -1, 2, -1, 2);
    if ret != 0 {
        dev_err((*cs35l41).dev, cstr!("Error adding GPIO mapping: %d\n"), ret);
        return ret;
    }

    cs35l41_hda_parse_acpi(cs35l41, physdev, id)
}

type add_prop_fn = unsafe fn(
    cs35l41: *mut cs35l41_hda,
    physdev: *mut device,
    id: c_int,
    hid: *const c_char,
) -> c_int;

#[repr(C)]
#[derive(Copy, Clone)]
struct cs35l41_prop_model {
    hid: *const c_char,
    ssid: *const c_char,
    add_prop: Option<add_prop_fn>,
}

macro_rules! model {
    ($hid:literal, NULL, $func:ident) => {
        cs35l41_prop_model { hid: cstr!($hid), ssid: ptr::null(), add_prop: Some($func) }
    };
    ($hid:literal, $ssid:literal, $func:ident) => {
        cs35l41_prop_model { hid: cstr!($hid), ssid: cstr!($ssid), add_prop: Some($func) }
    };
}

static cs35l41_prop_model_table: &[cs35l41_prop_model] = &[
    model!("CLSA0100", NULL, lenovo_legion_no_acpi),
    model!("CLSA0101", NULL, lenovo_legion_no_acpi),
    model!("CSC3551", "10251826", generic_dsd_config),
    model!("CSC3551", "1025182C", generic_dsd_config),
    model!("CSC3551", "10251844", generic_dsd_config),
    model!("CSC3551", "10280B27", generic_dsd_config),
    model!("CSC3551", "10280B28", generic_dsd_config),
    model!("CSC3551", "10280BEB", generic_dsd_config),
    model!("CSC3551", "10280C4D", generic_dsd_config),
    model!("CSC3551", "103C89C6", generic_dsd_config),
    model!("CSC3551", "103C8A28", generic_dsd_config),
    model!("CSC3551", "103C8A29", generic_dsd_config),
    model!("CSC3551", "103C8A2A", generic_dsd_config),
    model!("CSC3551", "103C8A2B", generic_dsd_config),
    model!("CSC3551", "103C8A2C", generic_dsd_config),
    model!("CSC3551", "103C8A2D", generic_dsd_config),
    model!("CSC3551", "103C8A2E", generic_dsd_config),
    model!("CSC3551", "103C8A30", generic_dsd_config),
    model!("CSC3551", "103C8A31", generic_dsd_config),
    model!("CSC3551", "103C8A6E", generic_dsd_config),
    model!("CSC3551", "103C8B63", generic_dsd_config),
    model!("CSC3551", "103C8BB3", generic_dsd_config),
    model!("CSC3551", "103C8BB4", generic_dsd_config),
    model!("CSC3551", "103C8BDD", generic_dsd_config),
    model!("CSC3551", "103C8BDE", generic_dsd_config),
    model!("CSC3551", "103C8BDF", generic_dsd_config),
    model!("CSC3551", "103C8BE0", generic_dsd_config),
    model!("CSC3551", "103C8BE1", generic_dsd_config),
    model!("CSC3551", "103C8BE2", generic_dsd_config),
    model!("CSC3551", "103C8BE3", generic_dsd_config),
    model!("CSC3551", "103C8BE5", generic_dsd_config),
    model!("CSC3551", "103C8BE6", generic_dsd_config),
    model!("CSC3551", "103C8BE7", generic_dsd_config),
    model!("CSC3551", "103C8BE8", generic_dsd_config),
    model!("CSC3551", "103C8BE9", generic_dsd_config),
    model!("CSC3551", "103C8B3A", generic_dsd_config),
    model!("CSC3551", "103C8C15", generic_dsd_config),
    model!("CSC3551", "103C8C16", generic_dsd_config),
    model!("CSC3551", "103C8C17", generic_dsd_config),
    model!("CSC3551", "103C8C4D", generic_dsd_config),
    model!("CSC3551", "103C8C4E", generic_dsd_config),
    model!("CSC3551", "103C8C4F", generic_dsd_config),
    model!("CSC3551", "103C8C50", generic_dsd_config),
    model!("CSC3551", "103C8C51", generic_dsd_config),
    model!("CSC3551", "103C8C66", hp_i2c_int_2amp_dual_spkid),
    model!("CSC3551", "103C8C67", hp_i2c_int_2amp_dual_spkid),
    model!("CSC3551", "103C8C68", hp_i2c_int_2amp_dual_spkid),
    model!("CSC3551", "103C8C6A", hp_i2c_int_2amp_dual_spkid),
    model!("CSC3551", "103C8CDD", generic_dsd_config),
    model!("CSC3551", "103C8CDE", generic_dsd_config),
    model!("CSC3551", "104312AF", generic_dsd_config),
    model!("CSC3551", "10431433", generic_dsd_config),
    model!("CSC3551", "10431463", generic_dsd_config),
    model!("CSC3551", "10431473", generic_dsd_config),
    model!("CSC3551", "10431483", generic_dsd_config),
    model!("CSC3551", "10431493", generic_dsd_config),
    model!("CSC3551", "104314D3", generic_dsd_config),
    model!("CSC3551", "104314E3", generic_dsd_config),
    model!("CSC3551", "10431503", generic_dsd_config),
    model!("CSC3551", "10431533", generic_dsd_config),
    model!("CSC3551", "10431573", generic_dsd_config),
    model!("CSC3551", "10431663", generic_dsd_config),
    model!("CSC3551", "10431683", generic_dsd_config),
    model!("CSC3551", "104316A3", generic_dsd_config),
    model!("CSC3551", "104316D3", generic_dsd_config),
    model!("CSC3551", "104316F3", generic_dsd_config),
    model!("CSC3551", "104317F3", generic_dsd_config),
    model!("CSC3551", "10431863", generic_dsd_config),
    model!("CSC3551", "104318D3", generic_dsd_config),
    model!("CSC3551", "10431A63", missing_speaker_id_gpio2),
    model!("CSC3551", "10431A83", generic_dsd_config),
    model!("CSC3551", "10431B93", generic_dsd_config),
    model!("CSC3551", "10431C9F", generic_dsd_config),
    model!("CSC3551", "10431CAF", generic_dsd_config),
    model!("CSC3551", "10431CCF", generic_dsd_config),
    model!("CSC3551", "10431CDF", generic_dsd_config),
    model!("CSC3551", "10431CEF", generic_dsd_config),
    model!("CSC3551", "10431D1F", generic_dsd_config),
    model!("CSC3551", "10431DA2", generic_dsd_config),
    model!("CSC3551", "10431E02", generic_dsd_config),
    model!("CSC3551", "10431E12", generic_dsd_config),
    model!("CSC3551", "10431EE2", generic_dsd_config),
    model!("CSC3551", "10431F12", generic_dsd_config),
    model!("CSC3551", "10431F1F", generic_dsd_config),
    model!("CSC3551", "10431F62", generic_dsd_config),
    model!("CSC3551", "10433A20", generic_dsd_config),
    model!("CSC3551", "10433A30", generic_dsd_config),
    model!("CSC3551", "10433A40", generic_dsd_config),
    model!("CSC3551", "10433A50", generic_dsd_config),
    model!("CSC3551", "10433A60", generic_dsd_config),
    model!("CSC3551", "17AA3865", generic_dsd_config),
    model!("CSC3551", "17AA3866", generic_dsd_config),
    model!("CSC3551", "17AA386E", generic_dsd_config),
    model!("CSC3551", "17AA386F", generic_dsd_config),
    model!("CSC3551", "17AA3874", generic_dsd_config),
    model!("CSC3551", "17AA3877", generic_dsd_config),
    model!("CSC3551", "17AA3878", generic_dsd_config),
    model!("CSC3551", "17AA38A9", generic_dsd_config),
    model!("CSC3551", "17AA38AB", generic_dsd_config),
    model!("CSC3551", "17AA38B4", generic_dsd_config),
    model!("CSC3551", "17AA38B5", generic_dsd_config),
    model!("CSC3551", "17AA38B6", generic_dsd_config),
    model!("CSC3551", "17AA38B7", generic_dsd_config),
    model!("CSC3551", "17AA38C7", generic_dsd_config),
    model!("CSC3551", "17AA38C8", generic_dsd_config),
    model!("CSC3551", "17AA38F9", generic_dsd_config),
    model!("CSC3551", "17AA38FA", generic_dsd_config),
    model!("CSC3551", "17AA3929", generic_dsd_config),
    model!("CSC3551", "17AA392B", generic_dsd_config),
    cs35l41_prop_model { hid: ptr::null(), ssid: ptr::null(), add_prop: None },
];

#[no_mangle]
pub unsafe extern "C" fn cs35l41_add_dsd_properties(
    cs35l41: *mut cs35l41_hda,
    physdev: *mut device,
    id: c_int,
    hid: *const c_char,
) -> c_int {
    let mut model: *const cs35l41_prop_model = cs35l41_prop_model_table.as_ptr();

    while !(*model).hid.is_null() {
        if strcmp((*model).hid, hid) == 0
            && ((*model).ssid.is_null()
                || (!(*cs35l41).acpi_subsystem_id.is_null()
                    && strcasecmp((*model).ssid, (*cs35l41).acpi_subsystem_id) == 0))
        {
            return ((*model).add_prop.unwrap())(cs35l41, physdev, id, hid);
        }
        model = model.add(1);
    }

    -ENOENT
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
