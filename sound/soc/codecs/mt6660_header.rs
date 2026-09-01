// SPDX-License-Identifier: GPL-2.0
/*
 * Copyright (c) 2019 MediaTek Inc.
 */

// C header dependencies: <linux/mutex.h>, <linux/regmap.h>.

#[repr(C, packed)]
pub struct mt6660_platform_data {
    pub init_setting_num: u8,
    pub init_setting_addr: *mut u32,
    pub init_setting_mask: *mut u32,
    pub init_setting_val: *mut u32,
}

#[repr(C, packed)]
pub struct mt6660_chip {
    pub i2c: *mut i2c_client,
    pub dev: *mut device,
    pub param_dev: *mut platform_device,
    pub plat_data: mt6660_platform_data,
    pub io_lock: mutex,
    pub regmap: *mut regmap,
    pub chip_rev: u16,
}

pub const MT6660_REG_DEVID: u32 = 0x00;
pub const MT6660_REG_SYSTEM_CTRL: u32 = 0x03;
pub const MT6660_REG_IRQ_STATUS1: u32 = 0x05;
pub const MT6660_REG_ADDA_CLOCK: u32 = 0x07;
pub const MT6660_REG_SERIAL_CFG1: u32 = 0x10;
pub const MT6660_REG_DATAO_SEL: u32 = 0x12;
pub const MT6660_REG_TDM_CFG3: u32 = 0x15;
pub const MT6660_REG_HPF_CTRL: u32 = 0x18;
pub const MT6660_REG_HPF1_COEF: u32 = 0x1A;
pub const MT6660_REG_HPF2_COEF: u32 = 0x1B;
pub const MT6660_REG_PATH_BYPASS: u32 = 0x1E;
pub const MT6660_REG_WDT_CTRL: u32 = 0x20;
pub const MT6660_REG_HCLIP_CTRL: u32 = 0x24;
pub const MT6660_REG_VOL_CTRL: u32 = 0x29;
pub const MT6660_REG_SPS_CTRL: u32 = 0x30;
pub const MT6660_REG_SIGMAX: u32 = 0x33;
pub const MT6660_REG_CALI_T0: u32 = 0x3F;
pub const MT6660_REG_BST_CTRL: u32 = 0x40;
pub const MT6660_REG_PROTECTION_CFG: u32 = 0x46;
pub const MT6660_REG_DA_GAIN: u32 = 0x4c;
pub const MT6660_REG_AUDIO_IN2_SEL: u32 = 0x50;
pub const MT6660_REG_SIG_GAIN: u32 = 0x51;
pub const MT6660_REG_PLL_CFG1: u32 = 0x60;
pub const MT6660_REG_DRE_CTRL: u32 = 0x68;
pub const MT6660_REG_DRE_THDMODE: u32 = 0x69;
pub const MT6660_REG_DRE_CORASE: u32 = 0x6B;
pub const MT6660_REG_PWM_CTRL: u32 = 0x70;
pub const MT6660_REG_DC_PROTECT_CTRL: u32 = 0x74;
pub const MT6660_REG_ADC_USB_MODE: u32 = 0x7c;
pub const MT6660_REG_INTERNAL_CFG: u32 = 0x88;
pub const MT6660_REG_RESV0: u32 = 0x98;
pub const MT6660_REG_RESV1: u32 = 0x99;
pub const MT6660_REG_RESV2: u32 = 0x9A;
pub const MT6660_REG_RESV3: u32 = 0x9B;
pub const MT6660_REG_RESV6: u32 = 0xA2;
pub const MT6660_REG_RESV7: u32 = 0xA3;
pub const MT6660_REG_RESV10: u32 = 0xB0;
pub const MT6660_REG_RESV11: u32 = 0xB1;
pub const MT6660_REG_RESV16: u32 = 0xB6;
pub const MT6660_REG_RESV17: u32 = 0xB7;
pub const MT6660_REG_RESV19: u32 = 0xB9;
pub const MT6660_REG_RESV21: u32 = 0xBB;
pub const MT6660_REG_RESV23: u32 = 0xBD;
pub const MT6660_REG_RESV31: u32 = 0xD3;
pub const MT6660_REG_RESV40: u32 = 0xE0;

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
