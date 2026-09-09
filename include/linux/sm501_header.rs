/* SPDX-License-Identifier: GPL-2.0-only */
/* include/linux/sm501.h
 *
 * Copyright (c) 2006 Simtec Electronics
 *	Ben Dooks <ben@simtec.co.uk>
 *	Vincent Sanders <vince@simtec.co.uk>
 */

/* Types supplied by the surrounding kernel code. */
pub enum device {}
pub enum fb_videomode {}

pub unsafe extern "C" fn sm501_unit_power(
    dev: *mut device,
    unit: u32,
    to: u32,
) -> i32;

pub unsafe extern "C" fn sm501_set_clock(
    dev: *mut device,
    clksrc: i32,
    freq: usize,
) -> usize;

/* sm501_misc_control
 *
 * Modify the SM501's MISC_CONTROL register
 */
pub unsafe extern "C" fn sm501_misc_control(
    dev: *mut device,
    set: usize,
    clear: usize,
) -> i32;

/* sm501_modify_reg
 *
 * Modify a register in the SM501 which may be shared with other
 * drivers.
 */
pub unsafe extern "C" fn sm501_modify_reg(
    dev: *mut device,
    reg: usize,
    set: usize,
    clear: usize,
) -> usize;

/* Platform data definitions */
pub const SM501FB_FLAG_USE_INIT_MODE: u32 = 1 << 0;
pub const SM501FB_FLAG_DISABLE_AT_EXIT: u32 = 1 << 1;
pub const SM501FB_FLAG_USE_HWCURSOR: u32 = 1 << 2;
pub const SM501FB_FLAG_USE_HWACCEL: u32 = 1 << 3;
pub const SM501FB_FLAG_PANEL_NO_FPEN: u32 = 1 << 4;
pub const SM501FB_FLAG_PANEL_NO_VBIASEN: u32 = 1 << 5;
pub const SM501FB_FLAG_PANEL_INV_FPEN: u32 = 1 << 6;
pub const SM501FB_FLAG_PANEL_INV_VBIASEN: u32 = 1 << 7;

#[repr(C)]
pub struct sm501_platdata_fbsub {
    pub def_mode: *mut fb_videomode,
    pub def_bpp: u32,
    pub max_mem: usize,
    pub flags: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum sm501_fb_routing {
    SM501_FB_OWN = 0,        /* CRT=>CRT, Panel=>Panel */
    SM501_FB_CRT_PANEL = 1,  /* Panel=>CRT, Panel=>Panel */
}

/* sm501_platdata_fb flag field bit definitions */
pub const SM501_FBPD_SWAP_FB_ENDIAN: u32 = 1 << 0; /* need to endian swap */

/* sm501_platdata_fb
 *
 * configuration data for the framebuffer driver
 */
#[repr(C)]
pub struct sm501_platdata_fb {
    pub fb_route: sm501_fb_routing,
    pub flags: u32,
    pub fb_crt: *mut sm501_platdata_fbsub,
    pub fb_pnl: *mut sm501_platdata_fbsub,
}

/* gpio i2c
 *
 * Note, we have to pass in the bus number, as the number used will be
 * passed to the i2c-gpio driver's platform_device.id, subsequently used to
 * register the i2c bus.
 */
#[repr(C)]
pub struct sm501_platdata_gpio_i2c {
    pub bus_num: u32,
    pub pin_sda: u32,
    pub pin_scl: u32,
    pub udelay: i32,
    pub timeout: i32,
}

/* sm501_initdata
 *
 * use for initialising values that may not have been setup
 * before the driver is loaded.
 */
#[repr(C)]
pub struct sm501_reg_init {
    pub set: usize,
    pub mask: usize,
}

pub const SM501_USE_USB_HOST: usize = 1 << 0;
pub const SM501_USE_USB_SLAVE: usize = 1 << 1;
pub const SM501_USE_SSP0: usize = 1 << 2;
pub const SM501_USE_SSP1: usize = 1 << 3;
pub const SM501_USE_UART0: usize = 1 << 4;
pub const SM501_USE_UART1: usize = 1 << 5;
pub const SM501_USE_FBACCEL: usize = 1 << 6;
pub const SM501_USE_AC97: usize = 1 << 7;
pub const SM501_USE_I2S: usize = 1 << 8;
pub const SM501_USE_GPIO: usize = 1 << 9;
pub const SM501_USE_ALL: usize = 0xffff_ffff;

#[repr(C)]
pub struct sm501_initdata {
    pub gpio_low: sm501_reg_init,
    pub gpio_high: sm501_reg_init,
    pub misc_timing: sm501_reg_init,
    pub misc_control: sm501_reg_init,
    pub devices: usize,
    pub mclk: usize,  /* non-zero to modify */
    pub m1xclk: usize, /* non-zero to modify */
}

/* sm501_init_gpio
 *
 * default gpio settings
 */
#[repr(C)]
pub struct sm501_init_gpio {
    pub gpio_data_low: sm501_reg_init,
    pub gpio_data_high: sm501_reg_init,
    pub gpio_ddr_low: sm501_reg_init,
    pub gpio_ddr_high: sm501_reg_init,
}

pub const SM501_FLAG_SUSPEND_OFF: i32 = 1 << 4;

/* sm501_platdata
 *
 * This is passed with the platform device to allow the board
 * to control the behaviour of the SM501 driver(s) which attach
 * to the device.
 */
#[repr(C)]
pub struct sm501_platdata {
    pub init: *mut sm501_initdata,
    pub init_gpiop: *mut sm501_init_gpio,
    pub fb: *mut sm501_platdata_fb,
    pub flags: i32,
    pub gpio_base: i32,
    pub get_power: Option<unsafe extern "C" fn(dev: *mut device) -> i32>,
    pub set_power: Option<unsafe extern "C" fn(dev: *mut device, on: u32) -> i32>,
    pub gpio_i2c: *mut sm501_platdata_gpio_i2c,
    pub gpio_i2c_nr: u32,
}

/* CONFIG_PPC32 selects big-endian I/O; preserve the source conditional. */
#[cfg(CONFIG_PPC32)]
#[macro_export]
macro_rules! smc501_readl { ($addr:expr) => { ioread32be($addr) }; }
#[cfg(CONFIG_PPC32)]
#[macro_export]
macro_rules! smc501_writel { ($val:expr, $addr:expr) => { iowrite32be($val, $addr) }; }
#[cfg(not(CONFIG_PPC32))]
#[macro_export]
macro_rules! smc501_readl { ($addr:expr) => { readl($addr) }; }
#[cfg(not(CONFIG_PPC32))]
#[macro_export]
macro_rules! smc501_writel { ($val:expr, $addr:expr) => { writel($val, $addr) }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
