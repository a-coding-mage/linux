/* SPDX-License-Identifier: GPL-2.0-or-later */
/*
 * Copyright (C) 2004 Paul Mackerras <paulus@au.ibm.com>, IBM
 */
// Dependency: asm/inst.h

// Opaque declaration supplied by the surrounding kernel bindings.
#[repr(C)]
pub struct pt_regs {
    _private: [u8; 0],
}

/*
 * We don't allow single-stepping an mtmsrd that would clear
 * MSR_RI, since that would make the exception unrecoverable.
 * Since we need to single-step to proceed from a breakpoint,
 * we don't allow putting a breakpoint on an mtmsrd instruction.
 * Similarly we don't allow breakpoints on rfid instructions.
 * These macros tell us if an instruction is a mtmsrd or rfid.
 * Note that these return true for both mtmsr/rfi (32-bit)
 * and mtmsrd/rfid (64-bit).
 */
#[inline]
pub unsafe fn is_mtmsrd(instr: ppc_inst_t) -> bool {
    (ppc_inst_val(instr) & 0xfc0007be) == 0x7c000124
}

#[inline]
pub unsafe fn is_rfid(instr: ppc_inst_t) -> bool {
    (ppc_inst_val(instr) & 0xfc0007be) == 0x4c000024
}

#[repr(C)]
#[derive(Copy, Clone, PartialEq, Eq)]
pub enum instruction_type {
    COMPUTE, // arith/logical/CR op, etc.
    LOAD, // load and store types need to be contiguous
    LOAD_MULTI,
    LOAD_FP,
    LOAD_VMX,
    LOAD_VSX,
    STORE,
    STORE_MULTI,
    STORE_FP,
    STORE_VMX,
    STORE_VSX,
    LARX,
    STCX,
    BRANCH,
    MFSPR,
    MTSPR,
    CACHEOP,
    BARRIER,
    SYSCALL,
    SYSCALL_VECTORED_0,
    MFMSR,
    MTMSR,
    RFI,
    INTERRUPT,
    UNKNOWN,
}

pub const INSTR_TYPE_MASK: i32 = 0x1f;
pub const SETREG: i32 = 0x20;
pub const SETCC: i32 = 0x40;
pub const SETXER: i32 = 0x80;
pub const SETLK: i32 = 0x20;
pub const BRTAKEN: i32 = 0x40;
pub const DECCTR: i32 = 0x80;
pub const SIGNEXT: i32 = 0x20;
pub const UPDATE: i32 = 0x40;
pub const BYTEREV: i32 = 0x80;
pub const FPCONV: i32 = 0x100;
pub const BARRIER_MASK: i32 = 0xe0;
pub const BARRIER_SYNC: i32 = 0x00;
pub const BARRIER_ISYNC: i32 = 0x20;
pub const BARRIER_EIEIO: i32 = 0x40;
pub const BARRIER_LWSYNC: i32 = 0x60;
pub const BARRIER_PTESYNC: i32 = 0x80;
pub const CACHEOP_MASK: i32 = 0x700;
pub const DCBST: i32 = 0;
pub const DCBF: i32 = 0x100;
pub const DCBTST: i32 = 0x200;
pub const DCBT: i32 = 0x300;
pub const ICBI: i32 = 0x400;
pub const DCBZ: i32 = 0x500;
pub const VSX_FPCONV: i32 = 1;
pub const VSX_SPLAT: i32 = 2;
pub const VSX_LDLEFT: i32 = 4;
pub const VSX_CHECK_VEC: i32 = 8;
pub const PREFIXED: i32 = 0x800;

#[inline] pub const fn size(n: i32) -> i32 { n << 12 }
#[inline] pub const fn getsize(w: i32) -> i32 { w >> 12 }
#[inline] pub const fn gettype(t: i32) -> i32 { t & INSTR_TYPE_MASK }
#[inline] pub const fn getlength(t: i32) -> i32 { if t & PREFIXED != 0 { 8 } else { 4 } }
#[inline] pub const fn mkop(t: i32, f: i32, s: i32) -> i32 { t | f | size(s) }
#[inline] pub const fn get_prefix_ra(i: usize) -> usize { (i >> 16) & 0x1f }
#[inline] pub const fn get_prefix_r(i: usize) -> usize { i & (1usize << 20) }

#[repr(C)]
pub struct instruction_op {
    pub type_: i32,
    pub reg: i32,
    pub val: usize,
    pub ea: usize,
    pub update_reg: i32,
    pub spr: i32,
    pub ccval: u32,
    pub xerval: u32,
    pub element_size: u8,
    pub vsx_flags: u8,
}

#[repr(C)]
pub union vsx_reg {
    pub b: [u8; 16],
    pub h: [u16; 8],
    pub w: [u32; 4],
    pub d: [usize; 2],
    pub fp: [f32; 4],
    pub dp: [f64; 2],
    pub v: __vector128,
}

extern "C" {
    pub static mut patch__exec_instr: i32;
    pub fn analyse_instr(op: *mut instruction_op, regs: *const pt_regs, instr: ppc_inst_t) -> i32;
    pub fn emulate_update_regs(reg: *mut pt_regs, op: *mut instruction_op);
    pub fn emulate_step(regs: *mut pt_regs, instr: ppc_inst_t) -> i32;
    pub fn emulate_loadstore(regs: *mut pt_regs, op: *mut instruction_op) -> i32;
    pub fn emulate_dcbz(ea: usize, regs: *mut pt_regs) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
