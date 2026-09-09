/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2013 Imagination Technologies
 * Author: Paul Burton <paul.burton@mips.com>
 */

// C dependency: asm/mipsregs.h and asm/inst.h.

extern "C" {
    pub fn _save_msa(t: *mut task_struct);
    pub fn _restore_msa(t: *mut task_struct);
    pub fn _init_msa_upper();

    pub fn read_msa_wr_b(idx: u32, to: *mut fpureg);
    pub fn read_msa_wr_h(idx: u32, to: *mut fpureg);
    pub fn read_msa_wr_w(idx: u32, to: *mut fpureg);
    pub fn read_msa_wr_d(idx: u32, to: *mut fpureg);

    pub fn write_msa_wr_b(idx: u32, from: *mut fpureg);
    pub fn write_msa_wr_h(idx: u32, from: *mut fpureg);
    pub fn write_msa_wr_w(idx: u32, from: *mut fpureg);
    pub fn write_msa_wr_d(idx: u32, from: *mut fpureg);
}

// Supplied by the kernel and by asm/inst.h.
pub enum task_struct {}
#[repr(C)]
pub union fpureg {
    _opaque: [u8; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub enum msa_2b_fmt {
    msa_fmt_b,
    msa_fmt_h,
    msa_fmt_w,
    msa_fmt_d,
}

#[inline]
pub unsafe fn read_msa_wr(idx: u32, to: *mut fpureg, fmt: msa_2b_fmt) {
    match fmt {
        msa_2b_fmt::msa_fmt_b => read_msa_wr_b(idx, to),
        msa_2b_fmt::msa_fmt_h => read_msa_wr_h(idx, to),
        msa_2b_fmt::msa_fmt_w => read_msa_wr_w(idx, to),
        msa_2b_fmt::msa_fmt_d => read_msa_wr_d(idx, to),
    }
}

#[inline]
pub unsafe fn write_msa_wr(idx: u32, from: *mut fpureg, fmt: msa_2b_fmt) {
    match fmt {
        msa_2b_fmt::msa_fmt_b => write_msa_wr_b(idx, from),
        msa_2b_fmt::msa_fmt_h => write_msa_wr_h(idx, from),
        msa_2b_fmt::msa_fmt_w => write_msa_wr_w(idx, from),
        msa_2b_fmt::msa_fmt_d => write_msa_wr_d(idx, from),
    }
}

// The following kernel symbols and operations are supplied by asm/mipsregs.h.
extern "C" {
    pub static mut cpu_has_msa: bool;
    pub fn set_c0_config5(value: u32);
    pub fn clear_c0_config5(value: u32);
    pub fn enable_fpu_hazard();
    pub fn disable_fpu_hazard();
    pub fn read_c0_config5() -> u32;
    pub fn test_thread_flag(flag: u32) -> i32;
}

// Values supplied by asm/mipsregs.h.
pub const MIPS_CONF5_MSAEN: u32 = 1 << 1;
pub const TIF_MSA_CTX_LIVE: u32 = 0;

#[inline]
pub unsafe fn enable_msa() {
    if cpu_has_msa {
        set_c0_config5(MIPS_CONF5_MSAEN);
        enable_fpu_hazard();
    }
}

#[inline]
pub unsafe fn disable_msa() {
    if cpu_has_msa {
        clear_c0_config5(MIPS_CONF5_MSAEN);
        disable_fpu_hazard();
    }
}

#[inline]
pub unsafe fn is_msa_enabled() -> i32 {
    if !cpu_has_msa { return 0; }
    (read_c0_config5() & MIPS_CONF5_MSAEN) as i32
}

#[inline]
pub unsafe fn thread_msa_context_live() -> i32 {
    if !cpu_has_msa { return 0; }
    test_thread_flag(TIF_MSA_CTX_LIVE)
}

#[inline]
pub unsafe fn save_msa(t: *mut task_struct) {
    if cpu_has_msa { _save_msa(t); }
}

#[inline]
pub unsafe fn restore_msa(t: *mut task_struct) {
    if cpu_has_msa { _restore_msa(t); }
}

#[inline]
pub unsafe fn init_msa_upper() {
    if cpu_has_msa { _init_msa_upper(); }
}

pub const MSA_IR: u32 = 0;
pub const MSA_CSR: u32 = 1;
pub const MSA_ACCESS: u32 = 2;
pub const MSA_SAVE: u32 = 3;
pub const MSA_MODIFY: u32 = 4;
pub const MSA_REQUEST: u32 = 5;
pub const MSA_MAP: u32 = 6;
pub const MSA_UNMAP: u32 = 7;

pub const MSA_IR_REVB: u32 = 0;
pub const MSA_IR_REVF: u32 = 0xff << MSA_IR_REVB;
pub const MSA_IR_PROCB: u32 = 8;
pub const MSA_IR_PROCF: u32 = 0xff << MSA_IR_PROCB;
pub const MSA_IR_WRPB: u32 = 16;
pub const MSA_IR_WRPF: u32 = 1 << MSA_IR_WRPB;

pub const MSA_CSR_RMB: u32 = 0;
pub const MSA_CSR_RMF: u32 = 3 << MSA_CSR_RMB;
pub const MSA_CSR_RM_NEAREST: u32 = 0;
pub const MSA_CSR_RM_TO_ZERO: u32 = 1;
pub const MSA_CSR_RM_TO_POS: u32 = 2;
pub const MSA_CSR_RM_TO_NEG: u32 = 3;
pub const MSA_CSR_FLAGSB: u32 = 2;
pub const MSA_CSR_FLAGSF: u32 = 0x1f << MSA_CSR_FLAGSB;
pub const MSA_CSR_FLAGS_IB: u32 = 2;
pub const MSA_CSR_FLAGS_IF: u32 = 1 << MSA_CSR_FLAGS_IB;
pub const MSA_CSR_FLAGS_UB: u32 = 3;
pub const MSA_CSR_FLAGS_UF: u32 = 1 << MSA_CSR_FLAGS_UB;
pub const MSA_CSR_FLAGS_OB: u32 = 4;
pub const MSA_CSR_FLAGS_OF: u32 = 1 << MSA_CSR_FLAGS_OB;
pub const MSA_CSR_FLAGS_ZB: u32 = 5;
pub const MSA_CSR_FLAGS_ZF: u32 = 1 << MSA_CSR_FLAGS_ZB;
pub const MSA_CSR_FLAGS_VB: u32 = 6;
pub const MSA_CSR_FLAGS_VF: u32 = 1 << MSA_CSR_FLAGS_VB;
pub const MSA_CSR_ENABLESB: u32 = 7;
pub const MSA_CSR_ENABLESF: u32 = 0x1f << MSA_CSR_ENABLESB;
pub const MSA_CSR_ENABLES_IB: u32 = 7;
pub const MSA_CSR_ENABLES_IF: u32 = 1 << MSA_CSR_ENABLES_IB;
pub const MSA_CSR_ENABLES_UB: u32 = 8;
pub const MSA_CSR_ENABLES_UF: u32 = 1 << MSA_CSR_ENABLES_UB;
pub const MSA_CSR_ENABLES_OB: u32 = 9;
pub const MSA_CSR_ENABLES_OF: u32 = 1 << MSA_CSR_ENABLES_OB;
pub const MSA_CSR_ENABLES_ZB: u32 = 10;
pub const MSA_CSR_ENABLES_ZF: u32 = 1 << MSA_CSR_ENABLES_ZB;
pub const MSA_CSR_ENABLES_VB: u32 = 11;
pub const MSA_CSR_ENABLES_VF: u32 = 1 << MSA_CSR_ENABLES_VB;
pub const MSA_CSR_CAUSEB: u32 = 12;
pub const MSA_CSR_CAUSEF: u32 = 0x3f << MSA_CSR_CAUSEB;
pub const MSA_CSR_CAUSE_IB: u32 = 12;
pub const MSA_CSR_CAUSE_IF: u32 = 1 << MSA_CSR_CAUSE_IB;
pub const MSA_CSR_CAUSE_UB: u32 = 13;
pub const MSA_CSR_CAUSE_UF: u32 = 1 << MSA_CSR_CAUSE_UB;
pub const MSA_CSR_CAUSE_OB: u32 = 14;
pub const MSA_CSR_CAUSE_OF: u32 = 1 << MSA_CSR_CAUSE_OB;
pub const MSA_CSR_CAUSE_ZB: u32 = 15;
pub const MSA_CSR_CAUSE_ZF: u32 = 1 << MSA_CSR_CAUSE_ZB;
pub const MSA_CSR_CAUSE_VB: u32 = 16;
pub const MSA_CSR_CAUSE_VF: u32 = 1 << MSA_CSR_CAUSE_VB;
pub const MSA_CSR_CAUSE_EB: u32 = 17;
pub const MSA_CSR_CAUSE_EF: u32 = 1 << MSA_CSR_CAUSE_EB;
pub const MSA_CSR_NXB: u32 = 18;
pub const MSA_CSR_NXF: u32 = 1 << MSA_CSR_NXB;
pub const MSA_CSR_FSB: u32 = 24;
pub const MSA_CSR_FSF: u32 = 1 << MSA_CSR_FSB;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
