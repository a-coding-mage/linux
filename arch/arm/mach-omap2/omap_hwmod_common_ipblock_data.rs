// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap_hwmod_common_ipblock_data.c - common IP block data for OMAP2+
 *
 * Copyright (C) 2011 Nokia Corporation
 * Copyright (C) 2012 Texas Instruments, Inc.
 * Paul Walmsley
 */

// Dependencies supplied by the corresponding OMAP hwmod headers.

extern "C" {
    static mut omap_hwmod_sysc_type1: omap_hwmod_sysc_fields;
    fn omap_dss_reset(hwmod: *mut omap_hwmod) -> i32;
}

/*
 * 'dss' class
 * display sub-system
 */

static mut omap2_dss_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x0000,
    sysc_offs: 0x0010,
    syss_offs: 0x0014,
    sysc_flags: SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE | SYSS_HAS_RESET_STATUS,
    sysc_fields: unsafe { &mut omap_hwmod_sysc_type1 },
};

static mut omap2_dss_hwmod_class: omap_hwmod_class = omap_hwmod_class {
    name: "dss",
    sysc: unsafe { &mut omap2_dss_sysc },
    reset: Some(omap_dss_reset),
};

/*
 * 'rfbi' class
 * remote frame buffer interface
 */

static mut omap2_rfbi_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x0000,
    sysc_offs: 0x0010,
    syss_offs: 0x0014,
    sysc_flags: SYSC_HAS_SIDLEMODE | SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE,
    idlemodes: SIDLE_FORCE | SIDLE_NO | SIDLE_SMART,
    sysc_fields: unsafe { &mut omap_hwmod_sysc_type1 },
};

static mut omap2_rfbi_hwmod_class: omap_hwmod_class = omap_hwmod_class {
    name: "rfbi",
    sysc: unsafe { &mut omap2_rfbi_sysc },
    ..unsafe { core::mem::zeroed() }
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
