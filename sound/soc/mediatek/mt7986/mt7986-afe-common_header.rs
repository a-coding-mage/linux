/* SPDX-License-Identifier: GPL-2.0 */
/*
 * mt7986-afe-common.h  --  MediaTek 7986 audio driver definitions
 *
 * Copyright (c) 2023 MediaTek Inc.
 * Authors: Vic Wu <vic.wu@mediatek.com>
 *          Maso Huang <maso.huang@mediatek.com>
 */

/*
 * C dependencies:
 * #include <sound/soc.h>
 * #include <linux/clk.h>
 * #include <linux/list.h>
 * #include <linux/regmap.h>
 * #include "../common/mtk-base-afe.h"
 */

use core::ffi::{c_int, c_uint, c_void};

#[repr(C)]
pub struct device {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct clk_bulk_data {
    _unused: [u8; 0],
}

#[repr(C)]
pub struct mtk_base_afe {
    _unused: [u8; 0],
}

pub const MT7986_MEMIF_DL1: c_int = 0;
pub const MT7986_MEMIF_VUL12: c_int = 1;
pub const MT7986_MEMIF_NUM: c_int = 2;
pub const MT7986_DAI_ETDM: c_int = MT7986_MEMIF_NUM;
pub const MT7986_DAI_NUM: c_int = 3;

pub const MT7986_IRQ_0: c_int = 0;
pub const MT7986_IRQ_1: c_int = 1;
pub const MT7986_IRQ_2: c_int = 2;
pub const MT7986_IRQ_NUM: c_int = 3;

#[repr(C)]
pub struct mt7986_afe_private {
    pub clks: *mut clk_bulk_data,
    pub num_clks: c_int,

    pub pm_runtime_bypass_reg_ctl: c_int,

    /* dai */
    pub dai_priv: [*mut c_void; MT7986_DAI_NUM as usize],
}

unsafe extern "C" {
    pub fn mt7986_afe_rate_transform(dev: *mut device, rate: c_uint) -> c_uint;

    /* dai register */
    pub fn mt7986_dai_etdm_register(afe: *mut mtk_base_afe) -> c_int;
}

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
