// SPDX-License-Identifier: GPL-2.0
// Copyright (C) 2018 Hangzhou C-SKY Microsystems co.,ltd.

// The declarations below are supplied by the kernel and ABI headers.
use crate::{pt_regs, user_fp};

const MTCR_MASK: usize = 0xFC00FFE0;
const MFCR_MASK: usize = 0xFC00FFE0;
const MTCR_DIST: usize = 0xC0006420;
const MFCR_DIST: usize = 0xC0006020;

extern "C" {
    fn instruction_pointer(regs: *mut pt_regs) -> usize;
    fn __get_user(dst: *mut u16, src: *const u16) -> i32;
    fn mtcr(name: *const u8, value: usize);
    fn mfcr(name: *const u8) -> usize;
    fn local_irq_save(flags: *mut usize);
    fn local_irq_restore(flags: usize);
    fn force_sig_fault(sig: i32, code: i32, addr: *mut core::ffi::c_void);
}

// fpu_libc_helper() is to help libc to excute:
//  - mfcr %a, cr<1, 2>
//  - mfcr %a, cr<2, 2>
//  - mtcr %a, cr<1, 2>
//  - mtcr %a, cr<2, 2>
pub unsafe fn fpu_libc_helper(regs: *mut pt_regs) -> i32 {
    let mut fault: i32;
    let instrptr = instruction_pointer(regs);
    let mut regx: usize = 0;
    let mut index: usize = 0;
    let mut tmp: usize = 0;
    let mut tinstr: usize;
    let mut instr_hi: u16 = 0;
    let mut instr_low: u16 = 0;

    if instrptr & 1 != 0 { return 0; }
    fault = __get_user(&mut instr_low, instrptr as *const u16);
    if fault != 0 { return 0; }
    fault = __get_user(&mut instr_hi, (instrptr + 2) as *const u16);
    if fault != 0 { return 0; }
    tinstr = instr_hi as usize | ((instr_low as usize) << 16);

    if ((tinstr >> 21) & 0x1f) != 2 { return 0; }

    if (tinstr & MTCR_MASK) == MTCR_DIST {
        index = (tinstr >> 16) & 0x1f;
        if index > 13 { return 0; }
        tmp = tinstr & 0x1f;
        if tmp > 2 { return 0; }
        // `a0` through `a13` are consecutive fields in struct pt_regs.
        regx = *(&(*regs).a0 as *const _ .add(index));
        if tmp == 1 {
            mtcr(b"cr<1, 2>\0".as_ptr(), regx);
        } else if tmp == 2 {
            mtcr(b"cr<2, 2>\0".as_ptr(), regx);
        } else { return 0; }
        (*regs).pc += 4;
        return 1;
    }

    if (tinstr & MFCR_MASK) == MFCR_DIST {
        index = tinstr & 0x1f;
        if index > 13 { return 0; }
        tmp = (tinstr >> 16) & 0x1f;
        if tmp > 2 { return 0; }
        if tmp == 1 {
            regx = mfcr(b"cr<1, 2>\0".as_ptr());
        } else if tmp == 2 {
            regx = mfcr(b"cr<2, 2>\0".as_ptr());
        } else { return 0; }
        *(&mut (*regs).a0 as *mut _ .add(index)) = regx;
        (*regs).pc += 4;
        return 1;
    }
    0
}

pub unsafe fn fpu_fpe(regs: *mut pt_regs) {
    let fesr = mfcr(b"cr<2, 2>\0".as_ptr()) as u32;
    let mut sig = SIGFPE;
    let mut code = FPE_FLTUNK;
    if fesr & FPE_ILLE != 0 {
        sig = SIGILL; code = ILL_ILLOPC;
    } else if fesr & FPE_IDC != 0 {
        sig = SIGILL; code = ILL_ILLOPN;
    } else if fesr & FPE_FEC != 0 {
        if fesr & FPE_IOC != 0 { code = FPE_FLTINV; }
        else if fesr & FPE_DZC != 0 { code = FPE_FLTDIV; }
        else if fesr & FPE_UFC != 0 { code = FPE_FLTUND; }
        else if fesr & FPE_OFC != 0 { code = FPE_FLTOVF; }
        else if fesr & FPE_IXC != 0 { code = FPE_FLTRES; }
    }
    force_sig_fault(sig, code, (*regs).pc as *mut core::ffi::c_void);
}

// The following C preprocessor-selected assembly is retained as Rust inline
// assembly.  Kernel configuration selects the same instruction sequence.
pub unsafe fn save_to_user_fp(user_fp: *mut user_fp) {
    let mut flg = 0usize;
    local_irq_save(&mut flg);
    (*user_fp).fcr = mfcr(b"cr<1, 2>\0".as_ptr());
    (*user_fp).fesr = mfcr(b"cr<2, 2>\0".as_ptr());
    let mut fpregs = (*user_fp).vr.as_mut_ptr();
    core::arch::asm!(
        "vstmu.32 vr0-vr3, ({p})", "vstmu.32 vr4-vr7, ({p})",
        "vstmu.32 vr8-vr11, ({p})", "vstmu.32 vr12-vr15, ({p})",
        "fstmu.64 vr16-vr31, ({p})", p = inout(reg) fpregs,
        options(nostack)
    );
    local_irq_restore(flg);
}

pub unsafe fn restore_from_user_fp(user_fp: *mut user_fp) {
    let mut flg = 0usize;
    local_irq_save(&mut flg);
    mtcr(b"cr<1, 2>\0".as_ptr(), (*user_fp).fcr);
    mtcr(b"cr<2, 2>\0".as_ptr(), (*user_fp).fesr);
    let mut fpregs = (*user_fp).vr.as_mut_ptr();
    core::arch::asm!(
        "vldmu.32 vr0-vr3, ({p})", "vldmu.32 vr4-vr7, ({p})",
        "vldmu.32 vr8-vr11, ({p})", "vldmu.32 vr12-vr15, ({p})",
        "fldmu.64 vr16-vr31, ({p})", p = inout(reg) fpregs,
        options(nostack)
    );
    local_irq_restore(flg);
}

// ABI-provided signal and floating-point exception constants.
extern "C" {
    static SIGFPE: i32; static SIGILL: i32;
    static FPE_FLTUNK: i32; static ILL_ILLOPC: i32; static ILL_ILLOPN: i32;
    static FPE_FLTINV: i32; static FPE_FLTDIV: i32; static FPE_FLTUND: i32;
    static FPE_FLTOVF: i32; static FPE_FLTRES: i32;
    static FPE_ILLE: u32; static FPE_IDC: u32; static FPE_FEC: u32;
    static FPE_IOC: u32; static FPE_DZC: u32; static FPE_UFC: u32;
    static FPE_OFC: u32; static FPE_IXC: u32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
