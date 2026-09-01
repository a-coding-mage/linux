/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * rt1318-sdw.h -- RT1318 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2022 Realtek Semiconductor Corp.
 */

/* Dependencies from the original C header:
 * linux/regmap.h
 * linux/soundwire/sdw.h
 * linux/soundwire/sdw_type.h
 * linux/soundwire/sdw_registers.h
 * sound/soc.h
 */

/* imp-defined registers */
pub const RT1318_SAPU_SM: u32 = 0x3203;

pub const R1318_TCON: u32 = 0xc203;
pub const R1318_TCON_RELATED_1: u32 = 0xc206;

pub const R1318_SPK_TEMPERATRUE_PROTECTION_0: u32 = 0xdb00;
pub const R1318_SPK_TEMPERATRUE_PROTECTION_L_4: u32 = 0xdb08;
pub const R1318_SPK_TEMPERATRUE_PROTECTION_R_4: u32 = 0xdd08;

pub const R1318_SPK_TEMPERATRUE_PROTECTION_L_6: u32 = 0xdb12;
pub const R1318_SPK_TEMPERATRUE_PROTECTION_R_6: u32 = 0xdd12;

pub const RT1318_INIT_RECIPROCAL_REG_L_24: u32 = 0xdbb5;
pub const RT1318_INIT_RECIPROCAL_REG_L_23_16: u32 = 0xdbb6;
pub const RT1318_INIT_RECIPROCAL_REG_L_15_8: u32 = 0xdbb7;
pub const RT1318_INIT_RECIPROCAL_REG_L_7_0: u32 = 0xdbb8;
pub const RT1318_INIT_RECIPROCAL_REG_R_24: u32 = 0xddb5;
pub const RT1318_INIT_RECIPROCAL_REG_R_23_16: u32 = 0xddb6;
pub const RT1318_INIT_RECIPROCAL_REG_R_15_8: u32 = 0xddb7;
pub const RT1318_INIT_RECIPROCAL_REG_R_7_0: u32 = 0xddb8;

pub const RT1318_INIT_R0_RECIPROCAL_SYN_L_24: u32 = 0xdbc5;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_L_23_16: u32 = 0xdbc6;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_L_15_8: u32 = 0xdbc7;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_L_7_0: u32 = 0xdbc8;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_R_24: u32 = 0xddc5;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_R_23_16: u32 = 0xddc6;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_R_15_8: u32 = 0xddc7;
pub const RT1318_INIT_R0_RECIPROCAL_SYN_R_7_0: u32 = 0xddc8;

pub const RT1318_R0_COMPARE_FLAG_L: u32 = 0xdb35;
pub const RT1318_R0_COMPARE_FLAG_R: u32 = 0xdd35;

pub const RT1318_STP_INITIAL_RS_TEMP_H: u32 = 0xdd93;
pub const RT1318_STP_INITIAL_RS_TEMP_L: u32 = 0xdd94;

/* RT1318 SDCA Control - function number */
pub const FUNC_NUM_SMART_AMP: u32 = 0x04;

/* RT1318 SDCA entity */
pub const RT1318_SDCA_ENT_PDE23: u32 = 0x31;
pub const RT1318_SDCA_ENT_XU24: u32 = 0x24;
pub const RT1318_SDCA_ENT_FU21: u32 = 0x03;
pub const RT1318_SDCA_ENT_UDMPU21: u32 = 0x02;
pub const RT1318_SDCA_ENT_CS21: u32 = 0x21;
pub const RT1318_SDCA_ENT_SAPU: u32 = 0x29;

/* RT1318 SDCA control */
pub const RT1318_SDCA_CTL_SAMPLE_FREQ_INDEX: u32 = 0x10;
pub const RT1318_SDCA_CTL_REQ_POWER_STATE: u32 = 0x01;
pub const RT1318_SDCA_CTL_FU_MUTE: u32 = 0x01;
pub const RT1318_SDCA_CTL_FU_VOLUME: u32 = 0x02;
pub const RT1318_SDCA_CTL_UDMPU_CLUSTER: u32 = 0x10;
pub const RT1318_SDCA_CTL_SAPU_PROTECTION_MODE: u32 = 0x10;
pub const RT1318_SDCA_CTL_SAPU_PROTECTION_STATUS: u32 = 0x11;

/* RT1318 SDCA channel */
pub const CH_L: u32 = 0x01;
pub const CH_R: u32 = 0x02;

/* sample frequency index */
pub const RT1318_SDCA_RATE_16000HZ: u32 = 0x04;
pub const RT1318_SDCA_RATE_32000HZ: u32 = 0x07;
pub const RT1318_SDCA_RATE_44100HZ: u32 = 0x08;
pub const RT1318_SDCA_RATE_48000HZ: u32 = 0x09;
pub const RT1318_SDCA_RATE_96000HZ: u32 = 0x0b;
pub const RT1318_SDCA_RATE_192000HZ: u32 = 0x0d;

#[repr(C)]
pub struct rt1318_sdw_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sdw_slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
