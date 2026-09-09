/* SPDX-License-Identifier: GPL-2.0 */
/*
 * File ported from IRIX to Linux by Kanoj Sarcar, 06/08/00.
 * Copyright 2000 Silicon Graphics, Inc.
 */

// C header guard: __ASM_SN_KLKERNVARS_H

pub const KV_MAGIC_OFFSET: u32 = 0x0;
pub const KV_RO_NASID_OFFSET: u32 = 0x4;
pub const KV_RW_NASID_OFFSET: u32 = 0x6;

pub const KV_MAGIC: u32 = 0x5f4b565f;

// C dependency: #include <asm/sn/types.h>

#[repr(C)]
pub struct kern_vars_s {
    pub kv_magic: i32,
    pub kv_ro_nasid: nasid_t,
    pub kv_rw_nasid: nasid_t,
    pub kv_ro_baseaddr: usize,
    pub kv_rw_baseaddr: usize,
}

pub type kern_vars_t = kern_vars_s;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
