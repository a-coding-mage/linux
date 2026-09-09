// SPDX-License-Identifier: GPL-2.0
//
// Register map access API - SPI support
//
// Copyright 2011 Wolfson Microelectronics plc
//
// Author: Mark Brown <broonie@opensource.wolfsonmicro.com>

// Dependencies supplied by the Linux regmap, SPI, module, and internal APIs.

use core::ffi::{c_char, c_int, c_void};

#[repr(C)]
pub struct regmap_async {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_message {
    pub status: c_int,
    pub complete: Option<unsafe extern "C" fn(*mut c_void)>,
    pub context: *mut c_void,
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_transfer {
    pub tx_buf: *const c_void,
    pub len: usize,
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct spi_device {
    pub dev: device,
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_config {
    pub reg_bits: usize,
    pub pad_bits: usize,
    _private: [u8; 0],
}

#[repr(C)]
pub struct lock_class_key {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _private: [u8; 0],
}

#[repr(C)]
pub struct regmap_bus {
    pub write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize) -> c_int>,
    pub gather_write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize, *const c_void, usize) -> c_int>,
    pub async_write: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize, *const c_void, usize, *mut regmap_async) -> c_int>,
    pub async_alloc: Option<unsafe extern "C" fn() -> *mut regmap_async>,
    pub read: Option<unsafe extern "C" fn(*mut c_void, *const c_void, usize, *mut c_void, usize) -> c_int>,
    pub read_flag_mask: u32,
    pub reg_format_endian_default: c_int,
    pub val_format_endian_default: c_int,
    pub free_on_exit: bool,
    pub max_raw_read: usize,
    pub max_raw_write: usize,
}

extern "C" {
    fn regmap_async_complete_cb(async_: *mut regmap_async, status: c_int);
    fn spi_write(spi: *mut spi_device, data: *const c_void, count: usize) -> c_int;
    fn to_spi_device(dev: *mut device) -> *mut spi_device;
    fn spi_message_init(message: *mut spi_message);
    fn spi_message_add_tail(transfer: *mut spi_transfer, message: *mut spi_message);
    fn spi_sync(spi: *mut spi_device, message: *mut spi_message) -> c_int;
    fn spi_async(spi: *mut spi_device, message: *mut spi_message) -> c_int;
    fn spi_write_then_read(spi: *mut spi_device, reg: *const c_void, reg_size: usize, val: *mut c_void, val_size: usize) -> c_int;
    fn spi_max_transfer_size(spi: *mut spi_device) -> usize;
    fn spi_max_message_size(spi: *mut spi_device) -> usize;
    fn kmemdup(src: *const c_void, size: usize, flags: c_int) -> *mut regmap_bus;
    fn regmap_get_error(ptr: *const c_void) -> bool;
    fn regmap_error_cast(ptr: *const regmap_bus) -> *mut regmap_bus;
    fn regmap_init(spi_dev: *mut device, bus: *const regmap_bus, context: *mut device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const c_char) -> *mut regmap;
    fn devm_regmap_init(spi_dev: *mut device, bus: *const regmap_bus, context: *mut device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const c_char) -> *mut regmap;
}

const REGMAP_ENDIAN_BIG: c_int = 0;
const BITS_PER_BYTE: usize = 8;
const SIZE_MAX: usize = usize::MAX;

#[repr(C)]
struct regmap_async_spi {
    core: regmap_async,
    m: spi_message,
    t: [spi_transfer; 2],
}

unsafe extern "C" fn regmap_spi_complete(data: *mut c_void) {
    let async_ = data as *mut regmap_async_spi;
    regmap_async_complete_cb(&mut (*async_).core, (*async_).m.status);
}

unsafe extern "C" fn regmap_spi_write(context: *mut c_void, data: *const c_void, count: usize) -> c_int {
    let dev = context as *mut device;
    let spi = to_spi_device(dev);
    spi_write(spi, data, count)
}

unsafe extern "C" fn regmap_spi_gather_write(context: *mut c_void, reg: *const c_void, reg_len: usize, val: *const c_void, val_len: usize) -> c_int {
    let dev = context as *mut device;
    let spi = to_spi_device(dev);
    let mut m = core::mem::zeroed::<spi_message>();
    let mut t: [spi_transfer; 2] = [
        spi_transfer { tx_buf: reg, len: reg_len, _private: [] },
        spi_transfer { tx_buf: val, len: val_len, _private: [] },
    ];
    spi_message_init(&mut m);
    spi_message_add_tail(&mut t[0], &mut m);
    spi_message_add_tail(&mut t[1], &mut m);
    spi_sync(spi, &mut m)
}

unsafe extern "C" fn regmap_spi_async_write(context: *mut c_void, reg: *const c_void, reg_len: usize, val: *const c_void, val_len: usize, a: *mut regmap_async) -> c_int {
    let async_ = (a as *mut u8).sub(core::mem::offset_of!(regmap_async_spi, core)) as *mut regmap_async_spi;
    let dev = context as *mut device;
    let spi = to_spi_device(dev);
    (*async_).t[0].tx_buf = reg;
    (*async_).t[0].len = reg_len;
    (*async_).t[1].tx_buf = val;
    (*async_).t[1].len = val_len;
    spi_message_init(&mut (*async_).m);
    spi_message_add_tail(&mut (*async_).t[0], &mut (*async_).m);
    if !val.is_null() {
        spi_message_add_tail(&mut (*async_).t[1], &mut (*async_).m);
    }
    (*async_).m.complete = Some(regmap_spi_complete);
    (*async_).m.context = async_ as *mut c_void;
    spi_async(spi, &mut (*async_).m)
}

unsafe extern "C" fn regmap_spi_async_alloc() -> *mut regmap_async {
    let async_spi = libc::calloc(1, core::mem::size_of::<regmap_async_spi>()) as *mut regmap_async_spi;
    if async_spi.is_null() { return core::ptr::null_mut(); }
    &mut (*async_spi).core
}

unsafe extern "C" fn regmap_spi_read(context: *mut c_void, reg: *const c_void, reg_size: usize, val: *mut c_void, val_size: usize) -> c_int {
    let dev = context as *mut device;
    let spi = to_spi_device(dev);
    spi_write_then_read(spi, reg, reg_size, val, val_size)
}

static mut REGMAP_SPI: regmap_bus = regmap_bus {
    write: Some(regmap_spi_write), gather_write: Some(regmap_spi_gather_write), async_write: Some(regmap_spi_async_write), async_alloc: Some(regmap_spi_async_alloc), read: Some(regmap_spi_read), read_flag_mask: 0x80, reg_format_endian_default: REGMAP_ENDIAN_BIG, val_format_endian_default: REGMAP_ENDIAN_BIG, free_on_exit: false, max_raw_read: 0, max_raw_write: 0,
};

unsafe fn regmap_get_spi_bus(spi: *mut spi_device, config: *const regmap_config) -> *const regmap_bus {
    let mut max_size = spi_max_transfer_size(spi);
    if max_size != SIZE_MAX {
        let bus = kmemdup(&REGMAP_SPI as *const _, core::mem::size_of::<regmap_bus>(), 0);
        if bus.is_null() { return regmap_error_cast(core::ptr::null()); }
        let max_msg_size = spi_max_message_size(spi);
        let reg_reserve_size = ((*config).reg_bits + (*config).pad_bits) / BITS_PER_BYTE;
        if max_size + reg_reserve_size > max_msg_size { max_size -= reg_reserve_size; }
        (*bus).free_on_exit = true;
        (*bus).max_raw_read = max_size;
        (*bus).max_raw_write = max_size;
        return bus;
    }
    &REGMAP_SPI
}

pub unsafe fn __regmap_init_spi(spi: *mut spi_device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const c_char) -> *mut regmap {
    let bus = regmap_get_spi_bus(spi, config);
    if regmap_get_error(bus) { return regmap_error_cast(bus); }
    regmap_init(&mut (*spi).dev, bus, &mut (*spi).dev, config, lock_key, lock_name)
}

pub unsafe fn __devm_regmap_init_spi(spi: *mut spi_device, config: *const regmap_config, lock_key: *mut lock_class_key, lock_name: *const c_char) -> *mut regmap {
    let bus = regmap_get_spi_bus(spi, config);
    if regmap_get_error(bus) { return regmap_error_cast(bus); }
    devm_regmap_init(&mut (*spi).dev, bus, &mut (*spi).dev, config, lock_key, lock_name)
}

// EXPORT_SYMBOL_GPL(__regmap_init_spi);
// EXPORT_SYMBOL_GPL(__devm_regmap_init_spi);
// MODULE_DESCRIPTION("regmap SPI Module");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
