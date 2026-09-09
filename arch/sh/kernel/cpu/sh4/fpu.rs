// SPDX-License-Identifier: GPL-2.0
/*
 * Save/restore floating point context for signal handlers.
 *
 * Copyright (C) 1999, 2000  Kaz Kojima & Niibe Yutaka
 * Copyright (C) 2006  ST Microelectronics Ltd. (denorm support)
 *
 * FIXME! These routines have not been tested for big endian case.
 */

// The PR (precision) bit in the FP Status Register must be clear when an
// frchg instruction is executed, otherwise the instruction is undefined.
// Executing frchg with PR set causes a trap on some SH4 implementations.

const FPSCR_RCHG: usize = 0x00000000;

extern "C" {
    fn float64_div(a: u64, b: u64) -> u64;
    fn float32_div(a: u32, b: u32) -> u32;
    fn float64_mul(a: u64, b: u64) -> u64;
    fn float32_mul(a: u32, b: u32) -> u32;
    fn float64_add(a: u64, b: u64) -> u64;
    fn float32_add(a: u32, b: u32) -> u32;
    fn float64_sub(a: u64, b: u64) -> u64;
    fn float32_sub(a: u32, b: u32) -> u32;
    fn float64_to_float32(a: u64) -> u32;
}

static mut fpu_exception_flags: u32 = 0;

// External kernel types and symbols supplied by the surrounding kernel.
#[repr(C)]
pub struct task_struct {
    pub thread: thread_struct,
}
#[repr(C)]
pub struct thread_struct {
    pub xstate: *mut sh_fpu_xstate,
}
#[repr(C)]
pub struct sh_fpu_xstate {
    pub hardfpu: sh_fpu_hard_struct,
}
#[repr(C)]
pub struct sh_fpu_hard_struct {
    pub status: u32,
    pub fpscr: u32,
    pub fpul: u32,
    pub fp_regs: [u32; 32],
}
#[repr(C)]
pub struct pt_regs {
    pub pc: usize,
    pub pr: usize,
    pub sr: usize,
    pub regs: [usize; 16],
}
extern "C" {
    static mut current: *mut task_struct;
    fn enable_fpu();
    fn disable_fpu();
    fn instruction_size(insn: u16) -> usize;
    fn __unlazy_fpu(tsk: *mut task_struct, regs: *mut pt_regs);
    fn grab_fpu(regs: *mut pt_regs);
    fn force_sig(sig: i32);
    fn task_thread_info(tsk: *mut task_struct) -> *mut thread_info;
}
#[repr(C)]
pub struct thread_info { pub status: usize }

// These constants are supplied by the SH FPU headers.
extern "C" {
    static FPSCR_INIT: u32;
    static FPSCR_CAUSE_ERROR: u32;
    static FPSCR_DBL_PRECISION: u32;
    static FPSCR_CAUSE_MASK: u32;
    static FPSCR_FLAG_MASK: u32;
    static FPSCR_ENABLE_MASK: u32;
    static FPSCR_ROUNDING_MODE: unsafe extern "C" fn(u32) -> i32;
}

pub unsafe fn save_fpu(tsk: *mut task_struct) {
    let mut dummy: usize;
    enable_fpu();
    core::arch::asm!(
        "sts.l fpul, @-{0}", "sts.l fpscr, @-{0}", "lds {1}, fpscr", "frchg",
        "fmov.s fr15, @-{0}", "fmov.s fr14, @-{0}", "fmov.s fr13, @-{0}", "fmov.s fr12, @-{0}",
        "fmov.s fr11, @-{0}", "fmov.s fr10, @-{0}", "fmov.s fr9, @-{0}", "fmov.s fr8, @-{0}",
        "fmov.s fr7, @-{0}", "fmov.s fr6, @-{0}", "fmov.s fr5, @-{0}", "fmov.s fr4, @-{0}",
        "fmov.s fr3, @-{0}", "fmov.s fr2, @-{0}", "fmov.s fr1, @-{0}", "fmov.s fr0, @-{0}",
        "frchg",
        "fmov.s fr15, @-{0}", "fmov.s fr14, @-{0}", "fmov.s fr13, @-{0}", "fmov.s fr12, @-{0}",
        "fmov.s fr11, @-{0}", "fmov.s fr10, @-{0}", "fmov.s fr9, @-{0}", "fmov.s fr8, @-{0}",
        "fmov.s fr7, @-{0}", "fmov.s fr6, @-{0}", "fmov.s fr5, @-{0}", "fmov.s fr4, @-{0}",
        "fmov.s fr3, @-{0}", "fmov.s fr2, @-{0}", "fmov.s fr1, @-{0}", "fmov.s fr0, @-{0}",
        "lds {2}, fpscr", inout(reg) dummy, in(reg) FPSCR_RCHG, in(reg) FPSCR_INIT,
        inout(reg) ((*(*tsk).thread.xstate).hardfpu.status as *mut u32), options(nostack));
    disable_fpu();
}

pub unsafe fn restore_fpu(tsk: *mut task_struct) {
    let mut dummy: usize;
    enable_fpu();
    core::arch::asm!(
        "lds {1}, fpscr",
        "fmov.s @{0}+, fr0", "fmov.s @{0}+, fr1", "fmov.s @{0}+, fr2", "fmov.s @{0}+, fr3",
        "fmov.s @{0}+, fr4", "fmov.s @{0}+, fr5", "fmov.s @{0}+, fr6", "fmov.s @{0}+, fr7",
        "fmov.s @{0}+, fr8", "fmov.s @{0}+, fr9", "fmov.s @{0}+, fr10", "fmov.s @{0}+, fr11",
        "fmov.s @{0}+, fr12", "fmov.s @{0}+, fr13", "fmov.s @{0}+, fr14", "fmov.s @{0}+, fr15",
        "frchg",
        "fmov.s @{0}+, fr0", "fmov.s @{0}+, fr1", "fmov.s @{0}+, fr2", "fmov.s @{0}+, fr3",
        "fmov.s @{0}+, fr4", "fmov.s @{0}+, fr5", "fmov.s @{0}+, fr6", "fmov.s @{0}+, fr7",
        "fmov.s @{0}+, fr8", "fmov.s @{0}+, fr9", "fmov.s @{0}+, fr10", "fmov.s @{0}+, fr11",
        "fmov.s @{0}+, fr12", "fmov.s @{0}+, fr13", "fmov.s @{0}+, fr14", "fmov.s @{0}+, fr15",
        "frchg", "lds.l @{0}+, fpscr", "lds.l @{0}+, fpul",
        inout(reg) dummy, in(reg) FPSCR_RCHG, in("r0") (*tsk).thread.xstate, options(nostack));
    disable_fpu();
}

unsafe fn denormal_to_double(fpu: *mut sh_fpu_hard_struct, n: i32) {
    let mut x = (*fpu).fpul;
    let mut exp: i32 = 1023 - 126;
    if x != 0 && (x & 0x7f800000) == 0 {
        let mut du = x & 0x80000000;
        while (x & 0x00800000) == 0 { x <<= 1; exp -= 1; }
        x &= 0x007fffff;
        du |= ((exp as u32) << 20) | (x >> 3);
        (*fpu).fp_regs[n as usize] = du;
        (*fpu).fp_regs[n as usize + 1] = x << 29;
    }
}

unsafe fn ieee_fpe_handler(regs: *mut pt_regs) -> i32 {
    let insn = *((*regs).pc as *const u16);
    let nib = [(insn >> 12) & 0xf, (insn >> 8) & 0xf, (insn >> 4) & 0xf, insn & 0xf];
    let mut nextpc: usize;
    let finsn: u16;
    if nib[0] == 0xb || (nib[0] == 4 && nib[2] == 0 && nib[3] == 0xb) { (*regs).pr = (*regs).pc + 4; }
    if nib[0] == 0xa || nib[0] == 0xb {
        nextpc = (*regs).pc + 4 + ((((insn & 0xfff) as i16) << 4 >> 3) as usize);
        finsn = *(((*regs).pc + 2) as *const u16);
    } else if nib[0] == 8 && nib[1] == 0xd {
        nextpc = if (*regs).sr & 1 != 0 { (*regs).pc + 4 + (((insn & 0xff) as i8 as isize * 2) as usize) } else { (*regs).pc + 4 };
        finsn = *(((*regs).pc + 2) as *const u16);
    } else if nib[0] == 8 && nib[1] == 0xf {
        nextpc = if (*regs).sr & 1 != 0 { (*regs).pc + 4 } else { (*regs).pc + 4 + (((insn & 0xff) as i8 as isize * 2) as usize) };
        finsn = *(((*regs).pc + 2) as *const u16);
    } else if nib[0] == 4 && nib[3] == 0xb && (nib[2] == 0 || nib[2] == 2) {
        nextpc = (*regs).regs[nib[1] as usize]; finsn = *(((*regs).pc + 2) as *const u16);
    } else if nib[0] == 0 && nib[3] == 3 && (nib[2] == 0 || nib[2] == 2) {
        nextpc = (*regs).pc + 4 + (*regs).regs[nib[1] as usize]; finsn = *(((*regs).pc + 2) as *const u16);
    } else if insn == 0x000b { nextpc = (*regs).pr; finsn = *(((*regs).pc + 2) as *const u16); }
    else { nextpc = (*regs).pc + instruction_size(insn); finsn = insn; }
    let tsk = current;
    let fpu = &mut (*(*tsk).thread.xstate).hardfpu;
    let n = ((finsn >> 8) & 0xf) as usize;
    let m = ((finsn >> 4) & 0xf) as usize;
    let fpscr = fpu.fpscr;
    let prec = fpscr & FPSCR_DBL_PRECISION;
    if (finsn & 0xf1ff) == 0xf0ad { if fpscr & FPSCR_CAUSE_ERROR != 0 { denormal_to_double(fpu, n as i32); } else { return 0; } }
    else if (finsn & 0xf00f) == 0xf002 || (finsn & 0xf00e) == 0xf000 || (finsn & 0xf003) == 0xf003 {
        let hx = fpu.fp_regs[n]; let hy = fpu.fp_regs[m];
        if fpscr & FPSCR_CAUSE_ERROR == 0 || (prec != 0 && ((hx & 0x7fffffff) >= 0x00100000 && (hy & 0x7fffffff) >= 0x00100000)) || (prec == 0 && ((hx & 0x7fffffff) >= 0x00800000 && (hy & 0x7fffffff) >= 0x00800000)) { return 0; }
        if prec != 0 { let x = ((hx as u64) << 32) | fpu.fp_regs[n+1] as u64; let y = ((hy as u64) << 32) | fpu.fp_regs[m+1] as u64; let z = if (finsn & 0xf00f)==0xf002 { float64_mul(x,y) } else if (finsn & 0xf00f)==0xf000 { float64_add(x,y) } else if (finsn & 0xf00f)==0xf001 { float64_sub(x,y) } else { float64_div(x,y) }; fpu.fp_regs[n]= (z>>32) as u32; fpu.fp_regs[n+1]=z as u32; } else { let z = if (finsn & 0xf00f)==0xf002 {float32_mul(hx,hy)} else if (finsn & 0xf00f)==0xf000 {float32_add(hx,hy)} else if (finsn & 0xf00f)==0xf001 {float32_sub(hx,hy)} else {float32_div(hx,hy)}; fpu.fp_regs[n]=z; }
    } else if (finsn & 0xf0bd) == 0xf0bd { let hx=fpu.fp_regs[n]; if fpscr & FPSCR_CAUSE_ERROR == 0 || (hx&0x7fffffff)>=0x00100000 {return 0;} let x=((fpu.fp_regs[n] as u64)<<32)|fpu.fp_regs[n+1] as u64; fpu.fpul=float64_to_float32(x); }
    else { return 0; }
    (*regs).pc=nextpc; 1
}

pub unsafe fn float_raise(flags: u32) { fpu_exception_flags |= flags; }
pub unsafe fn float_rounding_mode() -> i32 { FPSCR_ROUNDING_MODE((*(*current).thread.xstate).hardfpu.fpscr) }

pub unsafe fn fpu_error(regs: *mut pt_regs) {
    let tsk=current; __unlazy_fpu(tsk, regs); fpu_exception_flags=0;
    if ieee_fpe_handler(regs) != 0 { let fpu=&mut (*(*tsk).thread.xstate).hardfpu; fpu.fpscr &= !(FPSCR_CAUSE_MASK|FPSCR_FLAG_MASK); fpu.fpscr |= fpu_exception_flags | (fpu_exception_flags>>10); grab_fpu(regs); restore_fpu(tsk); (*task_thread_info(tsk)).status |= TS_USEDFPU; if (((fpu.fpscr & FPSCR_ENABLE_MASK)>>7) & (fpu_exception_flags>>2)) == 0 {return;} }
    force_sig(SIGFPE);
}

const TS_USEDFPU: usize = 0;
const SIGFPE: i32 = 8;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
