/* SPDX-License-Identifier: GPL-2.0
 *
 * Fifo-attached Serial Interface (FSI) support for SH7724
 *
 * Copyright (C) 2009 Renesas Solutions Corp.
 * Kuninori Morimoto <morimoto.kuninori@renesas.com>
 */

// C dependencies:
// #include <linux/clk.h>
// #include <sound/soc.h>

/*
 * flags
 */
pub const SH_FSI_FMT_SPDIF: u32 = 1 << 0; // spdif for HDMI
pub const SH_FSI_ENABLE_STREAM_MODE: u32 = 1 << 1; // for 16bit data
pub const SH_FSI_CLK_CPG: u32 = 1 << 2; // FSIxCK + FSI-DIV

#[repr(C)]
pub struct sh_fsi_port_info {
    pub flags: core::ffi::c_ulong,
    pub tx_id: core::ffi::c_int,
    pub rx_id: core::ffi::c_int,
}

#[repr(C)]
pub struct sh_fsi_platform_info {
    pub port_a: sh_fsi_port_info,
    pub port_b: sh_fsi_port_info,
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
