// SPDX-License-Identifier: GPL-2.0-only
/*
 * rt1017-sdca-sdw.h -- RT1017 SDCA ALSA SoC audio driver header
 *
 * Copyright(c) 2023 Realtek Semiconductor Corp.
 */

// C header dependencies:
// #include <linux/regmap.h>
// #include <linux/soundwire/sdw.h>
// #include <linux/soundwire/sdw_type.h>
// #include <linux/soundwire/sdw_registers.h>
// #include <sound/soc.h>

use core::ffi::c_uint;

/* RT1017 SDCA Control - function number */
pub const FUNC_NUM_SMART_AMP: c_uint = 0x04;

/* RT1017 SDCA entity */
pub const RT1017_SDCA_ENT_PDE23: c_uint = 0x31;
pub const RT1017_SDCA_ENT_PDE22: c_uint = 0x33;
pub const RT1017_SDCA_ENT_CS21: c_uint = 0x21;
pub const RT1017_SDCA_ENT_SAPU29: c_uint = 0x29;
pub const RT1017_SDCA_ENT_XU22: c_uint = 0x22;
pub const RT1017_SDCA_ENT_FU: c_uint = 0x03;
pub const RT1017_SDCA_ENT_UDMPU21: c_uint = 0x02;

/* RT1017 SDCA control */
pub const RT1017_SDCA_CTL_FS_INDEX: c_uint = 0x10;
pub const RT1017_SDCA_CTL_REQ_POWER_STATE: c_uint = 0x01;
pub const RT1017_SDCA_CTL_PROT_STAT: c_uint = 0x11;
pub const RT1017_SDCA_CTL_BYPASS: c_uint = 0x01;
pub const RT1017_SDCA_CTL_FU_MUTE: c_uint = 0x01;
pub const RT1017_SDCA_CTL_FU_VOLUME: c_uint = 0x02;
pub const RT1017_SDCA_CTL_UDMPU_CLUSTER: c_uint = 0x10;

pub const RT1017_CLASSD_INT_1: c_uint = 0xd300;
pub const RT1017_PWM_TRIM_1: c_uint = 0xd370;

pub const RT1017_PWM_FREQ_CTL_SRC_SEL_MASK: c_uint = 0x3 << 2;
pub const RT1017_PWM_FREQ_CTL_SRC_SEL_EFUSE: c_uint = 0x2 << 2;
pub const RT1017_PWM_FREQ_CTL_SRC_SEL_REG: c_uint = 0x0 << 2;

pub const RT1017_SDCA_RATE_44100HZ: c_uint = 0x8;
pub const RT1017_SDCA_RATE_48000HZ: c_uint = 0x9;
pub const RT1017_SDCA_RATE_96000HZ: c_uint = 0xb;
pub const RT1017_SDCA_RATE_192000HZ: c_uint = 0xd;

#[repr(C)]
pub struct snd_soc_component {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct sdw_slave {
    _unused: [u8; 0],
}

// Definition is supplied by <linux/soundwire/sdw.h> in the original C source.
pub type sdw_bus_params = crate::sdw_bus_params;

#[repr(C)]
pub struct rt1017_sdca_priv {
    pub component: *mut snd_soc_component,
    pub regmap: *mut regmap,
    pub sdw_slave: *mut sdw_slave,
    pub params: sdw_bus_params,
    pub hw_init: bool,
    pub first_hw_init: bool,
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct reg_default {
    pub reg: c_uint,
    pub def: c_uint,
}

pub static rt1017_sdca_reg_defaults: &[reg_default] = &[
    reg_default { reg: 0x3206, def: 0x00 },
    reg_default { reg: 0xc001, def: 0x43 },
    reg_default { reg: 0xc030, def: 0x54 },
    reg_default { reg: 0xc104, def: 0x8a },
    reg_default { reg: 0xc10b, def: 0x2f },
    reg_default { reg: 0xc10c, def: 0x2f },
    reg_default { reg: 0xc110, def: 0x49 },
    reg_default { reg: 0xc112, def: 0x10 },
    reg_default { reg: 0xc300, def: 0xff },
    reg_default { reg: 0xc301, def: 0xdd },
    reg_default { reg: 0xc318, def: 0x40 },
    reg_default { reg: 0xc325, def: 0x00 },
    reg_default { reg: 0xc326, def: 0x00 },
    reg_default { reg: 0xc327, def: 0x00 },
    reg_default { reg: 0xc328, def: 0x02 },
    reg_default { reg: 0xc331, def: 0xb2 },
    reg_default { reg: 0xc340, def: 0x02 },
    reg_default { reg: 0xc350, def: 0x21 },
    reg_default { reg: 0xc500, def: 0x00 },
    reg_default { reg: 0xc502, def: 0x00 },
    reg_default { reg: 0xc504, def: 0x3f },
    reg_default { reg: 0xc507, def: 0x1f },
    reg_default { reg: 0xc509, def: 0x1f },
    reg_default { reg: 0xc510, def: 0x40 },
    reg_default { reg: 0xc512, def: 0x00 },
    reg_default { reg: 0xc518, def: 0x02 },
    reg_default { reg: 0xc51b, def: 0x7f },
    reg_default { reg: 0xc51d, def: 0x0f },
    reg_default { reg: 0xc520, def: 0x00 },
    reg_default { reg: 0xc540, def: 0x80 },
    reg_default { reg: 0xc541, def: 0x00 },
    reg_default { reg: 0xc542, def: 0x0a },
    reg_default { reg: 0xc550, def: 0x80 },
    reg_default { reg: 0xc551, def: 0x0f },
    reg_default { reg: 0xc552, def: 0xff },
    reg_default { reg: 0xc600, def: 0x10 },
    reg_default { reg: 0xc602, def: 0x83 },
    reg_default { reg: 0xc612, def: 0x40 },
    reg_default { reg: 0xc622, def: 0x40 },
    reg_default { reg: 0xc632, def: 0x40 },
    reg_default { reg: 0xc642, def: 0x40 },
    reg_default { reg: 0xc651, def: 0x00 },
    reg_default { reg: 0xca00, def: 0xc1 },
    reg_default { reg: 0xca09, def: 0x00 },
    reg_default { reg: 0xca0a, def: 0x51 },
    reg_default { reg: 0xca0b, def: 0xeb },
    reg_default { reg: 0xca0c, def: 0x85 },
    reg_default { reg: 0xca0e, def: 0x00 },
    reg_default { reg: 0xca0f, def: 0x10 },
    reg_default { reg: 0xca10, def: 0x62 },
    reg_default { reg: 0xca11, def: 0x4d },
    reg_default { reg: 0xca16, def: 0x0f },
    reg_default { reg: 0xca17, def: 0x00 },
    reg_default { reg: 0xcb00, def: 0x10 },
    reg_default { reg: 0xcc00, def: 0x10 },
    reg_default { reg: 0xcc02, def: 0x0b },
    reg_default { reg: 0xd017, def: 0x09 },
    reg_default { reg: 0xd01a, def: 0x00 },
    reg_default { reg: 0xd01b, def: 0x00 },
    reg_default { reg: 0xd01c, def: 0x00 },
    reg_default { reg: 0xd101, def: 0xa0 },
    reg_default { reg: 0xd20c, def: 0x14 },
    reg_default { reg: 0xd300, def: 0x0f },
    reg_default { reg: 0xd370, def: 0x18 },
    reg_default { reg: 0xd500, def: 0x00 },
    reg_default { reg: 0xd545, def: 0x0b },
    reg_default { reg: 0xd546, def: 0xf9 },
    reg_default { reg: 0xd547, def: 0xb2 },
    reg_default { reg: 0xd548, def: 0xa9 },
    reg_default { reg: 0xd5a5, def: 0x00 },
    reg_default { reg: 0xd5a6, def: 0x00 },
    reg_default { reg: 0xd5a7, def: 0x00 },
    reg_default { reg: 0xd5a8, def: 0x00 },
    reg_default { reg: 0xd5aa, def: 0x00 },
    reg_default { reg: 0xd5ab, def: 0x00 },
    reg_default { reg: 0xd5ac, def: 0x00 },
    reg_default { reg: 0xd5ad, def: 0x00 },
    reg_default { reg: 0xda04, def: 0x03 },
    reg_default { reg: 0xda05, def: 0x33 },
    reg_default { reg: 0xda06, def: 0x33 },
    reg_default { reg: 0xda07, def: 0x33 },
    reg_default { reg: 0xda09, def: 0x5d },
    reg_default { reg: 0xda0a, def: 0xc0 },
    reg_default { reg: 0xda0c, def: 0x00 },
    reg_default { reg: 0xda0d, def: 0x01 },
    reg_default { reg: 0xda0e, def: 0x5d },
    reg_default { reg: 0xda0f, def: 0x86 },
    reg_default { reg: 0xda11, def: 0x20 },
    reg_default { reg: 0xda12, def: 0x00 },
    reg_default { reg: 0xda13, def: 0x00 },
    reg_default { reg: 0xda14, def: 0x00 },
    reg_default { reg: 0xda16, def: 0x7f },
    reg_default { reg: 0xda17, def: 0xff },
    reg_default { reg: 0xda18, def: 0xff },
    reg_default { reg: 0xda19, def: 0xff },
    reg_default { reg: 0xdab6, def: 0x00 },
    reg_default { reg: 0xdab7, def: 0x01 },
    reg_default { reg: 0xdab8, def: 0x00 },
    reg_default { reg: 0xdab9, def: 0x01 },
    reg_default { reg: 0xdaba, def: 0x00 },
    reg_default { reg: 0xdabb, def: 0x01 },
    reg_default { reg: 0xdb09, def: 0x0f },
    reg_default { reg: 0xdb0a, def: 0xff },
    reg_default { reg: 0xdb14, def: 0x00 },
    reg_default {
        reg: SDW_SDCA_CTL!(
            FUNC_NUM_SMART_AMP,
            RT1017_SDCA_ENT_FU,
            RT1017_SDCA_CTL_FU_MUTE,
            0x01
        ),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(
            FUNC_NUM_SMART_AMP,
            RT1017_SDCA_ENT_XU22,
            RT1017_SDCA_CTL_BYPASS,
            0
        ),
        def: 0x01,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(
            FUNC_NUM_SMART_AMP,
            RT1017_SDCA_ENT_PDE23,
            RT1017_SDCA_CTL_REQ_POWER_STATE,
            0
        ),
        def: 0x03,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(
            FUNC_NUM_SMART_AMP,
            RT1017_SDCA_ENT_PDE22,
            RT1017_SDCA_CTL_REQ_POWER_STATE,
            0
        ),
        def: 0x03,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(
            FUNC_NUM_SMART_AMP,
            RT1017_SDCA_ENT_UDMPU21,
            RT1017_SDCA_CTL_UDMPU_CLUSTER,
            0
        ),
        def: 0x00,
    },
    reg_default {
        reg: SDW_SDCA_CTL!(
            FUNC_NUM_SMART_AMP,
            RT1017_SDCA_ENT_CS21,
            RT1017_SDCA_CTL_FS_INDEX,
            0
        ),
        def: 0x09,
    },
];

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
