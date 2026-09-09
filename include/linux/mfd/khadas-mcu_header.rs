/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Khadas System control Microcontroller Register map
 *
 * Copyright (C) 2020 BayLibre SAS
 *
 * Author(s): Neil Armstrong <narmstrong@baylibre.com>
 */

pub const KHADAS_MCU_PASSWD_VEN_0_REG: u32 = 0x00; /* RO */
pub const KHADAS_MCU_PASSWD_VEN_1_REG: u32 = 0x01; /* RO */
pub const KHADAS_MCU_PASSWD_VEN_2_REG: u32 = 0x02; /* RO */
pub const KHADAS_MCU_PASSWD_VEN_3_REG: u32 = 0x03; /* RO */
pub const KHADAS_MCU_PASSWD_VEN_4_REG: u32 = 0x04; /* RO */
pub const KHADAS_MCU_PASSWD_VEN_5_REG: u32 = 0x05; /* RO */
pub const KHADAS_MCU_MAC_0_REG: u32 = 0x06; /* RO */
pub const KHADAS_MCU_MAC_1_REG: u32 = 0x07; /* RO */
pub const KHADAS_MCU_MAC_2_REG: u32 = 0x08; /* RO */
pub const KHADAS_MCU_MAC_3_REG: u32 = 0x09; /* RO */
pub const KHADAS_MCU_MAC_4_REG: u32 = 0x0a; /* RO */
pub const KHADAS_MCU_MAC_5_REG: u32 = 0x0b; /* RO */
pub const KHADAS_MCU_USID_0_REG: u32 = 0x0c; /* RO */
pub const KHADAS_MCU_USID_1_REG: u32 = 0x0d; /* RO */
pub const KHADAS_MCU_USID_2_REG: u32 = 0x0e; /* RO */
pub const KHADAS_MCU_USID_3_REG: u32 = 0x0f; /* RO */
pub const KHADAS_MCU_USID_4_REG: u32 = 0x10; /* RO */
pub const KHADAS_MCU_USID_5_REG: u32 = 0x11; /* RO */
pub const KHADAS_MCU_VERSION_0_REG: u32 = 0x12; /* RO */
pub const KHADAS_MCU_VERSION_1_REG: u32 = 0x13; /* RO */
pub const KHADAS_MCU_DEVICE_NO_0_REG: u32 = 0x14; /* RO */
pub const KHADAS_MCU_DEVICE_NO_1_REG: u32 = 0x15; /* RO */
pub const KHADAS_MCU_FACTORY_TEST_REG: u32 = 0x16; /* R */
pub const KHADAS_MCU_BOOT_MODE_REG: u32 = 0x20; /* RW */
pub const KHADAS_MCU_BOOT_EN_WOL_REG: u32 = 0x21; /* RW */
pub const KHADAS_MCU_BOOT_EN_RTC_REG: u32 = 0x22; /* RW */
pub const KHADAS_MCU_BOOT_EN_EXP_REG: u32 = 0x23; /* RW */
pub const KHADAS_MCU_BOOT_EN_IR_REG: u32 = 0x24; /* RW */
pub const KHADAS_MCU_BOOT_EN_DCIN_REG: u32 = 0x25; /* RW */
pub const KHADAS_MCU_BOOT_EN_KEY_REG: u32 = 0x26; /* RW */
pub const KHADAS_MCU_KEY_MODE_REG: u32 = 0x27; /* RW */
pub const KHADAS_MCU_LED_MODE_ON_REG: u32 = 0x28; /* RW */
pub const KHADAS_MCU_LED_MODE_OFF_REG: u32 = 0x29; /* RW */
pub const KHADAS_MCU_SHUTDOWN_NORMAL_REG: u32 = 0x2c; /* RW */
pub const KHADAS_MCU_MAC_SWITCH_REG: u32 = 0x2d; /* RW */
pub const KHADAS_MCU_MCU_SLEEP_MODE_REG: u32 = 0x2e; /* RW */
pub const KHADAS_MCU_IR_CODE1_0_REG: u32 = 0x2f; /* RW */
pub const KHADAS_MCU_IR_CODE1_1_REG: u32 = 0x30; /* RW */
pub const KHADAS_MCU_IR_CODE1_2_REG: u32 = 0x31; /* RW */
pub const KHADAS_MCU_IR_CODE1_3_REG: u32 = 0x32; /* RW */
pub const KHADAS_MCU_USB_PCIE_SWITCH_REG: u32 = 0x33; /* RW */
pub const KHADAS_MCU_IR_CODE2_0_REG: u32 = 0x34; /* RW */
pub const KHADAS_MCU_IR_CODE2_1_REG: u32 = 0x35; /* RW */
pub const KHADAS_MCU_IR_CODE2_2_REG: u32 = 0x36; /* RW */
pub const KHADAS_MCU_IR_CODE2_3_REG: u32 = 0x37; /* RW */
pub const KHADAS_MCU_PASSWD_USER_0_REG: u32 = 0x40; /* RW */
pub const KHADAS_MCU_PASSWD_USER_1_REG: u32 = 0x41; /* RW */
pub const KHADAS_MCU_PASSWD_USER_2_REG: u32 = 0x42; /* RW */
pub const KHADAS_MCU_PASSWD_USER_3_REG: u32 = 0x43; /* RW */
pub const KHADAS_MCU_PASSWD_USER_4_REG: u32 = 0x44; /* RW */
pub const KHADAS_MCU_PASSWD_USER_5_REG: u32 = 0x45; /* RW */
pub const KHADAS_MCU_USER_DATA_0_REG: u32 = 0x46; /* RW 56 bytes */
pub const KHADAS_MCU_PWR_OFF_CMD_REG: u32 = 0x80; /* WO */
pub const KHADAS_MCU_PASSWD_START_REG: u32 = 0x81; /* WO */
pub const KHADAS_MCU_CHECK_VEN_PASSWD_REG: u32 = 0x82; /* WO */
pub const KHADAS_MCU_CHECK_USER_PASSWD_REG: u32 = 0x83; /* WO */
pub const KHADAS_MCU_SHUTDOWN_NORMAL_STATUS_REG: u32 = 0x86; /* RO */
pub const KHADAS_MCU_WOL_INIT_START_REG: u32 = 0x87; /* WO */
pub const KHADAS_MCU_CMD_FAN_STATUS_CTRL_REG: u32 = 0x88; /* WO */

#[repr(u32)]
pub enum KhadasBoard {
    KHADAS_BOARD_VIM1 = 0x1,
    KHADAS_BOARD_VIM2,
    KHADAS_BOARD_VIM3,
    KHADAS_BOARD_EDGE = 0x11,
    KHADAS_BOARD_EDGE_V,
}

/* External dependency declarations supplied by other files. */
pub struct device;
pub struct regmap;

/**
 * struct khadas_mcu - Khadas MCU structure
 * @device:             device reference used for logs
 * @regmap:             register map
 */
#[repr(C)]
pub struct khadas_mcu {
    pub dev: *mut device,
    pub regmap: *mut regmap,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
