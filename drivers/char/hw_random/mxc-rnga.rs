// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * RNG driver for Freescale RNGA
 *
 * Copyright 2008-2009 Freescale Semiconductor, Inc. All Rights Reserved.
 * Author: Alan Carvalho de Assis <acassis@gmail.com>
 */

/* This driver is based on other RNG drivers. */

/* Linux kernel dependencies supplied by the surrounding translation unit. */
use core::ffi::c_void;

const RNGA_CONTROL: usize = 0x00;
const RNGA_STATUS: usize = 0x04;
const RNGA_ENTROPY: usize = 0x08;
const RNGA_OUTPUT_FIFO: usize = 0x0c;
const RNGA_MODE: usize = 0x10;
const RNGA_VERIFICATION_CONTROL: usize = 0x14;
const RNGA_OSC_CONTROL_COUNTER: usize = 0x18;
const RNGA_OSC1_COUNTER: usize = 0x1c;
const RNGA_OSC2_COUNTER: usize = 0x20;
const RNGA_OSC_COUNTER_STATUS: usize = 0x24;
const RNG_ADDR_RANGE: usize = 0x28;

const RNGA_CONTROL_SLEEP: u32 = 0x00000010;
const RNGA_CONTROL_CLEAR_INT: u32 = 0x00000008;
const RNGA_CONTROL_MASK_INTS: u32 = 0x00000004;
const RNGA_CONTROL_HIGH_ASSURANCE: u32 = 0x00000002;
const RNGA_CONTROL_GO: u32 = 0x00000001;
const RNGA_STATUS_LEVEL_MASK: u32 = 0x0000ff00;
const RNGA_STATUS_OSC_DEAD: u32 = 0x80000000;
const RNGA_STATUS_SLEEP: u32 = 0x00000010;
const RNGA_STATUS_ERROR_INT: u32 = 0x00000008;
const RNGA_STATUS_FIFO_UNDERFLOW: u32 = 0x00000004;
const RNGA_STATUS_LAST_READ_STATUS: u32 = 0x00000002;
const RNGA_STATUS_SECURITY_VIOLATION: u32 = 0x00000001;

#[repr(C)]
pub struct device { _private: [u8; 0] }
#[repr(C)]
pub struct clk { _private: [u8; 0] }
#[repr(C)]
pub struct platform_device { pub dev: device }
#[repr(C)]
pub struct hwrng {
    pub name: *const u8,
    pub init: Option<unsafe extern "C" fn(*mut hwrng) -> i32>,
    pub cleanup: Option<unsafe extern "C" fn(*mut hwrng)>,
    pub data_present: Option<unsafe extern "C" fn(*mut hwrng, i32) -> i32>,
    pub data_read: Option<unsafe extern "C" fn(*mut hwrng, *mut u32) -> i32>,
}

#[repr(C)]
pub struct mxc_rng {
    pub dev: *mut device,
    pub rng: hwrng,
    pub mem: *mut c_void,
    pub clk: *mut clk,
}

extern "C" {
    fn __raw_readl(addr: *mut c_void) -> u32;
    fn __raw_writel(value: u32, addr: *mut c_void);
    fn udelay(usecs: u32);
    fn dev_dbg(dev: *mut device, fmt: *const u8, ...);
    fn dev_err(dev: *mut device, fmt: *const u8, ...);
    fn container_of<T, U>(ptr: *mut T, member: *const U) -> *mut mxc_rng;
    fn devm_kzalloc(dev: *mut device, size: usize, flags: u32) -> *mut c_void;
    fn devm_clk_get_enabled(dev: *mut device, id: *const u8) -> *mut clk;
    fn devm_platform_ioremap_resource(pdev: *mut platform_device, index: u32) -> *mut c_void;
    fn hwrng_register(rng: *mut hwrng) -> i32;
    fn hwrng_unregister(rng: *mut hwrng);
    fn platform_get_drvdata(pdev: *mut platform_device) -> *mut mxc_rng;
}

unsafe fn mxc_rnga_data_present(rng: *mut hwrng, wait: i32) -> i32 {
    let mxc_rng = container_of(rng, core::ptr::null());
    for _i in 0..20 {
        let level = (__raw_readl((*mxc_rng).mem.byte_add(RNGA_STATUS)) & RNGA_STATUS_LEVEL_MASK) >> 8;
        if level != 0 || wait == 0 { return (level != 0) as i32; }
        udelay(10);
    }
    0
}

unsafe fn mxc_rnga_data_read(rng: *mut hwrng, data: *mut u32) -> i32 {
    let mxc_rng = container_of(rng, core::ptr::null());
    *data = __raw_readl((*mxc_rng).mem.byte_add(RNGA_OUTPUT_FIFO));
    let err = __raw_readl((*mxc_rng).mem.byte_add(RNGA_STATUS)) & RNGA_STATUS_ERROR_INT;
    if err != 0 {
        dev_dbg((*mxc_rng).dev, b"Error while reading random number!\0".as_ptr());
        let ctrl = __raw_readl((*mxc_rng).mem.byte_add(RNGA_CONTROL));
        __raw_writel(ctrl | RNGA_CONTROL_CLEAR_INT, (*mxc_rng).mem.byte_add(RNGA_CONTROL));
        0
    } else { 4 }
}

unsafe fn mxc_rnga_init(rng: *mut hwrng) -> i32 {
    let mxc_rng = container_of(rng, core::ptr::null());
    let mut ctrl = __raw_readl((*mxc_rng).mem.byte_add(RNGA_CONTROL));
    __raw_writel(ctrl & !RNGA_CONTROL_SLEEP, (*mxc_rng).mem.byte_add(RNGA_CONTROL));
    let osc = __raw_readl((*mxc_rng).mem.byte_add(RNGA_STATUS));
    if osc & RNGA_STATUS_OSC_DEAD != 0 {
        dev_err((*mxc_rng).dev, b"RNGA Oscillator is dead!\0".as_ptr());
        return -19;
    }
    ctrl = __raw_readl((*mxc_rng).mem.byte_add(RNGA_CONTROL));
    __raw_writel(ctrl | RNGA_CONTROL_GO, (*mxc_rng).mem.byte_add(RNGA_CONTROL));
    0
}

unsafe fn mxc_rnga_cleanup(rng: *mut hwrng) {
    let mxc_rng = container_of(rng, core::ptr::null());
    let ctrl = __raw_readl((*mxc_rng).mem.byte_add(RNGA_CONTROL));
    __raw_writel(ctrl & !RNGA_CONTROL_GO, (*mxc_rng).mem.byte_add(RNGA_CONTROL));
}

unsafe fn mxc_rnga_probe(pdev: *mut platform_device) -> i32 {
    let mxc_rng = devm_kzalloc(&mut (*pdev).dev, core::mem::size_of::<mxc_rng>(), 0) as *mut mxc_rng;
    if mxc_rng.is_null() { return -12; }
    (*mxc_rng).dev = &mut (*pdev).dev;
    (*mxc_rng).rng.name = b"mxc-rnga\0".as_ptr();
    (*mxc_rng).rng.init = Some(mxc_rnga_init);
    (*mxc_rng).rng.cleanup = Some(mxc_rnga_cleanup);
    (*mxc_rng).rng.data_present = Some(mxc_rnga_data_present);
    (*mxc_rng).rng.data_read = Some(mxc_rnga_data_read);
    (*mxc_rng).clk = devm_clk_get_enabled(&mut (*pdev).dev, core::ptr::null());
    if (*mxc_rng).clk.is_null() {
        dev_err(&mut (*pdev).dev, b"Could not get rng_clk!\n\0".as_ptr());
        return -1;
    }
    (*mxc_rng).mem = devm_platform_ioremap_resource(pdev, 0);
    if (*mxc_rng).mem.is_null() { return -1; }
    let err = hwrng_register(&mut (*mxc_rng).rng);
    if err != 0 {
        dev_err(&mut (*pdev).dev, b"MXC RNGA registering failed (%d)\n\0".as_ptr(), err);
        return err;
    }
    0
}

unsafe fn mxc_rnga_remove(pdev: *mut platform_device) {
    let mxc_rng = platform_get_drvdata(pdev);
    hwrng_unregister(&mut (*mxc_rng).rng);
}

#[repr(C)]
struct of_device_id { compatible: *const u8 }

static MXC_RNGA_OF_MATCH: [of_device_id; 3] = [
    of_device_id { compatible: b"fsl,imx21-rnga\0".as_ptr() },
    of_device_id { compatible: b"fsl,imx31-rnga\0".as_ptr() },
    of_device_id { compatible: core::ptr::null() },
];

#[repr(C)]
struct platform_driver {
    name: *const u8,
    of_match_table: *const of_device_id,
    probe: Option<unsafe fn(*mut platform_device) -> i32>,
    remove: Option<unsafe fn(*mut platform_device)>,
}

static mut mxc_rnga_driver: platform_driver = platform_driver {
    name: b"mxc_rnga\0".as_ptr(),
    of_match_table: MXC_RNGA_OF_MATCH.as_ptr(),
    probe: Some(mxc_rnga_probe),
    remove: Some(mxc_rnga_remove),
};

// module_platform_driver(mxc_rnga_driver);
// MODULE_DEVICE_TABLE(of, mxc_rnga_of_match);
// MODULE_AUTHOR("Freescale Semiconductor, Inc.");
// MODULE_DESCRIPTION("H/W RNGA driver for i.MX");
// MODULE_LICENSE("GPL");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
