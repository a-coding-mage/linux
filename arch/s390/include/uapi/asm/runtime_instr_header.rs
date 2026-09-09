/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Translated from the Linux UAPI header. `__u64` and `__u32` correspond to
// `u64` and `u32` in Rust.

pub const S390_RUNTIME_INSTR_START: u32 = 0x1;
pub const S390_RUNTIME_INSTR_STOP: u32 = 0x2;

// The C bit-fields are represented by their three underlying 32-bit storage
// units, preserving the packed layout and field ordering.
#[repr(C, packed(1), align(8))]
pub struct runtime_instr_cb {
    pub rca: u64,
    pub roa: u64,
    pub rla: u64,
    pub flags1: u32,
    pub flags2: u32,
    pub flags3: u32,
    pub reserved7: u64,
    pub sf: u64,
    pub rsic: u64,
    pub reserved8: u64,
}

// Bit-field masks in the C declaration, relative to each 32-bit storage unit.
pub const RUNTIME_INSTR_CB_V: u32 = 1 << 0;
pub const RUNTIME_INSTR_CB_S: u32 = 1 << 1;
pub const RUNTIME_INSTR_CB_K: u32 = 1 << 2;
pub const RUNTIME_INSTR_CB_H: u32 = 1 << 3;
pub const RUNTIME_INSTR_CB_A: u32 = 1 << 4;
pub const RUNTIME_INSTR_CB_PS: u32 = 1 << 8;
pub const RUNTIME_INSTR_CB_QS: u32 = 1 << 9;
pub const RUNTIME_INSTR_CB_PC: u32 = 1 << 10;
pub const RUNTIME_INSTR_CB_QC: u32 = 1 << 11;
pub const RUNTIME_INSTR_CB_G: u32 = 1 << 13;
pub const RUNTIME_INSTR_CB_U: u32 = 1 << 14;
pub const RUNTIME_INSTR_CB_L: u32 = 1 << 15;
pub const RUNTIME_INSTR_CB_KEY: u32 = 0x000f_0000;
pub const RUNTIME_INSTR_CB_T: u32 = 1 << 28;
pub const RUNTIME_INSTR_CB_RGS: u32 = 0xE000_0000;

pub const RUNTIME_INSTR_CB_M: u32 = 0x0000_000f;
pub const RUNTIME_INSTR_CB_N: u32 = 1 << 4;
pub const RUNTIME_INSTR_CB_MAE: u32 = 1 << 5;
pub const RUNTIME_INSTR_CB_C: u32 = 1 << 8;
pub const RUNTIME_INSTR_CB_R: u32 = 1 << 9;
pub const RUNTIME_INSTR_CB_B: u32 = 1 << 10;
pub const RUNTIME_INSTR_CB_J: u32 = 1 << 11;
pub const RUNTIME_INSTR_CB_E: u32 = 1 << 12;
pub const RUNTIME_INSTR_CB_X: u32 = 1 << 13;
pub const RUNTIME_INSTR_CB_BPXN: u32 = 1 << 16;
pub const RUNTIME_INSTR_CB_BPXT: u32 = 1 << 17;
pub const RUNTIME_INSTR_CB_BPTI: u32 = 1 << 18;
pub const RUNTIME_INSTR_CB_BPNI: u32 = 1 << 19;
pub const RUNTIME_INSTR_CB_D: u32 = 1 << 0;
pub const RUNTIME_INSTR_CB_F: u32 = 1 << 1;
pub const RUNTIME_INSTR_CB_IC: u32 = 0x0000_003c;
pub const RUNTIME_INSTR_CB_DC: u32 = 0x0000_03c0;

// LRIC. The instruction encoding is architecture-specific and retained here
// as the direct inline-assembly operation used by the C header.
#[inline]
pub unsafe fn load_runtime_instr_cb(_cb: *mut runtime_instr_cb) {
    core::arch::asm!(".insn rsy,0xeb0000000060,0,0,{0}", in(reg) _cb);
}

// STRIC. The instruction encoding is architecture-specific and retained here
// as the direct inline-assembly operation used by the C header.
#[inline]
pub unsafe fn store_runtime_instr_cb(_cb: *mut runtime_instr_cb) {
    core::arch::asm!(".insn rsy,0xeb0000000061,0,0,{0}", in(reg) _cb, options(nostack));
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
