/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  include/linux/mmc/sdio_func.h
 *
 *  Copyright 2007-2008 Pierre Ossman
 */

// Dependencies supplied by other headers remain external to this translation.

use core::ffi::{c_char, c_int, c_uint, c_void};

#[repr(C)]
pub struct mmc_card {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device {
    _private: [u8; 0],
}

#[repr(C)]
pub struct device_driver {
    _private: [u8; 0],
}

#[repr(C)]
pub struct module {
    _private: [u8; 0],
}

pub type u8 = core::ffi::c_uchar;
pub type u16 = core::ffi::c_ushort;
pub type u32 = core::ffi::c_uint;
pub type mmc_pm_flag_t = u32;

#[repr(C)]
pub struct sdio_device_id {
    pub class: u8,
    pub vendor: u16,
    pub device: u16,
}

pub type sdio_irq_handler_t = unsafe extern "C" fn(func: *mut sdio_func);

/* SDIO function CIS tuple (unknown to the core) */
#[repr(C)]
pub struct sdio_func_tuple {
    pub next: *mut sdio_func_tuple,
    pub code: u8,
    pub size: u8,
    pub data: [u8; 0],
}

/* SDIO function devices */
#[repr(C)]
pub struct sdio_func {
    pub card: *mut mmc_card,
    pub dev: device,
    pub irq_handler: Option<sdio_irq_handler_t>,
    pub num: u32,
    pub class: u8,
    pub vendor: u16,
    pub device: u16,
    pub max_blksize: c_uint,
    pub cur_blksize: c_uint,
    pub enable_timeout: c_uint,
    pub state: c_uint,
    pub tmpbuf: *mut u8,
    pub major_rev: u8,
    pub minor_rev: u8,
    pub num_info: c_uint,
    pub info: *const *const c_char,
    pub tuples: *mut sdio_func_tuple,
}

pub const SDIO_STATE_PRESENT: u32 = 1 << 0;

/* sdio_func_present(f): ((f)->state & SDIO_STATE_PRESENT) */
#[inline]
pub unsafe fn sdio_func_present(f: *const sdio_func) -> u32 {
    (*f).state & SDIO_STATE_PRESENT
}

#[inline]
pub unsafe fn sdio_func_set_present(f: *mut sdio_func) {
    (*f).state |= SDIO_STATE_PRESENT;
}

/* sdio_func_id(f): dev_name(&(f)->dev) */
#[inline]
pub unsafe fn sdio_func_id(f: *const sdio_func) -> *const c_char {
    dev_name(&(*f).dev)
}

/* sdio_get_drvdata(f), sdio_set_drvdata(f,d), and dev_to_sdio_func(d) retain
 * their kernel helper semantics and depend on the external device helpers. */
extern "C" {
    pub fn dev_name(dev: *const device) -> *const c_char;
    pub fn dev_get_drvdata(dev: *const device) -> *mut c_void;
    pub fn dev_set_drvdata(dev: *mut device, data: *mut c_void);
}

#[inline]
pub unsafe fn sdio_get_drvdata(f: *const sdio_func) -> *mut c_void {
    dev_get_drvdata(&(*f).dev)
}

#[inline]
pub unsafe fn sdio_set_drvdata(f: *mut sdio_func, d: *mut c_void) {
    dev_set_drvdata(&mut (*f).dev, d);
}

/* SDIO function device driver */
#[repr(C)]
pub struct sdio_driver {
    pub name: *mut c_char,
    pub id_table: *const sdio_device_id,
    pub probe: Option<unsafe extern "C" fn(*mut sdio_func, *const sdio_device_id) -> c_int>,
    pub remove: Option<unsafe extern "C" fn(*mut sdio_func)>,
    pub shutdown: Option<unsafe extern "C" fn(*mut sdio_func)>,
    pub drv: device_driver,
}

pub const SDIO_ANY_ID: u16 = !0;

/* SDIO_DEVICE(vend, dev): .class = SDIO_ANY_ID, .vendor = vend, .device = dev */
/* SDIO_DEVICE_CLASS(dev_class): .class = dev_class, .vendor = SDIO_ANY_ID,
 * .device = SDIO_ANY_ID */

extern "C" {
    pub fn __sdio_register_driver(drv: *mut sdio_driver, module: *mut module) -> c_int;
    pub fn sdio_unregister_driver(drv: *mut sdio_driver);

    pub fn sdio_claim_host(func: *mut sdio_func);
    pub fn sdio_release_host(func: *mut sdio_func);
    pub fn sdio_enable_func(func: *mut sdio_func) -> c_int;
    pub fn sdio_disable_func(func: *mut sdio_func) -> c_int;
    pub fn sdio_set_block_size(func: *mut sdio_func, blksz: c_uint) -> c_int;
    pub fn sdio_claim_irq(func: *mut sdio_func, handler: Option<sdio_irq_handler_t>) -> c_int;
    pub fn sdio_release_irq(func: *mut sdio_func) -> c_int;
    pub fn sdio_align_size(func: *mut sdio_func, sz: c_uint) -> c_uint;
    pub fn sdio_readb(func: *mut sdio_func, addr: c_uint, err_ret: *mut c_int) -> u8;
    pub fn sdio_readw(func: *mut sdio_func, addr: c_uint, err_ret: *mut c_int) -> u16;
    pub fn sdio_readl(func: *mut sdio_func, addr: c_uint, err_ret: *mut c_int) -> u32;
    pub fn sdio_memcpy_fromio(func: *mut sdio_func, dst: *mut c_void, addr: c_uint, count: c_int) -> c_int;
    pub fn sdio_readsb(func: *mut sdio_func, dst: *mut c_void, addr: c_uint, count: c_int) -> c_int;
    pub fn sdio_writeb(func: *mut sdio_func, b: u8, addr: c_uint, err_ret: *mut c_int);
    pub fn sdio_writew(func: *mut sdio_func, b: u16, addr: c_uint, err_ret: *mut c_int);
    pub fn sdio_writel(func: *mut sdio_func, b: u32, addr: c_uint, err_ret: *mut c_int);
    pub fn sdio_writeb_readb(func: *mut sdio_func, write_byte: u8, addr: c_uint, err_ret: *mut c_int) -> u8;
    pub fn sdio_memcpy_toio(func: *mut sdio_func, addr: c_uint, src: *mut c_void, count: c_int) -> c_int;
    pub fn sdio_writesb(func: *mut sdio_func, addr: c_uint, src: *mut c_void, count: c_int) -> c_int;
    pub fn sdio_f0_readb(func: *mut sdio_func, addr: c_uint, err_ret: *mut c_int) -> u8;
    pub fn sdio_f0_writeb(func: *mut sdio_func, b: u8, addr: c_uint, err_ret: *mut c_int);
    pub fn sdio_get_host_pm_caps(func: *mut sdio_func) -> mmc_pm_flag_t;
    pub fn sdio_set_host_pm_flags(func: *mut sdio_func, flags: mmc_pm_flag_t) -> c_int;
    pub fn sdio_retune_crc_disable(func: *mut sdio_func);
    pub fn sdio_retune_crc_enable(func: *mut sdio_func);
    pub fn sdio_retune_hold_now(func: *mut sdio_func);
    pub fn sdio_retune_release(func: *mut sdio_func);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
