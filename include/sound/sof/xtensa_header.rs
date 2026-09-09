/* SPDX-License-Identifier: (GPL-2.0-only OR BSD-3-Clause) */
/*
 * This file is provided under a dual BSD/GPLv2 license.  When using or
 * redistributing this file, you may do so under either license.
 *
 * Copyright(c) 2018 Intel Corporation
 */

// Dependency supplied by sound/sof/header.h.

/*
 * Architecture specific debug
 */

/* Xtensa Firmware Oops data */
#[repr(C, packed)]
pub struct sof_ipc_dsp_oops_xtensa {
    pub arch_hdr: sof_ipc_dsp_oops_arch_hdr,
    pub plat_hdr: sof_ipc_dsp_oops_plat_hdr,
    pub exccause: u32,
    pub excvaddr: u32,
    pub ps: u32,
    pub epc1: u32,
    pub epc2: u32,
    pub epc3: u32,
    pub epc4: u32,
    pub epc5: u32,
    pub epc6: u32,
    pub epc7: u32,
    pub eps2: u32,
    pub eps3: u32,
    pub eps4: u32,
    pub eps5: u32,
    pub eps6: u32,
    pub eps7: u32,
    pub depc: u32,
    pub intenable: u32,
    pub interrupt: u32,
    pub sar: u32,
    pub debugcause: u32,
    pub windowbase: u32,
    pub windowstart: u32,
    pub excsave1: u32,
    pub ar: [u32; 0],
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
