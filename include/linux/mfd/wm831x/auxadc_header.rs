/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/mfd/wm831x/auxadc.h -- Auxiliary ADC interface for WM831x
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

#[repr(C)]
pub struct wm831x {
    _private: [u8; 0],
}

/*
 * R16429 (0x402D) - AuxADC Data
 */
pub const WM831X_AUX_DATA_SRC_MASK: u16 = 0xF000;
pub const WM831X_AUX_DATA_SRC_SHIFT: u32 = 12;
pub const WM831X_AUX_DATA_SRC_WIDTH: u32 = 4;
pub const WM831X_AUX_DATA_MASK: u16 = 0x0FFF;
pub const WM831X_AUX_DATA_SHIFT: u32 = 0;
pub const WM831X_AUX_DATA_WIDTH: u32 = 12;

/*
 * R16430 (0x402E) - AuxADC Control
 */
pub const WM831X_AUX_ENA: u16 = 0x8000;
pub const WM831X_AUX_ENA_MASK: u16 = 0x8000;
pub const WM831X_AUX_ENA_SHIFT: u32 = 15;
pub const WM831X_AUX_ENA_WIDTH: u32 = 1;
pub const WM831X_AUX_CVT_ENA: u16 = 0x4000;
pub const WM831X_AUX_CVT_ENA_MASK: u16 = 0x4000;
pub const WM831X_AUX_CVT_ENA_SHIFT: u32 = 14;
pub const WM831X_AUX_CVT_ENA_WIDTH: u32 = 1;
pub const WM831X_AUX_SLPENA: u16 = 0x1000;
pub const WM831X_AUX_SLPENA_MASK: u16 = 0x1000;
pub const WM831X_AUX_SLPENA_SHIFT: u32 = 12;
pub const WM831X_AUX_SLPENA_WIDTH: u32 = 1;
pub const WM831X_AUX_FRC_ENA: u16 = 0x0800;
pub const WM831X_AUX_FRC_ENA_MASK: u16 = 0x0800;
pub const WM831X_AUX_FRC_ENA_SHIFT: u32 = 11;
pub const WM831X_AUX_FRC_ENA_WIDTH: u32 = 1;
pub const WM831X_AUX_RATE_MASK: u16 = 0x003F;
pub const WM831X_AUX_RATE_SHIFT: u32 = 0;
pub const WM831X_AUX_RATE_WIDTH: u32 = 6;

/*
 * R16431 (0x402F) - AuxADC Source
 */
pub const WM831X_AUX_CAL_SEL: u16 = 0x8000;
pub const WM831X_AUX_CAL_SEL_MASK: u16 = 0x8000;
pub const WM831X_AUX_CAL_SEL_SHIFT: u32 = 15;
pub const WM831X_AUX_CAL_SEL_WIDTH: u32 = 1;
pub const WM831X_AUX_BKUP_BATT_SEL: u16 = 0x0400;
pub const WM831X_AUX_BKUP_BATT_SEL_MASK: u16 = 0x0400;
pub const WM831X_AUX_BKUP_BATT_SEL_SHIFT: u32 = 10;
pub const WM831X_AUX_BKUP_BATT_SEL_WIDTH: u32 = 1;
pub const WM831X_AUX_WALL_SEL: u16 = 0x0200;
pub const WM831X_AUX_WALL_SEL_MASK: u16 = 0x0200;
pub const WM831X_AUX_WALL_SEL_SHIFT: u32 = 9;
pub const WM831X_AUX_WALL_SEL_WIDTH: u32 = 1;
pub const WM831X_AUX_BATT_SEL: u16 = 0x0100;
pub const WM831X_AUX_BATT_SEL_MASK: u16 = 0x0100;
pub const WM831X_AUX_BATT_SEL_SHIFT: u32 = 8;
pub const WM831X_AUX_BATT_SEL_WIDTH: u32 = 1;
pub const WM831X_AUX_USB_SEL: u16 = 0x0080;
pub const WM831X_AUX_USB_SEL_MASK: u16 = 0x0080;
pub const WM831X_AUX_USB_SEL_SHIFT: u32 = 7;
pub const WM831X_AUX_USB_SEL_WIDTH: u32 = 1;
pub const WM831X_AUX_SYSVDD_SEL: u16 = 0x0040;
pub const WM831X_AUX_SYSVDD_SEL_MASK: u16 = 0x0040;
pub const WM831X_AUX_SYSVDD_SEL_SHIFT: u32 = 6;
pub const WM831X_AUX_SYSVDD_SEL_WIDTH: u32 = 1;
pub const WM831X_AUX_BATT_TEMP_SEL: u16 = 0x0020;
pub const WM831X_AUX_BATT_TEMP_SEL_MASK: u16 = 0x0020;
pub const WM831X_AUX_BATT_TEMP_SEL_SHIFT: u32 = 5;
pub const WM831X_AUX_BATT_TEMP_SEL_WIDTH: u32 = 1;
pub const WM831X_AUX_CHIP_TEMP_SEL: u16 = 0x0010;
pub const WM831X_AUX_CHIP_TEMP_SEL_MASK: u16 = 0x0010;
pub const WM831X_AUX_CHIP_TEMP_SEL_SHIFT: u32 = 4;
pub const WM831X_AUX_CHIP_TEMP_SEL_WIDTH: u32 = 1;
pub const WM831X_AUX_AUX4_SEL: u16 = 0x0008;
pub const WM831X_AUX_AUX4_SEL_MASK: u16 = 0x0008;
pub const WM831X_AUX_AUX4_SEL_SHIFT: u32 = 3;
pub const WM831X_AUX_AUX4_SEL_WIDTH: u32 = 1;
pub const WM831X_AUX_AUX3_SEL: u16 = 0x0004;
pub const WM831X_AUX_AUX3_SEL_MASK: u16 = 0x0004;
pub const WM831X_AUX_AUX3_SEL_SHIFT: u32 = 2;
pub const WM831X_AUX_AUX3_SEL_WIDTH: u32 = 1;
pub const WM831X_AUX_AUX2_SEL: u16 = 0x0002;
pub const WM831X_AUX_AUX2_SEL_MASK: u16 = 0x0002;
pub const WM831X_AUX_AUX2_SEL_SHIFT: u32 = 1;
pub const WM831X_AUX_AUX2_SEL_WIDTH: u32 = 1;
pub const WM831X_AUX_AUX1_SEL: u16 = 0x0001;
pub const WM831X_AUX_AUX1_SEL_MASK: u16 = 0x0001;
pub const WM831X_AUX_AUX1_SEL_SHIFT: u32 = 0;
pub const WM831X_AUX_AUX1_SEL_WIDTH: u32 = 1;

/*
 * R16432 (0x4030) - Comparator Control
 */
pub const WM831X_DCOMP4_STS: u16 = 0x0800;
pub const WM831X_DCOMP4_STS_MASK: u16 = 0x0800;
pub const WM831X_DCOMP4_STS_SHIFT: u32 = 11;
pub const WM831X_DCOMP4_STS_WIDTH: u32 = 1;
pub const WM831X_DCOMP3_STS: u16 = 0x0400;
pub const WM831X_DCOMP3_STS_MASK: u16 = 0x0400;
pub const WM831X_DCOMP3_STS_SHIFT: u32 = 10;
pub const WM831X_DCOMP3_STS_WIDTH: u32 = 1;
pub const WM831X_DCOMP2_STS: u16 = 0x0200;
pub const WM831X_DCOMP2_STS_MASK: u16 = 0x0200;
pub const WM831X_DCOMP2_STS_SHIFT: u32 = 9;
pub const WM831X_DCOMP2_STS_WIDTH: u32 = 1;
pub const WM831X_DCOMP1_STS: u16 = 0x0100;
pub const WM831X_DCOMP1_STS_MASK: u16 = 0x0100;
pub const WM831X_DCOMP1_STS_SHIFT: u32 = 8;
pub const WM831X_DCOMP1_STS_WIDTH: u32 = 1;
pub const WM831X_DCMP4_ENA: u16 = 0x0008;
pub const WM831X_DCMP4_ENA_MASK: u16 = 0x0008;
pub const WM831X_DCMP4_ENA_SHIFT: u32 = 3;
pub const WM831X_DCMP4_ENA_WIDTH: u32 = 1;
pub const WM831X_DCMP3_ENA: u16 = 0x0004;
pub const WM831X_DCMP3_ENA_MASK: u16 = 0x0004;
pub const WM831X_DCMP3_ENA_SHIFT: u32 = 2;
pub const WM831X_DCMP3_ENA_WIDTH: u32 = 1;
pub const WM831X_DCMP2_ENA: u16 = 0x0002;
pub const WM831X_DCMP2_ENA_MASK: u16 = 0x0002;
pub const WM831X_DCMP2_ENA_SHIFT: u32 = 1;
pub const WM831X_DCMP2_ENA_WIDTH: u32 = 1;
pub const WM831X_DCMP1_ENA: u16 = 0x0001;
pub const WM831X_DCMP1_ENA_MASK: u16 = 0x0001;
pub const WM831X_DCMP1_ENA_SHIFT: u32 = 0;
pub const WM831X_DCMP1_ENA_WIDTH: u32 = 1;

/*
 * R16433 (0x4031) - Comparator 1
 */
pub const WM831X_DCMP1_SRC_MASK: u16 = 0xE000;
pub const WM831X_DCMP1_SRC_SHIFT: u32 = 13;
pub const WM831X_DCMP1_SRC_WIDTH: u32 = 3;
pub const WM831X_DCMP1_GT: u16 = 0x1000;
pub const WM831X_DCMP1_GT_MASK: u16 = 0x1000;
pub const WM831X_DCMP1_GT_SHIFT: u32 = 12;
pub const WM831X_DCMP1_GT_WIDTH: u32 = 1;
pub const WM831X_DCMP1_THR_MASK: u16 = 0x0FFF;
pub const WM831X_DCMP1_THR_SHIFT: u32 = 0;
pub const WM831X_DCMP1_THR_WIDTH: u32 = 12;

/*
 * R16434 (0x4032) - Comparator 2
 */
pub const WM831X_DCMP2_SRC_MASK: u16 = 0xE000;
pub const WM831X_DCMP2_SRC_SHIFT: u32 = 13;
pub const WM831X_DCMP2_SRC_WIDTH: u32 = 3;
pub const WM831X_DCMP2_GT: u16 = 0x1000;
pub const WM831X_DCMP2_GT_MASK: u16 = 0x1000;
pub const WM831X_DCMP2_GT_SHIFT: u32 = 12;
pub const WM831X_DCMP2_GT_WIDTH: u32 = 1;
pub const WM831X_DCMP2_THR_MASK: u16 = 0x0FFF;
pub const WM831X_DCMP2_THR_SHIFT: u32 = 0;
pub const WM831X_DCMP2_THR_WIDTH: u32 = 12;

/*
 * R16435 (0x4033) - Comparator 3
 */
pub const WM831X_DCMP3_SRC_MASK: u16 = 0xE000;
pub const WM831X_DCMP3_SRC_SHIFT: u32 = 13;
pub const WM831X_DCMP3_SRC_WIDTH: u32 = 3;
pub const WM831X_DCMP3_GT: u16 = 0x1000;
pub const WM831X_DCMP3_GT_MASK: u16 = 0x1000;
pub const WM831X_DCMP3_GT_SHIFT: u32 = 12;
pub const WM831X_DCMP3_GT_WIDTH: u32 = 1;
pub const WM831X_DCMP3_THR_MASK: u16 = 0x0FFF;
pub const WM831X_DCMP3_THR_SHIFT: u32 = 0;
pub const WM831X_DCMP3_THR_WIDTH: u32 = 12;

/*
 * R16436 (0x4034) - Comparator 4
 */
pub const WM831X_DCMP4_SRC_MASK: u16 = 0xE000;
pub const WM831X_DCMP4_SRC_SHIFT: u32 = 13;
pub const WM831X_DCMP4_SRC_WIDTH: u32 = 3;
pub const WM831X_DCMP4_GT: u16 = 0x1000;
pub const WM831X_DCMP4_GT_MASK: u16 = 0x1000;
pub const WM831X_DCMP4_GT_SHIFT: u32 = 12;
pub const WM831X_DCMP4_GT_WIDTH: u32 = 1;
pub const WM831X_DCMP4_THR_MASK: u16 = 0x0FFF;
pub const WM831X_DCMP4_THR_SHIFT: u32 = 0;
pub const WM831X_DCMP4_THR_WIDTH: u32 = 12;

pub const WM831X_AUX_CAL_FACTOR: u32 = 0xfff;
pub const WM831X_AUX_CAL_NOMINAL: u32 = 0x222;

#[repr(C)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum wm831x_auxadc {
    WM831X_AUX_CAL = 15,
    WM831X_AUX_BKUP_BATT = 10,
    WM831X_AUX_WALL = 9,
    WM831X_AUX_BATT = 8,
    WM831X_AUX_USB = 7,
    WM831X_AUX_SYSVDD = 6,
    WM831X_AUX_BATT_TEMP = 5,
    WM831X_AUX_CHIP_TEMP = 4,
    WM831X_AUX_AUX4 = 3,
    WM831X_AUX_AUX3 = 2,
    WM831X_AUX_AUX2 = 1,
    WM831X_AUX_AUX1 = 0,
}

unsafe extern "C" {
    pub fn wm831x_auxadc_read(wm831x: *mut wm831x, input: wm831x_auxadc) -> i32;
    pub fn wm831x_auxadc_read_uv(wm831x: *mut wm831x, input: wm831x_auxadc) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
