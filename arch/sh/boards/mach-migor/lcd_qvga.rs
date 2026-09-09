// SPDX-License-Identifier: GPL-2.0
/*
 * Support for SuperH MigoR Quarter VGA LCD Panel
 *
 * Copyright (C) 2008 Magnus Damm
 *
 * Based on lcd_powertip.c from Kenati Technologies Pvt Ltd.
 * Copyright (c) 2007 Ujjwal Pande <ujjwal@kenati.com>,
 */

use core::ffi::c_void;

// Supplied by the Linux kernel and board-specific dependencies.
extern "C" {
    fn gpio_set_value(gpio: i32, value: i32);
    fn mdelay(milliseconds: u32);
    fn pr_info(format: *const u8, ...);
}

// Supplied by <video/sh_mobile_lcdc.h>.
#[repr(C)]
pub struct sh_mobile_lcdc_sys_bus_ops {
    pub write_index: unsafe extern "C" fn(*mut c_void, usize),
    pub write_data: unsafe extern "C" fn(*mut c_void, usize),
    pub read_data: unsafe extern "C" fn(*mut c_void) -> usize,
}

// Supplied by <mach/migor.h>.
const GPIO_PTH2: i32 = 0;

unsafe fn reset_lcd_module() {
    gpio_set_value(GPIO_PTH2, 0);
    mdelay(2);
    gpio_set_value(GPIO_PTH2, 1);
    mdelay(1);
}

/* DB0-DB7 are connected to D1-D8, and DB8-DB15 to D10-D17 */

unsafe fn adjust_reg18(data: u16) -> usize {
    let tmp1 = (((data as usize) << 1) | 0x00000001) & 0x000001FF;
    let tmp2 = (((data as usize) << 2) | 0x00000200) & 0x0003FE00;
    tmp1 | tmp2
}

unsafe fn write_reg(
    sys_ops_handle: *mut c_void,
    sys_ops: *mut sh_mobile_lcdc_sys_bus_ops,
    reg: u16,
    data: u16,
) {
    ((*sys_ops).write_index)(sys_ops_handle, adjust_reg18((reg << 8) | data));
}

unsafe fn write_reg16(
    sys_ops_handle: *mut c_void,
    sys_ops: *mut sh_mobile_lcdc_sys_bus_ops,
    reg: u16,
    data: u16,
) {
    ((*sys_ops).write_index)(sys_ops_handle, adjust_reg18(reg));
    ((*sys_ops).write_data)(sys_ops_handle, adjust_reg18(data));
}

unsafe fn read_reg16(
    sys_ops_handle: *mut c_void,
    sys_ops: *mut sh_mobile_lcdc_sys_bus_ops,
    reg: u16,
) -> usize {
    ((*sys_ops).write_index)(sys_ops_handle, adjust_reg18(reg));
    let data = ((*sys_ops).read_data)(sys_ops_handle);
    ((data >> 1) & 0xff) | ((data >> 2) & 0xff00)
}

unsafe fn migor_lcd_qvga_seq(
    sys_ops_handle: *mut c_void,
    sys_ops: *mut sh_mobile_lcdc_sys_bus_ops,
    data: &[u16],
    no_data: i32,
) {
    let mut i = 0;
    while i < no_data {
        write_reg16(sys_ops_handle, sys_ops, data[i as usize], data[(i + 1) as usize]);
        i += 2;
    }
}

static SYNC_DATA: [u16; 8] = [
    0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000, 0x0000,
];

static MAGIC0_DATA: [u16; 24] = [
    0x0060, 0x2700, 0x0008, 0x0808, 0x0090, 0x001A, 0x0007, 0x0001,
    0x0017, 0x0001, 0x0019, 0x0000, 0x0010, 0x17B0, 0x0011, 0x0116,
    0x0012, 0x0198, 0x0013, 0x1400, 0x0029, 0x000C, 0x0012, 0x01B8,
];

static MAGIC1_DATA: [u16; 50] = [
    0x0030, 0x0307, 0x0031, 0x0303, 0x0032, 0x0603, 0x0033, 0x0202,
    0x0034, 0x0202, 0x0035, 0x0202, 0x0036, 0x1F1F, 0x0037, 0x0303,
    0x0038, 0x0303, 0x0039, 0x0603, 0x003A, 0x0202, 0x003B, 0x0102,
    0x003C, 0x0204, 0x003D, 0x0000, 0x0001, 0x0100, 0x0002, 0x0300,
    0x0003, 0x5028, 0x0020, 0x00ef, 0x0021, 0x0000, 0x0004, 0x0000,
    0x0009, 0x0000, 0x000A, 0x0008, 0x000C, 0x0000, 0x000D, 0x0000,
    0x0015, 0x8000,
];

static MAGIC2_DATA: [u16; 8] = [0x0061, 0x0001, 0x0092, 0x0100, 0x0093, 0x0001, 0x0007, 0x0021];
static MAGIC3_DATA: [u16; 6] = [0x0010, 0x16B0, 0x0011, 0x0111, 0x0007, 0x0061];

pub unsafe extern "C" fn migor_lcd_qvga_setup(
    sohandle: *mut c_void,
    so: *mut sh_mobile_lcdc_sys_bus_ops,
) -> i32 {
    let xres: usize = 320;
    let yres: usize = 240;
    reset_lcd_module();
    migor_lcd_qvga_seq(sohandle, so, &SYNC_DATA, SYNC_DATA.len() as i32);
    if read_reg16(sohandle, so, 0) != 0x1505 { return -19; }
    pr_info(b"Migo-R QVGA LCD Module detected.\n\0".as_ptr());
    migor_lcd_qvga_seq(sohandle, so, &SYNC_DATA, SYNC_DATA.len() as i32);
    write_reg16(sohandle, so, 0x00A4, 0x0001); mdelay(10);
    migor_lcd_qvga_seq(sohandle, so, &MAGIC0_DATA, MAGIC0_DATA.len() as i32); mdelay(100);
    migor_lcd_qvga_seq(sohandle, so, &MAGIC1_DATA, MAGIC1_DATA.len() as i32);
    write_reg16(sohandle, so, 0x0050, 0x00ef - (yres as u16 - 1));
    write_reg16(sohandle, so, 0x0051, 0x00ef); write_reg16(sohandle, so, 0x0052, 0x0000);
    write_reg16(sohandle, so, 0x0053, (xres - 1) as u16);
    migor_lcd_qvga_seq(sohandle, so, &MAGIC2_DATA, MAGIC2_DATA.len() as i32); mdelay(10);
    migor_lcd_qvga_seq(sohandle, so, &MAGIC3_DATA, MAGIC3_DATA.len() as i32); mdelay(40);
    write_reg16(sohandle, so, 0x0020, 0x0000); write_reg16(sohandle, so, 0x0021, 0x0000);
    for _k in 0..(xres * 256) { write_reg16(sohandle, so, 0x0022, 0x0000); }
    write_reg16(sohandle, so, 0x0020, 0x0000); write_reg16(sohandle, so, 0x0021, 0x0000);
    write_reg16(sohandle, so, 0x0007, 0x0173); mdelay(40);
    write_reg(sohandle, so, 0x00, 0x22); mdelay(100);
    0
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
