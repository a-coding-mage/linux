// SPDX-License-Identifier: GPL-2.0
/*
 * KFR2R09 LCD panel support
 *
 * Copyright (C) 2009 Magnus Damm
 *
 * Register settings based on the out-of-tree t33fb.c driver
 * Copyright (C) 2008 Lineo Solutions, Inc.
 */

use core::ffi::c_void;

// Supplied by the Linux SH-Mobile LCDC and board support dependencies.
extern "C" {
    fn mdelay(msecs: u32);
    fn udelay(usecs: u32);
    fn gpio_set_value(gpio: i32, value: i32);
    fn pr_info(fmt: *const u8, ...);
}

// External C layout supplied by the SH-Mobile LCDC dependency.
#[repr(C)]
pub struct sh_mobile_lcdc_sys_bus_ops {
    pub read_data: unsafe extern "C" fn(*mut c_void) -> u64,
    pub write_data: unsafe extern "C" fn(*mut c_void, u64),
    pub write_index: unsafe extern "C" fn(*mut c_void, u64),
}

const GPIO_PTF4: i32 = 0;
const GPIO_PTE4: i32 = 0;
const ENODEV: i32 = 19;

static DATA_FRAME_IF: [u8; 5] = [
    0x02, /* WEMODE: 1=cont, 0=one-shot */
    0x00, 0x00,
    0x00, /* EPF, DFM */
    0x02, /* RIM[1] : 1 (18bpp) */
];

static DATA_PANEL: [u8; 9] = [
    0x0b,
    0x63, /* 400 lines */
    0x04, 0x00, 0x00, 0x04, 0x11, 0x00, 0x00,
];

static DATA_TIMING: [u8; 5] = [0x00, 0x00, 0x13, 0x08, 0x08];
static DATA_TIMING_SRC: [u8; 4] = [0x11, 0x01, 0x00, 0x01];
static DATA_GAMMA: [u8; 20] = [
    0x01, 0x02, 0x08, 0x23, 0x03, 0x0c, 0x00, 0x06, 0x00, 0x00,
    0x01, 0x00, 0x0c, 0x23, 0x03, 0x08, 0x02, 0x06, 0x00, 0x00,
];
static DATA_POWER: [u8; 6] = [0x07, 0xc5, 0xdc, 0x02, 0x33, 0x0a];

unsafe fn read_reg(sohandle: *mut c_void, so: *mut sh_mobile_lcdc_sys_bus_ops) -> u64 {
    ((*so).read_data)(sohandle)
}

unsafe fn write_reg(
    sohandle: *mut c_void,
    so: *mut sh_mobile_lcdc_sys_bus_ops,
    i: i32,
    v: u64,
) {
    if i != 0 {
        ((*so).write_data)(sohandle, v); /* PTH4/LCDRS High [param, 17:0] */
    } else {
        ((*so).write_index)(sohandle, v); /* PTH4/LCDRS Low [cmd, 7:0] */
    }
}

unsafe fn write_data(
    sohandle: *mut c_void,
    so: *mut sh_mobile_lcdc_sys_bus_ops,
    data: *const u8,
    no_data: i32,
) {
    let mut i = 0;
    while i < no_data {
        write_reg(sohandle, so, 1, *data.add(i as usize) as u64);
        i += 1;
    }
}

unsafe fn read_device_code(sohandle: *mut c_void, so: *mut sh_mobile_lcdc_sys_bus_ops) -> u64 {
    /* access protect OFF */
    write_reg(sohandle, so, 0, 0xb0);
    write_reg(sohandle, so, 1, 0x00);
    /* deep standby OFF */
    write_reg(sohandle, so, 0, 0xb1);
    write_reg(sohandle, so, 1, 0x00);
    /* device code command */
    write_reg(sohandle, so, 0, 0xbf);
    mdelay(50);
    /* dummy read */
    read_reg(sohandle, so);
    /* read device code */
    let mut device_code = (read_reg(sohandle, so) & 0xff) << 24;
    device_code |= (read_reg(sohandle, so) & 0xff) << 16;
    device_code |= (read_reg(sohandle, so) & 0xff) << 8;
    device_code |= read_reg(sohandle, so) & 0xff;
    device_code
}

unsafe fn write_memory_start(sohandle: *mut c_void, so: *mut sh_mobile_lcdc_sys_bus_ops) {
    write_reg(sohandle, so, 0, 0x2c);
}

unsafe fn clear_memory(sohandle: *mut c_void, so: *mut sh_mobile_lcdc_sys_bus_ops) {
    /* write start */
    write_memory_start(sohandle, so);
    /* paint it black */
    for _ in 0..(240 * 400) {
        write_reg(sohandle, so, 1, 0x00);
    }
}

unsafe fn display_on(sohandle: *mut c_void, so: *mut sh_mobile_lcdc_sys_bus_ops) {
    /* access protect off */
    write_reg(sohandle, so, 0, 0xb0); write_reg(sohandle, so, 1, 0x00);
    /* exit deep standby mode */
    write_reg(sohandle, so, 0, 0xb1); write_reg(sohandle, so, 1, 0x00);
    write_reg(sohandle, so, 0, 0xb3); write_data(sohandle, so, DATA_FRAME_IF.as_ptr(), DATA_FRAME_IF.len() as i32);
    write_reg(sohandle, so, 0, 0xb4); write_reg(sohandle, so, 1, 0x00);
    write_reg(sohandle, so, 0, 0xc0); write_data(sohandle, so, DATA_PANEL.as_ptr(), DATA_PANEL.len() as i32);
    for command in 0xc1..=0xc3 { write_reg(sohandle, so, 0, command); write_data(sohandle, so, DATA_TIMING.as_ptr(), DATA_TIMING.len() as i32); }
    write_reg(sohandle, so, 0, 0xc4); write_data(sohandle, so, DATA_TIMING_SRC.as_ptr(), DATA_TIMING_SRC.len() as i32);
    for command in 0xc8..=0xca { write_reg(sohandle, so, 0, command); write_data(sohandle, so, DATA_GAMMA.as_ptr(), DATA_GAMMA.len() as i32); }
    write_reg(sohandle, so, 0, 0xd0); write_data(sohandle, so, DATA_POWER.as_ptr(), DATA_POWER.len() as i32);
    write_reg(sohandle, so, 0, 0xd1); write_reg(sohandle, so, 1, 0x00); write_reg(sohandle, so, 1, 0x0f); write_reg(sohandle, so, 1, 0x02);
    for command in 0xd2..=0xd4 { write_reg(sohandle, so, 0, command); write_reg(sohandle, so, 1, 0x63); write_reg(sohandle, so, 1, 0x24); }
    write_reg(sohandle, so, 0, 0xd8); write_reg(sohandle, so, 1, 0x77); write_reg(sohandle, so, 1, 0x77);
    write_reg(sohandle, so, 0, 0x35); write_reg(sohandle, so, 1, 0x00);
    write_reg(sohandle, so, 0, 0x44); write_reg(sohandle, so, 1, 0x00); write_reg(sohandle, so, 1, 0x00);
    write_reg(sohandle, so, 0, 0x2a); for value in [0x00, 0x00, 0x00, 0xef] { write_reg(sohandle, so, 1, value); }
    write_reg(sohandle, so, 0, 0x2b); for value in [0x00, 0x00, 0x01, 0x8f] { write_reg(sohandle, so, 1, value); }
    write_reg(sohandle, so, 0, 0x11); mdelay(120);
    clear_memory(sohandle, so);
    write_reg(sohandle, so, 0, 0x29); mdelay(1);
    write_memory_start(sohandle, so);
}

pub unsafe fn kfr2r09_lcd_setup(sohandle: *mut c_void, so: *mut sh_mobile_lcdc_sys_bus_ops) -> i32 {
    gpio_set_value(GPIO_PTF4, 0); gpio_set_value(GPIO_PTE4, 0); gpio_set_value(GPIO_PTF4, 1);
    udelay(1100); gpio_set_value(GPIO_PTE4, 1); udelay(10); gpio_set_value(GPIO_PTF4, 0); mdelay(20);
    if read_device_code(sohandle, so) != 0x01221517 { return -ENODEV; }
    pr_info(b"KFR2R09 WQVGA LCD Module detected.\n\0".as_ptr());
    display_on(sohandle, so);
    0
}

pub unsafe fn kfr2r09_lcd_start(sohandle: *mut c_void, so: *mut sh_mobile_lcdc_sys_bus_ops) {
    write_memory_start(sohandle, so);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
