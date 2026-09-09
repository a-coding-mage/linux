/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Analog Devices ADP5585 I/O expander, PWM controller and keypad controller
 *
 * Copyright 2022 NXP
 * Copyright 2024 Ideas on Board Oy
 */

// Originally included <linux/bits.h> and <linux/notifier.h>.

#[inline(always)]
pub const fn bit(n: u32) -> u32 { 1u32 << n }

#[inline(always)]
pub const fn genmask(high: u32, low: u32) -> u32 {
    if high == 31 { u32::MAX << low } else { ((1u32 << (high + 1)) - 1) & !((1u32 << low) - 1) }
}

macro_rules! BIT { ($n:expr) => { bit($n) }; }
macro_rules! GENMASK { ($h:expr, $l:expr) => { genmask($h, $l) }; }

pub const ADP5585_ID: u32 = 0x00;
pub const ADP5585_MAN_ID_VALUE: u32 = 0x20;
pub const ADP5585_MAN_ID_MASK: u32 = GENMASK!(7, 4);
pub const ADP5585_REV_ID_MASK: u32 = GENMASK!(3, 0);
pub const ADP5585_INT_STATUS: u32 = 0x01;
pub const ADP5585_OVRFLOW_INT: u32 = BIT!(2);
pub const ADP5585_EVENT_INT: u32 = BIT!(0);
pub const ADP5585_STATUS: u32 = 0x02;
pub const ADP5585_EC_MASK: u32 = GENMASK!(4, 0);
pub const ADP5585_FIFO_1: u32 = 0x03;
pub const ADP5585_KEV_EV_PRESS_MASK: u32 = BIT!(7);
pub const ADP5585_KEY_EVENT_MASK: u32 = GENMASK!(6, 0);
pub const ADP5585_FIFO_2: u32 = 0x04;
pub const ADP5585_FIFO_3: u32 = 0x05;
pub const ADP5585_FIFO_4: u32 = 0x06;
pub const ADP5585_FIFO_5: u32 = 0x07;
pub const ADP5585_FIFO_6: u32 = 0x08;
pub const ADP5585_FIFO_7: u32 = 0x09;
pub const ADP5585_FIFO_8: u32 = 0x0a;
pub const ADP5585_FIFO_9: u32 = 0x0b;
pub const ADP5585_FIFO_10: u32 = 0x0c;
pub const ADP5585_FIFO_11: u32 = 0x0d;
pub const ADP5585_FIFO_12: u32 = 0x0e;
pub const ADP5585_FIFO_13: u32 = 0x0f;
pub const ADP5585_FIFO_14: u32 = 0x10;
pub const ADP5585_FIFO_15: u32 = 0x11;
pub const ADP5585_FIFO_16: u32 = 0x12;
pub const ADP5585_EV_MAX: u32 = ADP5585_FIFO_16 - ADP5585_FIFO_1 + 1;
pub const ADP5585_GPI_INT_STAT_A: u32 = 0x13;
pub const ADP5585_GPI_INT_STAT_B: u32 = 0x14;
pub const ADP5585_GPI_STATUS_A: u32 = 0x15;
pub const ADP5585_GPI_STATUS_B: u32 = 0x16;
pub const ADP5585_RPULL_CONFIG_A: u32 = 0x17;
pub const ADP5585_RPULL_CONFIG_B: u32 = 0x18;
pub const ADP5585_RPULL_CONFIG_C: u32 = 0x19;
pub const ADP5585_RPULL_CONFIG_D: u32 = 0x1a;
pub const ADP5585_Rx_PULL_CFG_PU_300K: u32 = 0;
pub const ADP5585_Rx_PULL_CFG_PD_300K: u32 = 1;
pub const ADP5585_Rx_PULL_CFG_PU_100K: u32 = 2;
pub const ADP5585_Rx_PULL_CFG_DISABLE: u32 = 3;
pub const ADP5585_Rx_PULL_CFG_MASK: u32 = 3;
pub const ADP5585_GPI_INT_LEVEL_A: u32 = 0x1b;
pub const ADP5585_GPI_INT_LEVEL_B: u32 = 0x1c;
pub const ADP5585_GPI_EVENT_EN_A: u32 = 0x1d;
pub const ADP5585_GPI_EVENT_EN_B: u32 = 0x1e;
pub const ADP5585_GPI_INTERRUPT_EN_A: u32 = 0x1f;
pub const ADP5585_GPI_INTERRUPT_EN_B: u32 = 0x20;
pub const ADP5585_DEBOUNCE_DIS_A: u32 = 0x21;
pub const ADP5585_DEBOUNCE_DIS_B: u32 = 0x22;
pub const ADP5585_GPO_DATA_OUT_A: u32 = 0x23;
pub const ADP5585_GPO_DATA_OUT_B: u32 = 0x24;
pub const ADP5585_GPO_OUT_MODE_A: u32 = 0x25;
pub const ADP5585_GPO_OUT_MODE_B: u32 = 0x26;
pub const ADP5585_GPIO_DIRECTION_A: u32 = 0x27;
pub const ADP5585_GPIO_DIRECTION_B: u32 = 0x28;
pub const ADP5585_RESET1_EVENT_A: u32 = 0x29;
pub const ADP5585_RESET_EV_PRESS: u32 = BIT!(7);
pub const ADP5585_RESET1_EVENT_B: u32 = 0x2a;
pub const ADP5585_RESET1_EVENT_C: u32 = 0x2b;
pub const ADP5585_RESET2_EVENT_A: u32 = 0x2c;
pub const ADP5585_RESET2_EVENT_B: u32 = 0x2d;
pub const ADP5585_RESET_CFG: u32 = 0x2e;
pub const ADP5585_PWM_OFFT_LOW: u32 = 0x2f;
pub const ADP5585_PWM_OFFT_HIGH: u32 = 0x30;
pub const ADP5585_PWM_ONT_LOW: u32 = 0x31;
pub const ADP5585_PWM_ONT_HIGH: u32 = 0x32;
pub const ADP5585_PWM_CFG: u32 = 0x33;
pub const ADP5585_PWM_IN_AND: u32 = BIT!(2);
pub const ADP5585_PWM_MODE: u32 = BIT!(1);
pub const ADP5585_PWM_EN: u32 = BIT!(0);
pub const ADP5585_LOGIC_CFG: u32 = 0x34;
pub const ADP5585_LOGIC_FF_CFG: u32 = 0x35;
pub const ADP5585_LOGIC_INT_EVENT_EN: u32 = 0x36;
pub const ADP5585_POLL_PTIME_CFG: u32 = 0x37;
pub const ADP5585_PIN_CONFIG_A: u32 = 0x38;
pub const ADP5585_PIN_CONFIG_B: u32 = 0x39;
pub const ADP5585_PIN_CONFIG_C: u32 = 0x3a;
pub const ADP5585_PULL_SELECT: u32 = BIT!(7);
pub const ADP5585_C4_EXTEND_CFG_GPIO11: u32 = 0u32 << 6;
pub const ADP5585_C4_EXTEND_CFG_RESET2: u32 = 1u32 << 6;
pub const ADP5585_C4_EXTEND_CFG_MASK: u32 = GENMASK!(6, 6);
pub const ADP5585_R4_EXTEND_CFG_GPIO5: u32 = 0u32 << 5;
pub const ADP5585_R4_EXTEND_CFG_RESET1: u32 = 1u32 << 5;
pub const ADP5585_R4_EXTEND_CFG_MASK: u32 = GENMASK!(5, 5);
pub const ADP5585_R3_EXTEND_CFG_GPIO4: u32 = 0u32 << 2;
pub const ADP5585_R3_EXTEND_CFG_LC: u32 = 1u32 << 2;
pub const ADP5585_R3_EXTEND_CFG_PWM_OUT: u32 = 2u32 << 2;
pub const ADP5585_R3_EXTEND_CFG_MASK: u32 = GENMASK!(3, 2);
pub const ADP5585_R0_EXTEND_CFG_GPIO1: u32 = 0;
pub const ADP5585_R0_EXTEND_CFG_LY: u32 = 1;
pub const ADP5585_R0_EXTEND_CFG_MASK: u32 = GENMASK!(0, 0);
pub const ADP5585_GENERAL_CFG: u32 = 0x3b;
pub const ADP5585_OSC_EN: u32 = BIT!(7);
pub const ADP5585_OSC_FREQ_50KHZ: u32 = 0;
pub const ADP5585_OSC_FREQ_100KHZ: u32 = 1u32 << 5;
pub const ADP5585_OSC_FREQ_200KHZ: u32 = 2u32 << 5;
pub const ADP5585_OSC_FREQ_500KHZ: u32 = 3u32 << 5;
pub const ADP5585_OSC_FREQ_MASK: u32 = GENMASK!(6, 5);
pub const ADP5585_INT_CFG: u32 = BIT!(1);
pub const ADP5585_RST_CFG: u32 = BIT!(0);
pub const ADP5585_INT_EN: u32 = 0x3c;
pub const ADP5585_OVRFLOW_IEN: u32 = BIT!(2);
pub const ADP5585_EVENT_IEN: u32 = BIT!(0);
pub const ADP5585_MAX_REG: u32 = ADP5585_INT_EN;
pub const ADP5585_PIN_MAX: u32 = 11;
pub const ADP5585_MAX_UNLOCK_TIME_SEC: u32 = 7;
pub const ADP5585_KEY_EVENT_START: u32 = 1;
pub const ADP5585_KEY_EVENT_END: u32 = 25;
pub const ADP5585_GPI_EVENT_START: u32 = 37;
pub const ADP5585_GPI_EVENT_END: u32 = 47;
pub const ADP5585_ROW5_KEY_EVENT_START: u32 = 1;
pub const ADP5585_ROW5_KEY_EVENT_END: u32 = 30;
pub const ADP5585_PWM_OUT: u32 = 3;
pub const ADP5585_RESET1_OUT: u32 = 4;
pub const ADP5585_RESET2_OUT: u32 = 9;
pub const ADP5585_ROW5: u32 = 5;

/* ADP5589 */
pub const ADP5589_MAN_ID_VALUE: u32 = 0x10;
pub const ADP5589_GPI_STATUS_A: u32 = 0x16;
pub const ADP5589_GPI_STATUS_C: u32 = 0x18;
pub const ADP5589_RPULL_CONFIG_A: u32 = 0x19;
pub const ADP5589_GPI_INT_LEVEL_A: u32 = 0x1e;
pub const ADP5589_GPI_EVENT_EN_A: u32 = 0x21;
pub const ADP5589_DEBOUNCE_DIS_A: u32 = 0x27;
pub const ADP5589_GPO_DATA_OUT_A: u32 = 0x2a;
pub const ADP5589_GPO_OUT_MODE_A: u32 = 0x2d;
pub const ADP5589_GPIO_DIRECTION_A: u32 = 0x30;
pub const ADP5589_UNLOCK1: u32 = 0x33;
pub const ADP5589_UNLOCK_EV_PRESS: u32 = BIT!(7);
pub const ADP5589_UNLOCK_TIMERS: u32 = 0x36;
pub const ADP5589_UNLOCK_TIMER: u32 = GENMASK!(2, 0);
pub const ADP5589_LOCK_CFG: u32 = 0x37;
pub const ADP5589_LOCK_EN: u32 = BIT!(0);
pub const ADP5589_RESET1_EVENT_A: u32 = 0x38;
pub const ADP5589_RESET2_EVENT_A: u32 = 0x3B;
pub const ADP5589_RESET_CFG: u32 = 0x3D;
pub const ADP5585_RESET2_POL: u32 = BIT!(7);
pub const ADP5585_RESET1_POL: u32 = BIT!(6);
pub const ADP5585_RST_PASSTHRU_EN: u32 = BIT!(5);
pub const ADP5585_RESET_TRIG_TIME: u32 = GENMASK!(4, 2);
pub const ADP5585_PULSE_WIDTH: u32 = GENMASK!(1, 0);
pub const ADP5589_PWM_OFFT_LOW: u32 = 0x3e;
pub const ADP5589_PWM_ONT_LOW: u32 = 0x40;
pub const ADP5589_PWM_CFG: u32 = 0x42;
pub const ADP5589_POLL_PTIME_CFG: u32 = 0x48;
pub const ADP5589_PIN_CONFIG_A: u32 = 0x49;
pub const ADP5589_PIN_CONFIG_D: u32 = 0x4C;
pub const ADP5589_GENERAL_CFG: u32 = 0x4d;
pub const ADP5589_INT_EN: u32 = 0x4e;
pub const ADP5589_MAX_REG: u32 = ADP5589_INT_EN;
pub const ADP5589_PIN_MAX: u32 = 19;
pub const ADP5589_KEY_EVENT_START: u32 = 1;
pub const ADP5589_KEY_EVENT_END: u32 = 88;
pub const ADP5589_GPI_EVENT_START: u32 = 97;
pub const ADP5589_GPI_EVENT_END: u32 = 115;
pub const ADP5589_UNLOCK_WILDCARD: u32 = 127;
pub const ADP5589_RESET2_OUT: u32 = 12;

#[repr(C)]
pub struct regmap;
#[repr(C)]
pub struct device;
#[repr(C)]
pub struct blocking_notifier_head;

#[repr(C)]
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum adp5585_variant {
    ADP5585_00 = 1,
    ADP5585_01,
    ADP5585_02,
    ADP5585_03,
    ADP5585_04,
    ADP5589_00,
    ADP5589_01,
    ADP5589_02,
    ADP5585_MAX,
}

#[repr(C)]
pub struct adp5585_regs {
    pub gen_cfg: u32,
    pub ext_cfg: u32,
    pub int_en: u32,
    pub poll_ptime_cfg: u32,
    pub reset_cfg: u32,
    pub reset1_event_a: u32,
    pub reset2_event_a: u32,
    pub pin_cfg_a: u32,
}

#[repr(C)]
pub struct adp5585_dev {
    pub dev: *mut device,
    pub regmap: *mut regmap,
    pub regs: *const adp5585_regs,
    pub event_notifier: blocking_notifier_head,
    pub pin_usage: *mut usize,
    pub n_pins: u32,
    pub reset2_out: u32,
    pub variant: adp5585_variant,
    pub id: u32,
    pub has_unlock: bool,
    pub has_pin6: bool,
    pub irq: i32,
    pub ev_poll_time: u32,
    pub unlock_time: u32,
    pub unlock_keys: [u32; 2],
    pub nkeys_unlock: u32,
    pub reset1_keys: [u32; 3],
    pub nkeys_reset1: u32,
    pub reset2_keys: [u32; 2],
    pub nkeys_reset2: u32,
    pub reset_cfg: u8,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
