/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * MXM WMI driver
 *
 * Copyright(C) 2010 Red Hat.
 */

/* discrete adapters */
pub const MXM_MXDS_ADAPTER_0: i32 = 0x0;
pub const MXM_MXDS_ADAPTER_1: i32 = 0x0;
/* integrated adapter */
pub const MXM_MXDS_ADAPTER_IGD: i32 = 0x10;

extern "C" {
    pub fn mxm_wmi_call_mxds(adapter: i32) -> i32;
    pub fn mxm_wmi_call_mxmx(adapter: i32) -> i32;
    pub fn mxm_wmi_supported() -> bool;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
