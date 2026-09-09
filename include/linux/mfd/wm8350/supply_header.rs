/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * supply.h  --  Power Supply Driver for Wolfson WM8350 PMIC
 *
 * Copyright 2007 Wolfson Microelectronics PLC
 */

// C dependencies: linux/mutex.h and linux/power_supply.h.

/* Charger registers */
pub const WM8350_BATTERY_CHARGER_CONTROL_1: u32 = 0xA8;
pub const WM8350_BATTERY_CHARGER_CONTROL_2: u32 = 0xA9;
pub const WM8350_BATTERY_CHARGER_CONTROL_3: u32 = 0xAA;

/* R168 (0xA8) - Battery Charger Control 1 */
pub const WM8350_CHG_ENA_R168: u32 = 0x8000;
pub const WM8350_CHG_THR: u32 = 0x2000;
pub const WM8350_CHG_EOC_SEL_MASK: u32 = 0x1C00;
pub const WM8350_CHG_TRICKLE_TEMP_CHOKE: u32 = 0x0200;
pub const WM8350_CHG_TRICKLE_USB_CHOKE: u32 = 0x0100;
pub const WM8350_CHG_RECOVER_T: u32 = 0x0080;
pub const WM8350_CHG_END_ACT: u32 = 0x0040;
pub const WM8350_CHG_FAST: u32 = 0x0020;
pub const WM8350_CHG_FAST_USB_THROTTLE: u32 = 0x0010;
pub const WM8350_CHG_NTC_MON: u32 = 0x0008;
pub const WM8350_CHG_BATT_HOT_MON: u32 = 0x0004;
pub const WM8350_CHG_BATT_COLD_MON: u32 = 0x0002;
pub const WM8350_CHG_CHIP_TEMP_MON: u32 = 0x0001;

/* R169 (0xA9) - Battery Charger Control 2 */
pub const WM8350_CHG_ACTIVE: u32 = 0x8000;
pub const WM8350_CHG_PAUSE: u32 = 0x4000;
pub const WM8350_CHG_STS_MASK: u32 = 0x3000;
pub const WM8350_CHG_TIME_MASK: u32 = 0x0F00;
pub const WM8350_CHG_MASK_WALL_FB: u32 = 0x0080;
pub const WM8350_CHG_TRICKLE_SEL: u32 = 0x0040;
pub const WM8350_CHG_VSEL_MASK: u32 = 0x0030;
pub const WM8350_CHG_ISEL_MASK: u32 = 0x000F;
pub const WM8350_CHG_STS_OFF: u32 = 0x0000;
pub const WM8350_CHG_STS_TRICKLE: u32 = 0x1000;
pub const WM8350_CHG_STS_FAST: u32 = 0x2000;

/* R170 (0xAA) - Battery Charger Control 3 */
pub const WM8350_CHG_THROTTLE_T_MASK: u32 = 0x0060;
pub const WM8350_CHG_SMART: u32 = 0x0010;
pub const WM8350_CHG_TIMER_ADJT_MASK: u32 = 0x000F;

/* Charger Interrupts */
pub const WM8350_IRQ_CHG_BAT_HOT: u32 = 0;
pub const WM8350_IRQ_CHG_BAT_COLD: u32 = 1;
pub const WM8350_IRQ_CHG_BAT_FAIL: u32 = 2;
pub const WM8350_IRQ_CHG_TO: u32 = 3;
pub const WM8350_IRQ_CHG_END: u32 = 4;
pub const WM8350_IRQ_CHG_START: u32 = 5;
pub const WM8350_IRQ_CHG_FAST_RDY: u32 = 6;
pub const WM8350_IRQ_CHG_VBATT_LT_3P9: u32 = 10;
pub const WM8350_IRQ_CHG_VBATT_LT_3P1: u32 = 11;
pub const WM8350_IRQ_CHG_VBATT_LT_2P85: u32 = 12;

/* Charger Policy */
pub const WM8350_CHG_TRICKLE_50mA: u32 = 0 << 6;
pub const WM8350_CHG_TRICKLE_100mA: u32 = 1 << 6;
pub const WM8350_CHG_4_05V: u32 = 0 << 4;
pub const WM8350_CHG_4_10V: u32 = 1 << 4;
pub const WM8350_CHG_4_15V: u32 = 2 << 4;
pub const WM8350_CHG_4_20V: u32 = 3 << 4;
pub const fn WM8350_CHG_FAST_LIMIT_mA(x: u32) -> u32 { (x / 50) & 0xf }
pub const fn WM8350_CHG_EOC_mA(x: u32) -> u32 { ((x - 10) & 0x7) << 10 }
pub const WM8350_CHG_TRICKLE_3_1V: u32 = 0 << 13;
pub const WM8350_CHG_TRICKLE_3_9V: u32 = 1 << 13;

/* Supply Registers. */
pub const WM8350_USB_VOLTAGE_READBACK: u32 = 0x9C;
pub const WM8350_LINE_VOLTAGE_READBACK: u32 = 0x9D;
pub const WM8350_BATT_VOLTAGE_READBACK: u32 = 0x9E;

/* Supply Interrupts. */
pub const WM8350_IRQ_USB_LIMIT: u32 = 15;
pub const WM8350_IRQ_EXT_USB_FB: u32 = 36;
pub const WM8350_IRQ_EXT_WALL_FB: u32 = 37;
pub const WM8350_IRQ_EXT_BAT_FB: u32 = 38;

/* Policy to control charger state machine. */
#[repr(C)]
pub struct wm8350_charger_policy {
    /* charger state machine policy  - set in machine driver */
    pub eoc_mA: i32,              /* end of charge current (mA)  */
    pub charge_mV: i32,           /* charge voltage */
    pub fast_limit_mA: i32,       /* fast charge current limit */
    pub fast_limit_USB_mA: i32,   /* USB fast charge current limit */
    pub charge_timeout: i32,      /* charge timeout (mins) */
    pub trickle_start_mV: i32,    /* trickle charge starts at mV */
    pub trickle_charge_mA: i32,   /* trickle charge current */
    pub trickle_charge_USB_mA: i32, /* USB trickle charge current */
}

// Supplied by the platform and power-supply dependencies.
#[repr(C)]
pub struct platform_device;
#[repr(C)]
pub struct power_supply;

#[repr(C)]
pub struct wm8350_power {
    pub pdev: *mut platform_device,
    pub battery: *mut power_supply,
    pub usb: *mut power_supply,
    pub ac: *mut power_supply,
    pub policy: *mut wm8350_charger_policy,
    pub rev_g_coeff: i32,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
