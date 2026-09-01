// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt1316-sdw.h -- RT1316 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2021 Realtek Semiconductor Corp.
 */

// C header dependencies:
// #include <linux/regmap.h>
// #include <linux/soundwire/sdw.h>
// #include <linux/soundwire/sdw_type.h>
// #include <linux/soundwire/sdw_registers.h>
// #include <sound/soc.h>

/* RT1316 SDCA Control - function number */
pub const FUNC_NUM_SMART_AMP: u32 = 0x04;

/* RT1316 SDCA entity */
pub const RT1316_SDCA_ENT_PDE23: u32 = 0x31;
pub const RT1316_SDCA_ENT_PDE27: u32 = 0x32;
pub const RT1316_SDCA_ENT_PDE22: u32 = 0x33;
pub const RT1316_SDCA_ENT_PDE24: u32 = 0x34;
pub const RT1316_SDCA_ENT_XU24: u32 = 0x24;
pub const RT1316_SDCA_ENT_FU21: u32 = 0x03;
pub const RT1316_SDCA_ENT_UDMPU21: u32 = 0x02;

/* RT1316 SDCA control */
pub const RT1316_SDCA_CTL_SAMPLE_FREQ_INDEX: u32 = 0x10;
pub const RT1316_SDCA_CTL_REQ_POWER_STATE: u32 = 0x01;
pub const RT1316_SDCA_CTL_BYPASS: u32 = 0x01;
pub const RT1316_SDCA_CTL_FU_MUTE: u32 = 0x01;
pub const RT1316_SDCA_CTL_FU_VOLUME: u32 = 0x02;
pub const RT1316_SDCA_CTL_UDMPU_CLUSTER: u32 = 0x10;

/* RT1316 SDCA channel */
pub const CH_L: u32 = 0x01;
pub const CH_R: u32 = 0x02;

#[repr(C)]
pub struct rt1316_sdw_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sdw_slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
    pub bq_params: *mut ::core::ffi::c_uchar,
    pub bq_params_cnt: ::core::ffi::c_uint,
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
