/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * include/linux/mfd/wm831x/otp.h -- OTP interface for WM831x
 *
 * Copyright 2009 Wolfson Microelectronics PLC.
 *
 * Author: Mark Brown <broonie@opensource.wolfsonmicro.com>
 */

use core::ffi::c_int;

/* The concrete WM831x type is supplied by the surrounding translation unit. */
#[allow(non_camel_case_types)]
pub enum wm831x {}

extern "C" {
    pub fn wm831x_otp_init(wm831x: *mut wm831x) -> c_int;
    pub fn wm831x_otp_exit(wm831x: *mut wm831x);
}

/*
 * R30720 (0x7800) - Unique ID 1
 * R30721 (0x7801) - Unique ID 2
 * R30722 (0x7802) - Unique ID 3
 * R30723 (0x7803) - Unique ID 4
 * R30724 (0x7804) - Unique ID 5
 * R30725 (0x7805) - Unique ID 6
 * R30726 (0x7806) - Unique ID 7
 * R30727 (0x7807) - Unique ID 8
 *
 * The C header repeats these register-local macros with identical definitions.
 */
pub const WM831X_UNIQUE_ID_MASK: u16 = 0xFFFF;
pub const WM831X_UNIQUE_ID_SHIFT: u32 = 0;
pub const WM831X_UNIQUE_ID_WIDTH: u32 = 16;

/* R30728 (0x7800) - Factory OTP ID */
pub const WM831X_OTP_FACT_ID_MASK: u16 = 0xFFFE;
pub const WM831X_OTP_FACT_ID_SHIFT: u32 = 1;
pub const WM831X_OTP_FACT_ID_WIDTH: u32 = 15;
pub const WM831X_OTP_FACT_FINAL: u16 = 0x0001;
pub const WM831X_OTP_FACT_FINAL_MASK: u16 = 0x0001;
pub const WM831X_OTP_FACT_FINAL_SHIFT: u32 = 0;
pub const WM831X_OTP_FACT_FINAL_WIDTH: u32 = 1;

/* R30729 (0x7809) - Factory OTP 1 */
pub const WM831X_DC3_TRIM_MASK: u16 = 0xF000;
pub const WM831X_DC3_TRIM_SHIFT: u32 = 12;
pub const WM831X_DC3_TRIM_WIDTH: u32 = 4;
pub const WM831X_DC2_TRIM_MASK: u16 = 0x0FC0;
pub const WM831X_DC2_TRIM_SHIFT: u32 = 6;
pub const WM831X_DC2_TRIM_WIDTH: u32 = 6;
pub const WM831X_DC1_TRIM_MASK: u16 = 0x003F;
pub const WM831X_DC1_TRIM_SHIFT: u32 = 0;
pub const WM831X_DC1_TRIM_WIDTH: u32 = 6;

/* R30730 (0x780A) - Factory OTP 2 */
pub const WM831X_CHIP_ID_MASK: u16 = 0xFFFF;
pub const WM831X_CHIP_ID_SHIFT: u32 = 0;
pub const WM831X_CHIP_ID_WIDTH: u32 = 16;

/* R30731 (0x780B) - Factory OTP 3 */
pub const WM831X_OSC_TRIM_MASK: u16 = 0x0780;
pub const WM831X_OSC_TRIM_SHIFT: u32 = 7;
pub const WM831X_OSC_TRIM_WIDTH: u32 = 4;
pub const WM831X_BG_TRIM_MASK: u16 = 0x0078;
pub const WM831X_BG_TRIM_SHIFT: u32 = 3;
pub const WM831X_BG_TRIM_WIDTH: u32 = 4;
pub const WM831X_LPBG_TRIM_MASK: u16 = 0x0007;
pub const WM831X_LPBG_TRIM_SHIFT: u32 = 0;
pub const WM831X_LPBG_TRIM_WIDTH: u32 = 3;

/* R30732 (0x780C) - Factory OTP 4 */
pub const WM831X_CHILD_I2C_ADDR_MASK: u16 = 0x00FE;
pub const WM831X_CHILD_I2C_ADDR_SHIFT: u32 = 1;
pub const WM831X_CHILD_I2C_ADDR_WIDTH: u32 = 7;
pub const WM831X_CH_AW: u16 = 0x0001;
pub const WM831X_CH_AW_MASK: u16 = 0x0001;
pub const WM831X_CH_AW_SHIFT: u32 = 0;
pub const WM831X_CH_AW_WIDTH: u32 = 1;

/* R30733 (0x780D) - Factory OTP 5 */
pub const WM831X_CHARGE_TRIM_MASK: u16 = 0x003F;
pub const WM831X_CHARGE_TRIM_SHIFT: u32 = 0;
pub const WM831X_CHARGE_TRIM_WIDTH: u32 = 6;

/* R30736 (0x7810) - Customer OTP ID */
pub const WM831X_OTP_AUTO_PROG: u16 = 0x8000;
pub const WM831X_OTP_AUTO_PROG_MASK: u16 = 0x8000;
pub const WM831X_OTP_AUTO_PROG_SHIFT: u32 = 15;
pub const WM831X_OTP_AUTO_PROG_WIDTH: u32 = 1;
pub const WM831X_OTP_CUST_ID_MASK: u16 = 0x7FFE;
pub const WM831X_OTP_CUST_ID_SHIFT: u32 = 1;
pub const WM831X_OTP_CUST_ID_WIDTH: u32 = 14;
pub const WM831X_OTP_CUST_FINAL: u16 = 0x0001;
pub const WM831X_OTP_CUST_FINAL_MASK: u16 = 0x0001;
pub const WM831X_OTP_CUST_FINAL_SHIFT: u32 = 0;
pub const WM831X_OTP_CUST_FINAL_WIDTH: u32 = 1;

/* R30759 (0x7827) - DBE CHECK DATA */
pub const WM831X_DBE_VALID_DATA_MASK: u16 = 0xFFFF;
pub const WM831X_DBE_VALID_DATA_SHIFT: u32 = 0;
pub const WM831X_DBE_VALID_DATA_WIDTH: u32 = 16;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
