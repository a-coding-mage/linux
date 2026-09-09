/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Header for AK8813 / AK8814 TV-ecoders from Asahi Kasei Microsystems Co., Ltd. (AKM)
 *
 * Copyright (C) 2010, Guennadi Liakhovetski <g.liakhovetski@gmx.de>
 */

pub const AK881X_IF_MODE_MASK: u32 = 3 << 0;
pub const AK881X_IF_MODE_BT656: u32 = 0 << 0;
pub const AK881X_IF_MODE_MASTER: u32 = 1 << 0;
pub const AK881X_IF_MODE_SLAVE: u32 = 2 << 0;
pub const AK881X_FIELD: u32 = 1 << 2;
pub const AK881X_COMPONENT: u32 = 1 << 3;

#[repr(C)]
pub struct ak881x_pdata {
    pub flags: core::ffi::c_ulong,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
