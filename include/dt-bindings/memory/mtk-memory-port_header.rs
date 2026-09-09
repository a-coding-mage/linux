/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (c) 2020 MediaTek Inc.
 * Author: Yong Wu <yong.wu@mediatek.com>
 */

pub const MTK_LARB_NR_MAX: u32 = 32;

#[inline]
pub const fn MTK_M4U_ID(larb: u32, port: u32) -> u32 {
    (larb << 5) | port
}

#[inline]
pub const fn MTK_M4U_TO_LARB(id: u32) -> u32 {
    (id >> 5) & 0x1f
}

#[inline]
pub const fn MTK_M4U_TO_PORT(id: u32) -> u32 {
    id & 0x1f
}

#[inline]
pub const fn MTK_IFAIOMMU_PERI_ID(port: u32) -> u32 {
    MTK_M4U_ID(0, port)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
