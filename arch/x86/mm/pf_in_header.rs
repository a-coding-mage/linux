/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 *  Fault Injection Test harness (FI)
 *  Copyright (C) Intel Crop.
 */

#[repr(C)]
pub enum reason_type {
    NOT_ME,      /* page fault is not in regions */
    NOTHING,     /* access others point in regions */
    REG_READ,    /* read from addr to reg */
    REG_WRITE,   /* write from reg to addr */
    IMM_WRITE,   /* write from imm to addr */
    OTHERS,      /* Other instructions can not intercept */
}

/* Declaration supplied by the surrounding system. */
pub struct pt_regs;

extern "C" {
    pub fn get_ins_type(ins_addr: core::ffi::c_ulong) -> reason_type;
    pub fn get_ins_mem_width(ins_addr: core::ffi::c_ulong) -> core::ffi::c_uint;
    pub fn get_ins_reg_val(
        ins_addr: core::ffi::c_ulong,
        regs: *mut pt_regs,
    ) -> core::ffi::c_ulong;
    pub fn get_ins_imm_val(ins_addr: core::ffi::c_ulong) -> core::ffi::c_ulong;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
