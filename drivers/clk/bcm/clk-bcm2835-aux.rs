// SPDX-License-Identifier: GPL-2.0+
/*
 * Copyright (C) 2015 Broadcom
 */

// Dependencies supplied by the kernel headers in the original implementation
// are referenced here as external Rust declarations.

use core::ffi::{c_char, c_int, c_uint, c_void};

const BCM2835_AUXIRQ: usize = 0x00;
const BCM2835_AUXENB: usize = 0x04;

// Generated from <dt-bindings/clock/bcm2835-aux.h>.
extern "C" {
    static BCM2835_AUX_CLOCK_COUNT: c_uint;
    static BCM2835_AUX_CLOCK_UART: c_uint;
    static BCM2835_AUX_CLOCK_SPI1: c_uint;
    static BCM2835_AUX_CLOCK_SPI2: c_uint;
}

#[repr(C)]
struct device {
    _private: [u8; 0],
}

#[repr(C)]
struct device_node {
    _private: [u8; 0],
}

#[repr(C)]
struct platform_device {
    dev: device,
    of_node: *mut device_node,
}

#[repr(C)]
struct clk;

#[repr(C)]
struct clk_hw;

#[repr(C)]
struct clk_hw_onecell_data {
    num: c_uint,
    hws: [*mut clk_hw; 0],
}

#[repr(C)]
struct of_device_id {
    compatible: *const c_char,
}

#[repr(C)]
struct device_driver {
    name: *const c_char,
    of_match_table: *const of_device_id,
}

#[repr(C)]
struct platform_driver {
    driver: device_driver,
    probe: Option<unsafe extern "C" fn(*mut platform_device) -> c_int>,
}

extern "C" {
    fn devm_clk_get(dev: *mut device, id: *const c_char) -> *mut clk;
    fn __clk_get_name(clk: *mut clk) -> *const c_char;
    fn devm_platform_ioremap_resource(
        pdev: *mut platform_device,
        index: c_uint,
    ) -> *mut c_void;
    fn devm_kmalloc(dev: *mut device, size: usize, flags: c_uint) -> *mut c_void;
    fn clk_hw_register_gate(
        dev: *mut device,
        name: *const c_char,
        parent_name: *const c_char,
        flags: c_ulong,
        reg: *mut c_void,
        bit_idx: u8,
        flags2: u8,
        lock: *mut c_void,
    ) -> *mut clk_hw;
    fn of_clk_add_hw_provider(
        node: *mut device_node,
        get: *const c_void,
        data: *mut clk_hw_onecell_data,
    ) -> c_int;
    fn of_clk_hw_onecell_get() -> !;
    fn platform_driver_register(driver: *mut platform_driver) -> c_int;
}

type c_ulong = usize;

const GFP_KERNEL: c_uint = 0;

#[inline]
unsafe fn is_err<T>(ptr: *mut T) -> bool {
    (ptr as isize) < 0
}

#[inline]
unsafe fn ptr_err<T>(ptr: *mut T) -> c_int {
    ptr as isize as c_int
}

unsafe extern "C" fn bcm2835_aux_clk_probe(pdev: *mut platform_device) -> c_int {
    let dev: *mut device = &mut (*pdev).dev;
    let mut onecell: *mut clk_hw_onecell_data;
    let parent: *const c_char;
    let parent_clk: *mut clk;
    let reg: *mut c_void;
    let gate: *mut c_void;

    parent_clk = devm_clk_get(dev, core::ptr::null());
    if is_err(parent_clk) {
        return ptr_err(parent_clk);
    }
    parent = __clk_get_name(parent_clk);

    reg = devm_platform_ioremap_resource(pdev, 0);
    if is_err(reg) {
        return ptr_err(reg);
    }

    onecell = devm_kmalloc(
        dev,
        core::mem::size_of::<clk_hw_onecell_data>()
            + core::mem::size_of::<*mut clk_hw>() * BCM2835_AUX_CLOCK_COUNT as usize,
        GFP_KERNEL,
    ) as *mut clk_hw_onecell_data;
    if onecell.is_null() {
        return -12;
    }
    (*onecell).num = BCM2835_AUX_CLOCK_COUNT;

    gate = (reg as *mut u8).add(BCM2835_AUXENB) as *mut c_void;
    (*onecell).hws.as_mut_ptr().add(BCM2835_AUX_CLOCK_UART as usize).write(
        clk_hw_register_gate(dev, b"aux_uart\0".as_ptr() as *const c_char, parent, 0, gate, 0, 0, core::ptr::null_mut()),
    );

    (*onecell).hws.as_mut_ptr().add(BCM2835_AUX_CLOCK_SPI1 as usize).write(
        clk_hw_register_gate(dev, b"aux_spi1\0".as_ptr() as *const c_char, parent, 0, gate, 1, 0, core::ptr::null_mut()),
    );

    (*onecell).hws.as_mut_ptr().add(BCM2835_AUX_CLOCK_SPI2 as usize).write(
        clk_hw_register_gate(dev, b"aux_spi2\0".as_ptr() as *const c_char, parent, 0, gate, 2, 0, core::ptr::null_mut()),
    );

    of_clk_add_hw_provider(
        (*pdev).of_node,
        of_clk_hw_onecell_get as *const c_void,
        onecell,
    )
}

static BCM2835_AUX_CLK_OF_MATCH: [of_device_id; 2] = [
    of_device_id {
        compatible: b"brcm,bcm2835-aux\0".as_ptr() as *const c_char,
    },
    of_device_id {
        compatible: core::ptr::null(),
    },
];

static mut BCM2835_AUX_CLK_DRIVER: platform_driver = platform_driver {
    driver: device_driver {
        name: b"bcm2835-aux-clk\0".as_ptr() as *const c_char,
        of_match_table: BCM2835_AUX_CLK_OF_MATCH.as_ptr(),
    },
    probe: Some(bcm2835_aux_clk_probe),
};

#[used]
#[allow(non_upper_case_globals)]
static __BUILTIN_PLATFORM_DRIVER: unsafe extern "C" fn(*mut platform_driver) -> c_int =
    platform_driver_register;

// MODULE_DEVICE_TABLE(of, bcm2835_aux_clk_of_match);
// builtin_platform_driver(bcm2835_aux_clk_driver);
// MODULE_AUTHOR("Eric Anholt <eric@anholt.net>");
// MODULE_DESCRIPTION("BCM2835 auxiliary peripheral clock driver");

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
