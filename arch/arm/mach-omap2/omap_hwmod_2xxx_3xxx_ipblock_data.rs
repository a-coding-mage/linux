// SPDX-License-Identifier: GPL-2.0-only
/*
 * omap_hwmod_2xxx_3xxx_ipblock_data.c - common IP block data for OMAP2/3
 *
 * Copyright (C) 2011 Nokia Corporation
 * Copyright (C) 2012 Texas Instruments, Inc.
 * Paul Walmsley
 */

// Dependencies supplied by the corresponding OMAP hwmod and DMA bindings.

/* UART */

static mut omap2_uart_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x50,
    sysc_offs: 0x54,
    syss_offs: 0x58,
    sysc_flags: SYSC_HAS_SIDLEMODE
        | SYSC_HAS_ENAWAKEUP
        | SYSC_HAS_SOFTRESET
        | SYSC_HAS_AUTOIDLE
        | SYSS_HAS_RESET_STATUS,
    idlemodes: SIDLE_FORCE | SIDLE_NO | SIDLE_SMART,
    sysc_fields: core::ptr::addr_of_mut!(omap_hwmod_sysc_type1),
};

pub static mut omap2_uart_class: omap_hwmod_class = omap_hwmod_class {
    name: "uart",
    sysc: core::ptr::addr_of_mut!(omap2_uart_sysc),
};

/*
 * 'venc' class
 * video encoder
 */

pub static mut omap2_venc_hwmod_class: omap_hwmod_class = omap_hwmod_class {
    name: "venc",
};

/*
 * omap_hwmod class data
 */

pub static mut l3_hwmod_class: omap_hwmod_class = omap_hwmod_class {
    name: "l3",
};

pub static mut l4_hwmod_class: omap_hwmod_class = omap_hwmod_class {
    name: "l4",
};

pub static mut mpu_hwmod_class: omap_hwmod_class = omap_hwmod_class {
    name: "mpu",
};

pub static mut iva_hwmod_class: omap_hwmod_class = omap_hwmod_class {
    name: "iva",
};

static mut omap2_hdq1w_sysc: omap_hwmod_class_sysconfig = omap_hwmod_class_sysconfig {
    rev_offs: 0x0,
    sysc_offs: 0x14,
    syss_offs: 0x18,
    sysc_flags: SYSC_HAS_SOFTRESET | SYSC_HAS_AUTOIDLE | SYSS_HAS_RESET_STATUS,
    sysc_fields: core::ptr::addr_of_mut!(omap_hwmod_sysc_type1),
};

pub static mut omap2_hdq1w_class: omap_hwmod_class = omap_hwmod_class {
    name: "hdq1w",
    sysc: core::ptr::addr_of_mut!(omap2_hdq1w_sysc),
    reset: core::ptr::addr_of_mut!(omap_hdq1w_reset),
};

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
