/* SPDX-License-Identifier: GPL-2.0
 *
 * ALSA SoC TAS2770 codec driver
 *
 *  Copyright (C) 2016-2017 Texas Instruments Incorporated - https://www.ti.com/
 */

// Header guard __TAS2770__ omitted in Rust.

pub enum snd_soc_component {}
pub enum gpio_desc {}
pub enum regmap {}
pub enum device {}

const fn BIT(nr: u32) -> u32 {
    1u32 << nr
}

const fn GENMASK(h: u32, l: u32) -> u32 {
    (!0u32 << l) & (!0u32 >> (u32::BITS - 1 - h))
}

/* Book Control Register (available in page0 of each book) */
pub const TAS2770_BOOKCTL_PAGE: u32 = 0;
pub const TAS2770_BOOKCTL_REG: u32 = 127;
pub const fn TAS2770_REG(page: u32, reg: u32) -> u32 {
    (page * 128) + reg
}
/* Page */
pub const TAS2770_PAGE: u32 = TAS2770_REG(0X0, 0x00);
pub const TAS2770_PAGE_PAGE_MASK: u32 = 255;
/* Software Reset */
pub const TAS2770_SW_RST: u32 = TAS2770_REG(0X0, 0x01);
pub const TAS2770_RST: u32 = BIT(0);
/* Power Control */
pub const TAS2770_PWR_CTRL: u32 = TAS2770_REG(0X0, 0x02);
pub const TAS2770_PWR_CTRL_MASK: u32 = GENMASK(1, 0);
pub const TAS2770_PWR_CTRL_ACTIVE: u32 = 0x0;
pub const TAS2770_PWR_CTRL_MUTE: u32 = BIT(0);
pub const TAS2770_PWR_CTRL_SHUTDOWN: u32 = 0x2;
/* Playback Configuration Reg0 */
pub const TAS2770_PLAY_CFG_REG0: u32 = TAS2770_REG(0X0, 0x03);
/* Playback Configuration Reg1 */
pub const TAS2770_PLAY_CFG_REG1: u32 = TAS2770_REG(0X0, 0x04);
/* Playback Configuration Reg2 */
pub const TAS2770_PLAY_CFG_REG2: u32 = TAS2770_REG(0X0, 0x05);
pub const TAS2770_PLAY_CFG_REG2_VMAX: u32 = 0xc9;
/* Misc Configuration Reg0 */
pub const TAS2770_MSC_CFG_REG0: u32 = TAS2770_REG(0X0, 0x07);
/* TDM Configuration Reg0 */
pub const TAS2770_TDM_CFG_REG0: u32 = TAS2770_REG(0X0, 0x0A);
pub const TAS2770_TDM_CFG_REG0_SMP_MASK: u32 = BIT(5);
pub const TAS2770_TDM_CFG_REG0_SMP_48KHZ: u32 = 0x0;
pub const TAS2770_TDM_CFG_REG0_SMP_44_1KHZ: u32 = BIT(5);
pub const TAS2770_TDM_CFG_REG0_31_MASK: u32 = GENMASK(3, 1);
pub const TAS2770_TDM_CFG_REG0_31_44_1_48KHZ: u32 = 0x6;
pub const TAS2770_TDM_CFG_REG0_31_88_2_96KHZ: u32 = 0x8;
pub const TAS2770_TDM_CFG_REG0_31_176_4_192KHZ: u32 = 0xa;
pub const TAS2770_TDM_CFG_REG0_FPOL_MASK: u32 = BIT(0);
pub const TAS2770_TDM_CFG_REG0_FPOL_RSING: u32 = 0;
pub const TAS2770_TDM_CFG_REG0_FPOL_FALING: u32 = 1;
/* TDM Configuration Reg1 */
pub const TAS2770_TDM_CFG_REG1: u32 = TAS2770_REG(0X0, 0x0B);
pub const TAS2770_TDM_CFG_REG1_MASK: u32 = GENMASK(5, 1);
pub const TAS2770_TDM_CFG_REG1_51_SHIFT: u32 = 1;
pub const TAS2770_TDM_CFG_REG1_RX_MASK: u32 = BIT(0);
pub const TAS2770_TDM_CFG_REG1_RX_RSING: u32 = 0x0;
pub const TAS2770_TDM_CFG_REG1_RX_FALING: u32 = BIT(0);
/* TDM Configuration Reg2 */
pub const TAS2770_TDM_CFG_REG2: u32 = TAS2770_REG(0X0, 0x0C);
pub const TAS2770_TDM_CFG_REG2_RXW_MASK: u32 = GENMASK(3, 2);
pub const TAS2770_TDM_CFG_REG2_RXW_16BITS: u32 = 0x0;
pub const TAS2770_TDM_CFG_REG2_RXW_24BITS: u32 = 0x8;
pub const TAS2770_TDM_CFG_REG2_RXW_32BITS: u32 = 0xc;
pub const TAS2770_TDM_CFG_REG2_RXS_MASK: u32 = GENMASK(1, 0);
pub const TAS2770_TDM_CFG_REG2_RXS_16BITS: u32 = 0x0;
pub const TAS2770_TDM_CFG_REG2_RXS_24BITS: u32 = BIT(0);
pub const TAS2770_TDM_CFG_REG2_RXS_32BITS: u32 = 0x2;
/* TDM Configuration Reg3 */
pub const TAS2770_TDM_CFG_REG3: u32 = TAS2770_REG(0X0, 0x0D);
pub const TAS2770_TDM_CFG_REG3_RXS_MASK: u32 = GENMASK(7, 4);
pub const TAS2770_TDM_CFG_REG3_RXS_SHIFT: u32 = 0x4;
pub const TAS2770_TDM_CFG_REG3_30_MASK: u32 = GENMASK(3, 0);
pub const TAS2770_TDM_CFG_REG3_30_SHIFT: u32 = 0;
/* TDM Configuration Reg4 */
pub const TAS2770_TDM_CFG_REG4: u32 = TAS2770_REG(0X0, 0x0E);
pub const TAS2770_TDM_CFG_REG4_TX_LSB_CFG: u32 = BIT(7);
pub const TAS2770_TDM_CFG_REG4_TX_KEEPER_CFG: u32 = BIT(6);
pub const TAS2770_TDM_CFG_REG4_TX_KEEPER: u32 = BIT(5);
pub const TAS2770_TDM_CFG_REG4_TX_FILL: u32 = BIT(4);
pub const TAS2770_TDM_CFG_REG4_TX_OFFSET_MASK: u32 = GENMASK(3, 1);
pub const TAS2770_TDM_CFG_REG4_TX_EDGE_FALLING: u32 = BIT(0);
/* TDM Configuration Reg5 */
pub const TAS2770_TDM_CFG_REG5: u32 = TAS2770_REG(0X0, 0x0F);
pub const TAS2770_TDM_CFG_REG5_VSNS_MASK: u32 = BIT(6);
pub const TAS2770_TDM_CFG_REG5_VSNS_ENABLE: u32 = BIT(6);
pub const TAS2770_TDM_CFG_REG5_50_MASK: u32 = GENMASK(5, 0);
/* TDM Configuration Reg6 */
pub const TAS2770_TDM_CFG_REG6: u32 = TAS2770_REG(0X0, 0x10);
pub const TAS2770_TDM_CFG_REG6_ISNS_MASK: u32 = BIT(6);
pub const TAS2770_TDM_CFG_REG6_ISNS_ENABLE: u32 = BIT(6);
pub const TAS2770_TDM_CFG_REG6_50_MASK: u32 = GENMASK(5, 0);
/* TDM Configuration Reg10 */
pub const TAS2770_TDM_CFG_REG7: u32 = TAS2770_REG(0X0, 0x11);
pub const TAS2770_TDM_CFG_REG7_PDM_MASK: u32 = BIT(6);
pub const TAS2770_TDM_CFG_REG7_PDM_ENABLE: u32 = BIT(6);
pub const TAS2770_TDM_CFG_REG7_50_MASK: u32 = GENMASK(5, 0);
/* Brown Out Prevention Reg0 */
pub const TAS2770_BO_PRV_REG0: u32 = TAS2770_REG(0X0, 0x1B);
/* Interrupt MASK Reg0 */
pub const TAS2770_INT_MASK_REG0: u32 = TAS2770_REG(0X0, 0x20);
pub const TAS2770_INT_REG0_DEFAULT: u32 = 0xfc;
pub const TAS2770_INT_MASK_REG0_DISABLE: u32 = 0xff;
/* Interrupt MASK Reg1 */
pub const TAS2770_INT_MASK_REG1: u32 = TAS2770_REG(0X0, 0x21);
pub const TAS2770_INT_REG1_DEFAULT: u32 = 0xb1;
pub const TAS2770_INT_MASK_REG1_DISABLE: u32 = 0xff;
/* Live-Interrupt Reg0 */
pub const TAS2770_LVE_INT_REG0: u32 = TAS2770_REG(0X0, 0x22);
/* Live-Interrupt Reg1 */
pub const TAS2770_LVE_INT_REG1: u32 = TAS2770_REG(0X0, 0x23);
/* Latched-Interrupt Reg0 */
pub const TAS2770_LAT_INT_REG0: u32 = TAS2770_REG(0X0, 0x24);
pub const TAS2770_LAT_INT_REG0_OCE_FLG: u32 = BIT(1);
pub const TAS2770_LAT_INT_REG0_OTE_FLG: u32 = BIT(0);
/* Latched-Interrupt Reg1 */
pub const TAS2770_LAT_INT_REG1: u32 = TAS2770_REG(0X0, 0x25);
pub const TAS2770_LAT_INT_REG1_VBA_TOV: u32 = BIT(3);
pub const TAS2770_LAT_INT_REG1_VBA_TUV: u32 = BIT(2);
pub const TAS2770_LAT_INT_REG1_BOUT_FLG: u32 = BIT(1);
/* VBAT MSB */
pub const TAS2770_VBAT_MSB: u32 = TAS2770_REG(0X0, 0x27);
/* VBAT LSB */
pub const TAS2770_VBAT_LSB: u32 = TAS2770_REG(0X0, 0x28);
/* TEMP MSB */
pub const TAS2770_TEMP_MSB: u32 = TAS2770_REG(0X0, 0x29);
/* TEMP LSB */
pub const TAS2770_TEMP_LSB: u32 = TAS2770_REG(0X0, 0x2A);
/* Interrupt Configuration */
pub const TAS2770_INT_CFG: u32 = TAS2770_REG(0X0, 0x30);
/* Data In Pull-Down */
pub const TAS2770_DIN_PD: u32 = TAS2770_REG(0X0, 0x31);
pub const TAS2770_DIN_PD_SDOUT: u32 = BIT(7);
/* Misc IRQ */
pub const TAS2770_MISC_IRQ: u32 = TAS2770_REG(0X0, 0x32);
/* Clock Configuration */
pub const TAS2770_CLK_CGF: u32 = TAS2770_REG(0X0, 0x3C);
/* TDM Clock detection monitor */
pub const TAS2770_TDM_CLK_DETC: u32 = TAS2770_REG(0X0, 0x77);
/* Revision and PG ID */
pub const TAS2770_REV_AND_GPID: u32 = TAS2770_REG(0X0, 0x7D);

pub const TAS2770_POWER_ACTIVE: u32 = 0;
pub const TAS2770_POWER_MUTE: u32 = BIT(0);
pub const TAS2770_POWER_SHUTDOWN: u32 = BIT(1);

pub const ERROR_OVER_CURRENT: u32 = BIT(0);
pub const ERROR_DIE_OVERTEMP: u32 = BIT(1);
pub const ERROR_OVER_VOLTAGE: u32 = BIT(2);
pub const ERROR_UNDER_VOLTAGE: u32 = BIT(3);
pub const ERROR_BROWNOUT: u32 = BIT(4);
pub const ERROR_CLASSD_PWR: u32 = BIT(5);

#[repr(C)]
pub struct tas2770_priv {
    pub component: *mut snd_soc_component,
    pub reset_gpio: *mut gpio_desc,
    pub sdz_gpio: *mut gpio_desc,
    pub regmap: *mut regmap,
    pub dev: *mut device,
    pub v_sense_slot: ::std::os::raw::c_int,
    pub i_sense_slot: ::std::os::raw::c_int,
    pub pdm_slot: ::std::os::raw::c_int,
    pub dac_powered: bool,
    pub unmuted: bool,
    pub idle_tx_mode: ::std::os::raw::c_int,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
