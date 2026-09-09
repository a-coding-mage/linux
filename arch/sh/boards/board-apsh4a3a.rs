// SPDX-License-Identifier: GPL-2.0
/*
 * ALPHAPROJECT AP-SH4A-3A Support.
 *
 * Copyright (C) 2010 ALPHAPROJECT Co.,Ltd.
 * Copyright (C) 2008  Yoshihiro Shimoda
 * Copyright (C) 2009  Paul Mundt
 */

// Kernel dependencies supplied by other translation units are intentionally
// referenced here rather than reimplemented.

#[repr(C)]
pub struct mtd_partition {
    pub name: *const core::ffi::c_char,
    pub offset: usize,
    pub size: usize,
}

#[repr(C)]
pub struct physmap_flash_data {
    pub width: u32,
    pub parts: *mut mtd_partition,
    pub nr_parts: usize,
}

#[repr(C)]
pub struct resource {
    pub name: *const core::ffi::c_char,
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

#[repr(C)]
pub struct device {
    pub platform_data: *mut core::ffi::c_void,
}

#[repr(C)]
pub struct platform_device {
    pub name: *const core::ffi::c_char,
    pub id: i32,
    pub num_resources: usize,
    pub resource: *mut resource,
    pub dev: device,
}

#[repr(C)]
pub struct regulator_consumer_supply {
    pub supply: *const core::ffi::c_char,
    pub dev_name: *const core::ffi::c_char,
}

#[repr(C)]
pub struct smsc911x_platform_config {
    pub irq_polarity: u32,
    pub irq_type: u32,
    pub flags: u32,
    pub phy_interface: u32,
}

#[repr(C)]
pub struct clk;

unsafe extern "C" {
    fn regulator_register_fixed(
        id: i32,
        supplies: *mut regulator_consumer_supply,
        num_supplies: usize,
    ) -> i32;
    fn platform_add_devices(devices: *mut *mut platform_device, num: usize) -> i32;
    fn clk_get(dev: *mut core::ffi::c_void, id: *const core::ffi::c_char) -> *mut clk;
    fn clk_set_rate(clk: *mut clk, rate: u32) -> i32;
    fn clk_put(clk: *mut clk);
    fn plat_irq_setup_pins(mode: u32);
    fn printk(fmt: *const core::ffi::c_char, ...);
}

const MTDPART_OFS_APPEND: usize = usize::MAX;
const MTDPART_SIZ_FULL: usize = usize::MAX;
const IORESOURCE_MEM: u64 = 0x0000_0200;
const IORESOURCE_IRQ: u64 = 0x0000_0400;
const SZ_256: usize = 256;
const SMSC911X_IRQ_POLARITY_ACTIVE_LOW: u32 = 0;
const SMSC911X_IRQ_TYPE_OPEN_DRAIN: u32 = 0;
const SMSC911X_USE_16BIT: u32 = 1;
const PHY_INTERFACE_MODE_MII: u32 = 0;
const IRQ_MODE_IRQ7654: u32 = 0;

const MODE_PIN0: i32 = 1 << 0;
const MODE_PIN1: i32 = 1 << 1;
const MODE_PIN2: i32 = 1 << 2;
const MODE_PIN3: i32 = 1 << 3;
const MODE_PIN4: i32 = 1 << 4;
const MODE_PIN5: i32 = 1 << 5;
const MODE_PIN6: i32 = 1 << 6;
const MODE_PIN7: i32 = 1 << 7;
const MODE_PIN8: i32 = 1 << 8;
const MODE_PIN9: i32 = 1 << 9;
const MODE_PIN10: i32 = 1 << 10;
const MODE_PIN11: i32 = 1 << 11;
const MODE_PIN12: i32 = 1 << 12;
const MODE_PIN13: i32 = 1 << 13;
const MODE_PIN14: i32 = 1 << 14;

static mut nor_flash_partitions: [mtd_partition; 4] = [
    mtd_partition { name: b"loader\0".as_ptr() as _, offset: 0x00000000, size: 512 * 1024 },
    mtd_partition { name: b"bootenv\0".as_ptr() as _, offset: MTDPART_OFS_APPEND, size: 512 * 1024 },
    mtd_partition { name: b"kernel\0".as_ptr() as _, offset: MTDPART_OFS_APPEND, size: 4 * 1024 * 1024 },
    mtd_partition { name: b"data\0".as_ptr() as _, offset: MTDPART_OFS_APPEND, size: MTDPART_SIZ_FULL },
];

static mut nor_flash_data: physmap_flash_data = physmap_flash_data {
    width: 4,
    parts: core::ptr::null_mut(),
    nr_parts: 4,
};

static mut nor_flash_resources: [resource; 1] = [resource {
    name: core::ptr::null(), start: 0x00000000, end: 0x01000000 - 1, flags: IORESOURCE_MEM,
}];

static mut nor_flash_device: platform_device = platform_device {
    name: b"physmap-flash\0".as_ptr() as _, id: 0, num_resources: 1,
    resource: core::ptr::null_mut(), dev: device { platform_data: core::ptr::null_mut() },
};

/* Dummy supplies, where voltage doesn't matter */
static mut dummy_supplies: [regulator_consumer_supply; 2] = [
    regulator_consumer_supply { supply: b"vddvario\0".as_ptr() as _, dev_name: b"smsc911x\0".as_ptr() as _ },
    regulator_consumer_supply { supply: b"vdd33a\0".as_ptr() as _, dev_name: b"smsc911x\0".as_ptr() as _ },
];

static mut smsc911x_resources: [resource; 2] = [
    resource { name: b"smsc911x-memory\0".as_ptr() as _, start: 0xA4000000, end: 0xA4000000 + SZ_256 - 1, flags: IORESOURCE_MEM },
    resource { name: b"smsc911x-irq\0".as_ptr() as _, start: 0x200, end: 0x200, flags: IORESOURCE_IRQ },
];

static mut smsc911x_config: smsc911x_platform_config = smsc911x_platform_config {
    irq_polarity: SMSC911X_IRQ_POLARITY_ACTIVE_LOW,
    irq_type: SMSC911X_IRQ_TYPE_OPEN_DRAIN,
    flags: SMSC911X_USE_16BIT,
    phy_interface: PHY_INTERFACE_MODE_MII,
};

static mut smsc911x_device: platform_device = platform_device {
    name: b"smsc911x\0".as_ptr() as _, id: -1, num_resources: 2,
    resource: core::ptr::null_mut(), dev: device { platform_data: core::ptr::null_mut() },
};

static mut apsh4a3a_devices: [*mut platform_device; 2] = [
    core::ptr::null_mut(), core::ptr::null_mut(),
];

unsafe fn apsh4a3a_devices_setup() -> i32 {
    regulator_register_fixed(0, dummy_supplies.as_mut_ptr(), dummy_supplies.len());
    platform_add_devices(apsh4a3a_devices.as_mut_ptr(), apsh4a3a_devices.len())
}

unsafe fn apsh4a3a_clk_init() -> i32 {
    let clk = clk_get(core::ptr::null_mut(), b"extal\0".as_ptr() as _);
    if clk.is_null() { return -1; }
    let ret = clk_set_rate(clk, 33333000);
    clk_put(clk);
    ret
}

/* Initialize the board */
unsafe fn apsh4a3a_setup(_cmdline_p: *mut *mut core::ffi::c_char) {
    printk(b"Alpha Project AP-SH4A-3A support:\n\0".as_ptr() as _);
}

unsafe fn apsh4a3a_init_irq() {
    plat_irq_setup_pins(IRQ_MODE_IRQ7654);
}

/* Return the board specific boot mode pin configuration */
fn apsh4a3a_mode_pins() -> i32 {
    let mut value = 0;
    value &= !MODE_PIN0; value &= !MODE_PIN1; value &= !MODE_PIN2; value &= !MODE_PIN3;
    value |= MODE_PIN4; value &= !MODE_PIN5; value |= MODE_PIN6; value |= MODE_PIN7;
    value |= MODE_PIN8; value |= MODE_PIN9; value |= MODE_PIN10; value |= MODE_PIN11;
    value |= MODE_PIN12; value &= !MODE_PIN13; value |= MODE_PIN14;
    value
}

/* The Machine Vector */
#[repr(C)]
struct sh_machine_vector {
    mv_name: *const core::ffi::c_char,
    mv_setup: unsafe fn(*mut *mut core::ffi::c_char),
    mv_clk_init: unsafe fn() -> i32,
    mv_init_irq: unsafe fn(),
    mv_mode_pins: fn() -> i32,
}

static mut mv_apsh4a3a: sh_machine_vector = sh_machine_vector {
    mv_name: b"AP-SH4A-3A\0".as_ptr() as _,
    mv_setup: apsh4a3a_setup,
    mv_clk_init: apsh4a3a_clk_init,
    mv_init_irq: apsh4a3a_init_irq,
    mv_mode_pins: apsh4a3a_mode_pins,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
