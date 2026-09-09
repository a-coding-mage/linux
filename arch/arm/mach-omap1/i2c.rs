// SPDX-License-Identifier: GPL-2.0-only
/*
 * Helper module for board specific I2C bus registration
 *
 * Copyright (C) 2009 Nokia Corporation.
 */

// Dependencies supplied by the surrounding kernel translation.

use core::ffi::{c_char, c_int, c_uint, c_void};

pub const OMAP_I2C_SIZE: u32 = 0x3f;
pub const OMAP1_I2C_BASE: u32 = 0xfffb3800;
pub const OMAP_I2C_MAX_CONTROLLERS: usize = 4;
pub const OMAP_I2C_CMDLINE_SETUP: u32 = 1u32 << 31;

pub const EINVAL: c_int = 22;
pub const IORESOURCE_MEM: u32 = 0x0000_0200;
pub const IORESOURCE_IRQ: u32 = 0x0000_0400;
pub const OMAP_I2C_IP_VERSION_1: u32 = 1;
pub const OMAP_I2C_FLAG_NO_FIFO: u32 = 1 << 0;
pub const OMAP_I2C_FLAG_SIMPLE_CLOCK: u32 = 1 << 1;
pub const OMAP_I2C_FLAG_16BIT_DATA_REG: u32 = 1 << 2;
pub const OMAP_I2C_FLAG_ALWAYS_ARMXOR_CLK: u32 = 1 << 3;
pub const OMAP_I2C_FLAG_BUS_SHIFT_2: u32 = 1 << 4;

#[repr(C)]
pub struct Resource {
    pub start: u32,
    pub end: u32,
    pub flags: u32,
}

#[repr(C)]
pub struct Device {
    pub platform_data: *mut c_void,
}

#[repr(C)]
pub struct PlatformDevice {
    pub id: c_int,
    pub name: *const c_char,
    pub num_resources: usize,
    pub resource: *mut Resource,
    pub dev: Device,
}

#[repr(C)]
pub struct OmapI2cBusPlatformData {
    pub rev: u32,
    pub flags: u32,
    pub clkrate: u32,
}

#[repr(C)]
pub struct I2cBoardInfo {
    _opaque: [u8; 0],
}

unsafe extern "C" {
    fn omap_cfg_reg(reg: c_int);
    fn platform_device_register(pdev: *mut PlatformDevice) -> c_int;
    fn get_options(str_: *mut c_char, nints: c_int, ints: *mut c_int) -> c_int;
    fn i2c_register_board_info(
        busnum: c_int,
        info: *const I2cBoardInfo,
        len: usize,
    ) -> c_int;
    fn BUG_ON(condition: bool);
}

unsafe extern "C" {
    static I2C_SDA: c_int;
    static I2C_SCL: c_int;
    static INT_I2C: u32;
}

static NAME: &[u8] = b"omap_i2c\0";
static mut I2C_RESOURCES: [Resource; 2] = [
    Resource { start: 0, end: 0, flags: 0 },
    Resource { start: 0, end: 0, flags: 0 },
];

static mut OMAP_I2C_DEVICES: [PlatformDevice; 1] = [PlatformDevice {
    id: 0,
    name: core::ptr::null(),
    num_resources: 0,
    resource: core::ptr::null_mut(),
    dev: Device { platform_data: core::ptr::null_mut() },
}];

unsafe fn omap1_i2c_mux_pins(_bus_id: c_int) {
    omap_cfg_reg(I2C_SDA);
    omap_cfg_reg(I2C_SCL);
}

pub unsafe fn omap_i2c_add_bus(
    pdata: *mut OmapI2cBusPlatformData,
    bus_id: c_int,
) -> c_int {
    if bus_id > 1 {
        return -EINVAL;
    }

    omap1_i2c_mux_pins(bus_id);

    let pdev = &mut OMAP_I2C_DEVICES[bus_id.wrapping_sub(1) as usize];
    pdev.id = bus_id;
    pdev.name = NAME.as_ptr() as *const c_char;
    pdev.num_resources = I2C_RESOURCES.len();
    let res = I2C_RESOURCES.as_mut_ptr();
    (*res.add(0)).start = OMAP1_I2C_BASE;
    (*res.add(0)).end = (*res.add(0)).start.wrapping_add(OMAP_I2C_SIZE);
    (*res.add(0)).flags = IORESOURCE_MEM;
    (*res.add(1)).start = INT_I2C;
    (*res.add(1)).flags = IORESOURCE_IRQ;
    pdev.resource = res;

    // all OMAP1 have IP version 1 register set
    (*pdata).rev = OMAP_I2C_IP_VERSION_1;

    // all OMAP1 I2C are implemented like this
    (*pdata).flags = OMAP_I2C_FLAG_NO_FIFO |
        OMAP_I2C_FLAG_SIMPLE_CLOCK |
        OMAP_I2C_FLAG_16BIT_DATA_REG |
        OMAP_I2C_FLAG_ALWAYS_ARMXOR_CLK;

    // how the cpu bus is wired up differs for 7xx only
    (*pdata).flags |= OMAP_I2C_FLAG_BUS_SHIFT_2;
    pdev.dev.platform_data = pdata as *mut c_void;

    platform_device_register(pdev)
}

static mut I2C_PDATA: [OmapI2cBusPlatformData; OMAP_I2C_MAX_CONTROLLERS] = [
    OmapI2cBusPlatformData { rev: 0, flags: 0, clkrate: 0 },
    OmapI2cBusPlatformData { rev: 0, flags: 0, clkrate: 0 },
    OmapI2cBusPlatformData { rev: 0, flags: 0, clkrate: 0 },
    OmapI2cBusPlatformData { rev: 0, flags: 0, clkrate: 0 },
];

/**
 * omap_i2c_bus_setup - Process command line options for the I2C bus speed
 * @str: String of options
 *
 * This function allow to override the default I2C bus speed for given I2C
 * bus with a command line option.
 *
 * Format: i2c_bus=bus_id,clkrate (in kHz)
 *
 * Returns 1 on success, 0 otherwise.
 */
unsafe fn omap_i2c_bus_setup(str_: *mut c_char) -> c_int {
    let mut ints = [0; 3];

    get_options(str_, 3, ints.as_mut_ptr());
    if ints[0] < 2 || ints[1] < 1 || ints[1] > OMAP_I2C_MAX_CONTROLLERS as c_int {
        return 0;
    }
    I2C_PDATA[(ints[1] - 1) as usize].clkrate = ints[2] as u32;
    I2C_PDATA[(ints[1] - 1) as usize].clkrate |= OMAP_I2C_CMDLINE_SETUP;

    1
}

// __setup("i2c_bus=", omap_i2c_bus_setup);

/*
 * Register busses defined in command line but that are not registered with
 * omap_register_i2c_bus from board initialization code.
 */
pub unsafe fn omap_register_i2c_bus_cmdline() -> c_int {
    let mut err = 0;

    for i in 0..I2C_PDATA.len() {
        if I2C_PDATA[i].clkrate & OMAP_I2C_CMDLINE_SETUP != 0 {
            I2C_PDATA[i].clkrate &= !OMAP_I2C_CMDLINE_SETUP;
            err = omap_i2c_add_bus(&mut I2C_PDATA[i], i as c_int + 1);
            if err != 0 {
                break;
            }
        }
    }

    err
}

/**
 * omap_register_i2c_bus - register I2C bus with device descriptors
 * @bus_id: bus id counting from number 1
 * @clkrate: clock rate of the bus in kHz
 * @info: pointer into I2C device descriptor table or NULL
 * @len: number of descriptors in the table
 *
 * Returns 0 on success or an error code.
 */
pub unsafe fn omap_register_i2c_bus(
    bus_id: c_int,
    clkrate: u32,
    info: *const I2cBoardInfo,
    len: usize,
) -> c_int {
    BUG_ON(bus_id < 1 || bus_id > OMAP_I2C_MAX_CONTROLLERS as c_int);

    if !info.is_null() {
        let err = i2c_register_board_info(bus_id, info, len);
        if err != 0 {
            return err;
        }
    }

    if I2C_PDATA[(bus_id - 1) as usize].clkrate == 0 {
        I2C_PDATA[(bus_id - 1) as usize].clkrate = clkrate;
    }
    I2C_PDATA[(bus_id - 1) as usize].clkrate &= !OMAP_I2C_CMDLINE_SETUP;

    omap_i2c_add_bus(&mut I2C_PDATA[(bus_id - 1) as usize], bus_id)
}

unsafe fn omap_i2c_cmdline() -> c_int {
    omap_register_i2c_bus_cmdline()
}

// subsys_initcall(omap_i2c_cmdline);

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
