/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * ROHM BD9571MWV-M and BD9574MWF-M driver
 *
 * Copyright (C) 2017 Marek Vasut <marek.vasut+renesas@gmail.com>
 * Copyright (C) 2020 Renesas Electronics Corporation
 *
 * Based on the TPS65086 driver
 */

// C dependencies: <linux/device.h> and <linux/regmap.h>.

/* List of registers for BD9571MWV and BD9574MWF */
pub const BD9571MWV_VENDOR_CODE: i32 = 0x00;
pub const BD9571MWV_VENDOR_CODE_VAL: i32 = 0xdb;
pub const BD9571MWV_PRODUCT_CODE: i32 = 0x01;
pub const BD9571MWV_PRODUCT_CODE_BD9571MWV: i32 = 0x60;
pub const BD9571MWV_PRODUCT_CODE_BD9574MWF: i32 = 0x74;
pub const BD9571MWV_PRODUCT_REVISION: i32 = 0x02;

pub const BD9571MWV_I2C_FUSA_MODE: i32 = 0x10;
pub const BD9571MWV_I2C_MD2_E1_BIT_1: i32 = 0x11;
pub const BD9571MWV_I2C_MD2_E1_BIT_2: i32 = 0x12;

pub const BD9571MWV_BKUP_MODE_CNT: i32 = 0x20;
pub const BD9571MWV_BKUP_MODE_CNT_KEEPON_MASK: i32 = 0x0f;
pub const BD9571MWV_BKUP_MODE_CNT_KEEPON_DDR0: i32 = 1 << 0;
pub const BD9571MWV_BKUP_MODE_CNT_KEEPON_DDR1: i32 = 1 << 1;
pub const BD9571MWV_BKUP_MODE_CNT_KEEPON_DDR0C: i32 = 1 << 2;
pub const BD9571MWV_BKUP_MODE_CNT_KEEPON_DDR1C: i32 = 1 << 3;
pub const BD9571MWV_BKUP_MODE_STATUS: i32 = 0x21;
pub const BD9571MWV_BKUP_RECOVERY_CNT: i32 = 0x22;
pub const BD9571MWV_BKUP_CTRL_TIM_CNT: i32 = 0x23;
pub const BD9571MWV_WAITBKUP_WDT_CNT: i32 = 0x24;
pub const BD9571MWV_128H_TIM_CNT: i32 = 0x26;
pub const BD9571MWV_QLLM_CNT: i32 = 0x27;

pub const BD9571MWV_AVS_SET_MONI: i32 = 0x31;
pub const BD9571MWV_AVS_SET_MONI_MASK: i32 = 0x3;
#[macro_export]
macro_rules! BD9571MWV_AVS_VD09_VID { ($n:expr) => { 0x32 + ($n) }; }
#[macro_export]
macro_rules! BD9571MWV_AVS_DVFS_VID { ($n:expr) => { 0x36 + ($n) }; }

pub const BD9571MWV_VD18_VID: i32 = 0x42;
pub const BD9571MWV_VD25_VID: i32 = 0x43;
pub const BD9571MWV_VD33_VID: i32 = 0x44;

pub const BD9571MWV_DVFS_VINIT: i32 = 0x50;
pub const BD9574MWF_VD09_VINIT: i32 = 0x51;
pub const BD9571MWV_DVFS_SETVMAX: i32 = 0x52;
pub const BD9571MWV_DVFS_BOOSTVID: i32 = 0x53;
pub const BD9571MWV_DVFS_SETVID: i32 = 0x54;
pub const BD9571MWV_DVFS_MONIVDAC: i32 = 0x55;
pub const BD9571MWV_DVFS_PGD_CNT: i32 = 0x56;

pub const BD9571MWV_GPIO_DIR: i32 = 0x60;
pub const BD9571MWV_GPIO_OUT: i32 = 0x61;
pub const BD9571MWV_GPIO_IN: i32 = 0x62;
pub const BD9571MWV_GPIO_DEB: i32 = 0x63;
pub const BD9571MWV_GPIO_INT_SET: i32 = 0x64;
pub const BD9571MWV_GPIO_INT: i32 = 0x65;
pub const BD9571MWV_GPIO_INTMASK: i32 = 0x66;
pub const BD9574MWF_GPIO_MUX: i32 = 0x67;

#[macro_export]
macro_rules! BD9571MWV_REG_KEEP { ($n:expr) => { 0x70 + ($n) }; }

pub const BD9571MWV_PMIC_INTERNAL_STATUS: i32 = 0x80;
pub const BD9571MWV_PROT_ERROR_STATUS0: i32 = 0x81;
pub const BD9571MWV_PROT_ERROR_STATUS1: i32 = 0x82;
pub const BD9571MWV_PROT_ERROR_STATUS2: i32 = 0x83;
pub const BD9571MWV_PROT_ERROR_STATUS3: i32 = 0x84;
pub const BD9571MWV_PROT_ERROR_STATUS4: i32 = 0x85;
pub const BD9574MWF_PROT_ERROR_STATUS5: i32 = 0x86;
pub const BD9574MWF_SYSTEM_ERROR_STATUS: i32 = 0x87;

pub const BD9571MWV_INT_INTREQ: i32 = 0x90;
pub const BD9571MWV_INT_INTREQ_MD1_INT: i32 = 1 << 0;
pub const BD9571MWV_INT_INTREQ_MD2_E1_INT: i32 = 1 << 1;
pub const BD9571MWV_INT_INTREQ_MD2_E2_INT: i32 = 1 << 2;
pub const BD9571MWV_INT_INTREQ_PROT_ERR_INT: i32 = 1 << 3;
pub const BD9571MWV_INT_INTREQ_GP_INT: i32 = 1 << 4;
pub const BD9571MWV_INT_INTREQ_128H_OF_INT: i32 = 1 << 5;
pub const BD9571MWV_INT_INTREQ_WDT_OF_INT: i32 = 1 << 6;
pub const BD9571MWV_INT_INTREQ_BKUP_TRG_INT: i32 = 1 << 7;
pub const BD9571MWV_INT_INTMASK: i32 = 0x91;

pub const BD9574MWF_SSCG_CNT: i32 = 0xA0;
pub const BD9574MWF_POFFB_MRB: i32 = 0xA1;
pub const BD9574MWF_SMRB_WR_PROT: i32 = 0xA2;
pub const BD9574MWF_SMRB_ASSERT: i32 = 0xA3;
pub const BD9574MWF_SMRB_STATUS: i32 = 0xA4;

pub const BD9571MWV_ACCESS_KEY: i32 = 0xff;

/* Define the BD9571MWV IRQ numbers */
#[repr(i32)]
pub enum bd9571mwv_irqs {
    BD9571MWV_IRQ_MD1,
    BD9571MWV_IRQ_MD2_E1,
    BD9571MWV_IRQ_MD2_E2,
    BD9571MWV_IRQ_PROT_ERR,
    BD9571MWV_IRQ_GP,
    BD9571MWV_IRQ_128H_OF, // BKUP_HOLD on BD9574MWF
    BD9571MWV_IRQ_WDT_OF,
    BD9571MWV_IRQ_BKUP_TRG,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
