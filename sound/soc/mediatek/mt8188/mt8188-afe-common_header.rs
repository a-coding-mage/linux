/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt8188-afe-common.h  --  MediaTek 8188 audio driver definitions
 *
 * Copyright (c) 2022 MediaTek Inc.
 * Author: Bicycle Tsai <bicycle.tsai@mediatek.com>
 *         Trevor Wu <trevor.wu@mediatek.com>
 *         Chun-Chia Chiu <chun-chia.chiu@mediatek.com>
 */

/* Dependencies from the original header:
 * <linux/list.h>
 * <linux/regmap.h>
 * <sound/soc.h>
 * "../common/mtk-base-afe.h"
 */

pub const MT8188_DAI_START: u32 = 0;
pub const MT8188_AFE_MEMIF_START: u32 = MT8188_DAI_START;
pub const MT8188_AFE_MEMIF_DL2: u32 = MT8188_AFE_MEMIF_START;
pub const MT8188_AFE_MEMIF_DL3: u32 = 1;
pub const MT8188_AFE_MEMIF_DL6: u32 = 2;
pub const MT8188_AFE_MEMIF_DL7: u32 = 3;
pub const MT8188_AFE_MEMIF_DL8: u32 = 4;
pub const MT8188_AFE_MEMIF_DL10: u32 = 5;
pub const MT8188_AFE_MEMIF_DL11: u32 = 6;
pub const MT8188_AFE_MEMIF_UL_START: u32 = 7;
pub const MT8188_AFE_MEMIF_UL1: u32 = MT8188_AFE_MEMIF_UL_START;
pub const MT8188_AFE_MEMIF_UL2: u32 = 8;
pub const MT8188_AFE_MEMIF_UL3: u32 = 9;
pub const MT8188_AFE_MEMIF_UL4: u32 = 10;
pub const MT8188_AFE_MEMIF_UL5: u32 = 11;
pub const MT8188_AFE_MEMIF_UL6: u32 = 12;
pub const MT8188_AFE_MEMIF_UL8: u32 = 13;
pub const MT8188_AFE_MEMIF_UL9: u32 = 14;
pub const MT8188_AFE_MEMIF_UL10: u32 = 15;
pub const MT8188_AFE_MEMIF_END: u32 = 16;
pub const MT8188_AFE_MEMIF_NUM: u32 = MT8188_AFE_MEMIF_END - MT8188_AFE_MEMIF_START;
pub const MT8188_AFE_IO_START: u32 = MT8188_AFE_MEMIF_END;
pub const MT8188_AFE_IO_DL_SRC: u32 = MT8188_AFE_IO_START;
pub const MT8188_AFE_IO_DMIC_IN: u32 = 17;
pub const MT8188_AFE_IO_DPTX: u32 = 18;
pub const MT8188_AFE_IO_ETDM_START: u32 = 19;
pub const MT8188_AFE_IO_ETDM1_IN: u32 = MT8188_AFE_IO_ETDM_START;
pub const MT8188_AFE_IO_ETDM2_IN: u32 = 20;
pub const MT8188_AFE_IO_ETDM1_OUT: u32 = 21;
pub const MT8188_AFE_IO_ETDM2_OUT: u32 = 22;
pub const MT8188_AFE_IO_ETDM3_OUT: u32 = 23;
pub const MT8188_AFE_IO_ETDM_END: u32 = 24;
pub const MT8188_AFE_IO_ETDM_NUM: u32 = MT8188_AFE_IO_ETDM_END - MT8188_AFE_IO_ETDM_START;
pub const MT8188_AFE_IO_PCM: u32 = MT8188_AFE_IO_ETDM_END;
pub const MT8188_AFE_IO_UL_SRC: u32 = 25;
pub const MT8188_AFE_IO_END: u32 = 26;
pub const MT8188_AFE_IO_NUM: u32 = MT8188_AFE_IO_END - MT8188_AFE_IO_START;
pub const MT8188_DAI_END: u32 = MT8188_AFE_IO_END;
pub const MT8188_DAI_NUM: u32 = MT8188_DAI_END - MT8188_DAI_START;

pub const MT8188_TOP_CG_A1SYS_TIMING: u32 = 0;
pub const MT8188_TOP_CG_A2SYS_TIMING: u32 = 1;
pub const MT8188_TOP_CG_26M_TIMING: u32 = 2;
pub const MT8188_TOP_CG_NUM: u32 = 3;

pub const MT8188_AFE_IRQ_1: u32 = 0;
pub const MT8188_AFE_IRQ_2: u32 = 1;
pub const MT8188_AFE_IRQ_3: u32 = 2;
pub const MT8188_AFE_IRQ_8: u32 = 3;
pub const MT8188_AFE_IRQ_9: u32 = 4;
pub const MT8188_AFE_IRQ_10: u32 = 5;
pub const MT8188_AFE_IRQ_13: u32 = 6;
pub const MT8188_AFE_IRQ_14: u32 = 7;
pub const MT8188_AFE_IRQ_15: u32 = 8;
pub const MT8188_AFE_IRQ_16: u32 = 9;
pub const MT8188_AFE_IRQ_17: u32 = 10;
pub const MT8188_AFE_IRQ_18: u32 = 11;
pub const MT8188_AFE_IRQ_19: u32 = 12;
pub const MT8188_AFE_IRQ_20: u32 = 13;
pub const MT8188_AFE_IRQ_21: u32 = 14;
pub const MT8188_AFE_IRQ_22: u32 = 15;
pub const MT8188_AFE_IRQ_23: u32 = 16;
pub const MT8188_AFE_IRQ_24: u32 = 17;
pub const MT8188_AFE_IRQ_25: u32 = 18;
pub const MT8188_AFE_IRQ_26: u32 = 19;
pub const MT8188_AFE_IRQ_27: u32 = 20;
pub const MT8188_AFE_IRQ_28: u32 = 21;
pub const MT8188_AFE_IRQ_NUM: u32 = 22;

pub const MT8188_ETDM_OUT1_1X_EN: u32 = 9;
pub const MT8188_ETDM_OUT2_1X_EN: u32 = 10;
pub const MT8188_ETDM_OUT3_1X_EN: u32 = 11;
pub const MT8188_ETDM_IN1_1X_EN: u32 = 12;
pub const MT8188_ETDM_IN2_1X_EN: u32 = 13;
pub const MT8188_ETDM_IN1_NX_EN: u32 = 25;
pub const MT8188_ETDM_IN2_NX_EN: u32 = 26;

pub const MT8188_MTKAIF_MISO_0: u32 = 0;
pub const MT8188_MTKAIF_MISO_1: u32 = 1;
pub const MT8188_MTKAIF_MISO_NUM: u32 = 2;

#[repr(C)]
pub struct mtk_dai_memif_irq_priv {
    pub asys_timing_sel: ::core::ffi::c_uint,
}

#[repr(C)]
pub struct mtkaif_param {
    pub mtkaif_calibration_ok: bool,
    pub mtkaif_chosen_phase: [::core::ffi::c_int; MT8188_MTKAIF_MISO_NUM as usize],
    pub mtkaif_phase_cycle: [::core::ffi::c_int; MT8188_MTKAIF_MISO_NUM as usize],
    pub mtkaif_dmic_on: ::core::ffi::c_int,
}

#[repr(C)]
pub struct clk {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct clk_lookup {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct regmap {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct spinlock_t {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mt8188_afe_private {
    pub clk: *mut *mut clk,
    pub lookup: *mut *mut clk_lookup,
    pub topckgen: *mut regmap,
    pub pm_runtime_bypass_reg_ctl: ::core::ffi::c_int,
    pub afe_ctrl_lock: spinlock_t, /* Lock for afe control */
    pub irq_priv: [mtk_dai_memif_irq_priv; MT8188_AFE_IRQ_NUM as usize],
    pub mtkaif_params: mtkaif_param,

    /* dai */
    pub dai_priv: [*mut ::core::ffi::c_void; MT8188_DAI_NUM as usize],
}

unsafe extern "C" {
    pub fn mt8188_afe_fs_timing(rate: ::core::ffi::c_uint) -> ::core::ffi::c_int;
    /* dai register */
    pub fn mt8188_dai_adda_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8188_dai_dmic_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8188_dai_etdm_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
    pub fn mt8188_dai_pcm_register(afe: *mut mtk_base_afe) -> ::core::ffi::c_int;
}

macro_rules! MT8188_SOC_ENUM_EXT {
    ($xname:expr, $xenum:expr, $xhandler_get:expr, $xhandler_put:expr, $id:expr) => {
        {
            iface: SNDRV_CTL_ELEM_IFACE_MIXER,
            name: $xname,
            info: snd_soc_info_enum_double,
            get: $xhandler_get,
            put: $xhandler_put,
            device: $id,
            private_value: &$xenum as *const _ as ::core::ffi::c_ulong,
        }
    };
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
