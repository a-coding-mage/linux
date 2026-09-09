// SPDX-License-Identifier: GPL-2.0 OR BSD-3-Clause
/*
 * Copyright (c) 2016 BayLibre, SAS.
 * Author: Neil Armstrong <narmstrong@baylibre.com>
 * Copyright (C) 2014 Amlogic, Inc.
 */
// C dependencies: linux/err.h, linux/module.h, linux/io.h,
// linux/platform_device.h, linux/hw_random.h, linux/slab.h,
// linux/types.h, linux/of.h, linux/clk.h, linux/iopoll.h

const RNG_DATA: usize = 0x00;
const RNG_S4_DATA: usize = 0x08;
const RNG_S4_CFG: usize = 0x00;

const RUN_BIT: u32 = 1u32 << 0;
const SEED_READY_STS_BIT: u32 = 1u32 << 31;

#[repr(C)]
pub struct meson_rng_priv {
    pub read: Option<unsafe extern "C" fn(rng: *mut hwrng, buf: *mut core::ffi::c_void, max: usize, wait: bool) -> i32>,
}

#[repr(C)]
pub struct meson_rng_data {
    pub base: *mut core::ffi::c_void,
    pub rng: hwrng,
    pub dev: *mut device,
}

#[repr(C)]
pub struct hwrng {
    pub name: *const core::ffi::c_char,
    pub read: Option<unsafe extern "C" fn(*mut hwrng, *mut core::ffi::c_void, usize, bool) -> i32>,
}

#[repr(C)]
pub struct device;
#[repr(C)]
pub struct platform_device {
    pub dev: device,
    pub name: *const core::ffi::c_char,
}
#[repr(C)]
pub struct clk;
#[repr(C)]
pub struct platform_driver;
#[repr(C)]
pub struct of_device_id {
    pub compatible: *const core::ffi::c_char,
    pub data: *const core::ffi::c_void,
}

unsafe extern "C" {
    fn readl_relaxed(addr: *mut core::ffi::c_void) -> u32;
    fn writel_relaxed(value: u32, addr: *mut core::ffi::c_void);
    fn readl_relaxed_poll_timeout_atomic(addr: *mut core::ffi::c_void, val: *mut u32, cond: bool, delay_us: u32, timeout_us: u32) -> i32;
    fn dev_err(dev: *mut device, fmt: *const core::ffi::c_char, ...);
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut core::ffi::c_void;
    fn device_get_match_data(dev: *mut device) -> *const core::ffi::c_void;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut core::ffi::c_void;
    fn devm_clk_get_optional_enabled(dev: *mut device, id: *const core::ffi::c_char) -> *mut clk;
    fn dev_err_probe(dev: *mut device, err: i32, fmt: *const core::ffi::c_char, ...) -> i32;
    fn devm_hwrng_register(dev: *mut device, rng: *mut hwrng) -> i32;
}

unsafe extern "C" fn meson_rng_read(rng: *mut hwrng, buf: *mut core::ffi::c_void, _max: usize, _wait: bool) -> i32 {
    let data = (rng as *mut u8).sub(core::mem::offset_of!(meson_rng_data, rng)) as *mut meson_rng_data;
    *(buf as *mut u32) = readl_relaxed((*data).base.add(RNG_DATA));
    core::mem::size_of::<u32>() as i32
}

unsafe extern "C" fn meson_rng_wait_status(cfg_addr: *mut core::ffi::c_void, bit: i32) -> i32 {
    let mut status: u32 = 0;
    let ret = readl_relaxed_poll_timeout_atomic(cfg_addr, &mut status, (status & bit as u32) == 0, 10, 10000);
    if ret != 0 { return -16; }
    0
}

unsafe extern "C" fn meson_s4_rng_read(rng: *mut hwrng, buf: *mut core::ffi::c_void, _max: usize, _wait: bool) -> i32 {
    let data = (rng as *mut u8).sub(core::mem::offset_of!(meson_rng_data, rng)) as *mut meson_rng_data;
    let cfg_addr = (*data).base.add(RNG_S4_CFG);
    writel_relaxed(readl_relaxed(cfg_addr) | SEED_READY_STS_BIT, cfg_addr);
    let mut err = meson_rng_wait_status(cfg_addr, SEED_READY_STS_BIT as i32);
    if err != 0 { dev_err((*data).dev, c"Seed isn't ready, try again\n".as_ptr(),); return err; }
    err = meson_rng_wait_status(cfg_addr, RUN_BIT as i32);
    if err != 0 { dev_err((*data).dev, c"Can't get random number, try again\n".as_ptr(),); return err; }
    *(buf as *mut u32) = readl_relaxed((*data).base.add(RNG_S4_DATA));
    core::mem::size_of::<u32>() as i32
}

unsafe extern "C" fn meson_rng_probe(pdev: *mut platform_device) -> i32 {
    let dev = &mut (*pdev).dev as *mut device;
    let data = devm_kzalloc(dev, core::mem::size_of::<meson_rng_data>(), 0) as *mut meson_rng_data;
    if data.is_null() { return -12; }
    let priv_data = device_get_match_data(dev) as *const meson_rng_priv;
    if priv_data.is_null() { return -19; }
    (*data).base = devm_platform_ioremap_resource(pdev, 0);
    (*data).rng.name = (*pdev).name;
    (*data).rng.read = (*priv_data).read;
    (*data).dev = dev;
    devm_hwrng_register(dev, &mut (*data).rng)
}

static meson_rng_priv: meson_rng_priv = meson_rng_priv { read: Some(meson_rng_read) };
static meson_rng_priv_s4: meson_rng_priv = meson_rng_priv { read: Some(meson_s4_rng_read) };

static meson_rng_of_match: [of_device_id; 3] = [
    of_device_id { compatible: c"amlogic,meson-rng".as_ptr(), data: &meson_rng_priv as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: c"amlogic,meson-s4-rng".as_ptr(), data: &meson_rng_priv_s4 as *const _ as *const core::ffi::c_void },
    of_device_id { compatible: core::ptr::null(), data: core::ptr::null() },
];

#[repr(C)]
pub struct driver {
    pub name: *const core::ffi::c_char,
    pub of_match_table: *const of_device_id,
}

#[repr(C)]
pub struct meson_rng_platform_driver {
    pub probe: Option<unsafe extern "C" fn(*mut platform_device) -> i32>,
    pub driver: driver,
}

static mut meson_rng_driver: meson_rng_platform_driver = meson_rng_platform_driver {
    probe: Some(meson_rng_probe),
    driver: driver { name: c"meson-rng".as_ptr(), of_match_table: meson_rng_of_match.as_ptr() },
};

// Equivalent kernel metadata and module_platform_driver registration:
// MODULE_DEVICE_TABLE(of, meson_rng_of_match);
// module_platform_driver(meson_rng_driver);
// MODULE_DESCRIPTION("Meson H/W Random Number Generator driver");
// MODULE_AUTHOR("Lawrence Mok <lawrence.mok@amlogic.com>");
// MODULE_AUTHOR("Neil Armstrong <narmstrong@baylibre.com>");
// MODULE_LICENSE("Dual BSD/GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
