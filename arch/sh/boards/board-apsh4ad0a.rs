// SPDX-License-Identifier: GPL-2.0
/*
 * ALPHAPROJECT AP-SH4AD-0A Support.
 *
 * Copyright (C) 2010 ALPHAPROJECT Co.,Ltd.
 * Copyright (C) 2010  Matt Fleming
 * Copyright (C) 2010  Paul Mundt
 */

use core::ffi::{c_char, c_int, c_void};

// Kernel-provided types, constants, and functions from the original includes.
#[repr(C)]
pub struct RegulatorConsumerSupply {
    pub supply: *const c_char,
    pub dev_name: *const c_char,
}

#[repr(C)]
pub struct Resource {
    pub name: *const c_char,
    pub start: usize,
    pub end: usize,
    pub flags: u64,
}

#[repr(C)]
pub struct Smsc911xPlatformConfig {
    pub irq_polarity: c_int,
    pub irq_type: c_int,
    pub flags: c_int,
    pub phy_interface: c_int,
}

#[repr(C)]
pub struct Device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct PlatformDevice {
    pub name: *const c_char,
    pub id: c_int,
    pub num_resources: usize,
    pub resource: *mut Resource,
    pub dev: Device,
}

#[repr(C)]
pub struct Clk {
    _private: [u8; 0],
}

extern "C" {
    fn evt2irq(event: c_int) -> c_int;
    fn regulator_register_fixed(
        id: c_int,
        supplies: *mut RegulatorConsumerSupply,
        count: usize,
    ) -> c_int;
    fn platform_add_devices(devices: *mut *mut PlatformDevice, count: usize) -> c_int;
    fn clk_get(dev: *mut c_void, name: *const c_char) -> *mut Clk;
    fn clk_set_rate(clk: *mut Clk, rate: u32) -> c_int;
    fn clk_put(clk: *mut Clk);
    fn plat_irq_setup_pins(mode: c_int);
    fn pr_info(format: *const c_char);
    fn is_err(ptr: *mut c_void) -> bool;
    fn ptr_err(ptr: *mut c_void) -> c_int;
}

const IORESOURCE_MEM: u64 = 0x0000_0200;
const IORESOURCE_IRQ: u64 = 0x0000_0400;
const SZ_256: usize = 256;
const SMSC911X_IRQ_POLARITY_ACTIVE_LOW: c_int = 0;
const SMSC911X_IRQ_TYPE_OPEN_DRAIN: c_int = 0;
const SMSC911X_USE_16BIT: c_int = 1;
const PHY_INTERFACE_MODE_MII: c_int = 0;
const IRQ_MODE_IRQ3210: c_int = 0;
const MODE_PIN0: c_int = 1 << 0;
const MODE_PIN1: c_int = 1 << 1;
const MODE_PIN2: c_int = 1 << 2;
const MODE_PIN3: c_int = 1 << 3;
const MODE_PIN4: c_int = 1 << 4;
const MODE_PIN5: c_int = 1 << 5;
const MODE_PIN6: c_int = 1 << 6;
const MODE_PIN7: c_int = 1 << 7;
const MODE_PIN8: c_int = 1 << 8;
const MODE_PIN9: c_int = 1 << 9;
const MODE_PIN10: c_int = 1 << 10;
const MODE_PIN11: c_int = 1 << 11;
const MODE_PIN12: c_int = 1 << 12;
const MODE_PIN13: c_int = 1 << 13;
const MODE_PIN14: c_int = 1 << 14;

const VDDVARIO: &[u8] = b"vddvario\0";
const VDD33A: &[u8] = b"vdd33a\0";
const SMSC911X: &[u8] = b"smsc911x\0";
const SMSC911X_NAME: &[u8] = b"smsc911x\0";
const SMSC911X_MEMORY: &[u8] = b"smsc911x-memory\0";
const SMSC911X_IRQ: &[u8] = b"smsc911x-irq\0";

/* Dummy supplies, where voltage doesn't matter */
static mut DUMMY_SUPPLIES: [RegulatorConsumerSupply; 2] = [
    RegulatorConsumerSupply { supply: VDDVARIO.as_ptr() as *const c_char, dev_name: SMSC911X.as_ptr() as *const c_char },
    RegulatorConsumerSupply { supply: VDD33A.as_ptr() as *const c_char, dev_name: SMSC911X.as_ptr() as *const c_char },
];

static mut SMSC911X_RESOURCES: [Resource; 2] = [
    Resource { name: SMSC911X_MEMORY.as_ptr() as *const c_char, start: 0xA4000000, end: 0xA4000000 + SZ_256 - 1, flags: IORESOURCE_MEM },
    Resource { name: SMSC911X_IRQ.as_ptr() as *const c_char, start: 0, end: 0, flags: IORESOURCE_IRQ },
];

static mut SMSC911X_CONFIG: Smsc911xPlatformConfig = Smsc911xPlatformConfig {
    irq_polarity: SMSC911X_IRQ_POLARITY_ACTIVE_LOW,
    irq_type: SMSC911X_IRQ_TYPE_OPEN_DRAIN,
    flags: SMSC911X_USE_16BIT,
    phy_interface: PHY_INTERFACE_MODE_MII,
};

static mut SMSC911X_DEVICE: PlatformDevice = PlatformDevice {
    name: SMSC911X_NAME.as_ptr() as *const c_char,
    id: -1,
    num_resources: 2,
    resource: core::ptr::null_mut(),
    dev: Device { platform_data: core::ptr::null_mut() },
};

static mut APSH4AD0A_DEVICES: [*mut PlatformDevice; 1] = [core::ptr::null_mut()];

unsafe fn apsh4ad0a_devices_setup() -> c_int {
    regulator_register_fixed(0, DUMMY_SUPPLIES.as_mut_ptr(), DUMMY_SUPPLIES.len());
    platform_add_devices(APSH4AD0A_DEVICES.as_mut_ptr(), APSH4AD0A_DEVICES.len())
}
// device_initcall(apsh4ad0a_devices_setup);

unsafe fn apsh4ad0a_mode_pins() -> c_int {
    let mut value: c_int = 0;
    /* These are the factory default settings of SW1 and SW2.
     * If you change these dip switches then you will need to
     * adjust the values below as well.
     */
    value |= MODE_PIN0; value |= MODE_PIN1; value &= !MODE_PIN2; value &= !MODE_PIN3;
    value &= !MODE_PIN4; value |= MODE_PIN5; value |= MODE_PIN6; value |= MODE_PIN7;
    value |= MODE_PIN8; value |= MODE_PIN9; value &= !MODE_PIN10; value &= !MODE_PIN11;
    value &= !MODE_PIN12; value |= MODE_PIN13; value &= !MODE_PIN14;
    value
}

unsafe fn apsh4ad0a_clk_init() -> c_int {
    let clk = clk_get(core::ptr::null_mut(), b"extal\0".as_ptr() as *const c_char);
    if is_err(clk as *mut c_void) { return ptr_err(clk as *mut c_void); }
    let ret = clk_set_rate(clk, 33333000);
    clk_put(clk);
    ret
}

/* Initialize the board */
unsafe fn apsh4ad0a_setup(_cmdline_p: *mut *mut c_char) {
    pr_info(b"Alpha Project AP-SH4AD-0A support:\n\0".as_ptr() as *const c_char);
}

unsafe fn apsh4ad0a_init_irq() { plat_irq_setup_pins(IRQ_MODE_IRQ3210); }

/* The Machine Vector */
#[repr(C)]
struct ShMachineVector {
    mv_name: *const c_char,
    mv_setup: unsafe fn(*mut *mut c_char),
    mv_mode_pins: unsafe fn() -> c_int,
    mv_clk_init: unsafe fn() -> c_int,
    mv_init_irq: unsafe fn(),
}

static mut MV_APSH4AD0A: ShMachineVector = ShMachineVector {
    mv_name: b"AP-SH4AD-0A\0".as_ptr() as *const c_char,
    mv_setup: apsh4ad0a_setup,
    mv_mode_pins: apsh4ad0a_mode_pins,
    mv_clk_init: apsh4ad0a_clk_init,
    mv_init_irq: apsh4ad0a_init_irq,
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
