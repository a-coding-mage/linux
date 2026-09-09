/* SPDX-License-Identifier: GPL-2.0-only */
/* Translated from arch/arm/probes/decode.h. */

// C header dependencies: linux/types.h, linux/stddef.h, asm/probes.h,
// asm/ptrace.h, and asm/kprobes.h.

extern "C" {
    pub fn arm_probes_decode_init();
}

pub type ProbesCheckCc = unsafe extern "C" fn();
extern "C" {
    pub static probes_condition_checks: [*const ProbesCheckCc; 16];
}

pub const STR_PC_OFFSET: i32 = 8;
pub const LOAD_WRITE_PC_INTERWORKS: bool = true;
pub const ALU_WRITE_PC_INTERWORKS: bool = true;

#[repr(C)]
pub struct PtRegs {
    pub ARM_cpsr: libc::c_long,
    pub ARM_pc: libc::c_long,
}

#[inline]
pub unsafe fn bx_write_pc(mut pcv: libc::c_long, regs: *mut PtRegs) {
    let mut cpsr = (*regs).ARM_cpsr;
    if (pcv & 0x1) != 0 {
        cpsr |= 0x20;
        pcv &= !0x1;
    } else {
        cpsr &= !0x20;
        pcv &= !0x2;
    }
    (*regs).ARM_cpsr = cpsr;
    (*regs).ARM_pc = pcv;
}

#[inline]
pub unsafe fn load_write_pc(pcv: libc::c_long, regs: *mut PtRegs) {
    if LOAD_WRITE_PC_INTERWORKS {
        bx_write_pc(pcv, regs);
    } else {
        (*regs).ARM_pc = pcv;
    }
}

#[inline]
pub unsafe fn alu_write_pc(pcv: libc::c_long, regs: *mut PtRegs) {
    if ALU_WRITE_PC_INTERWORKS {
        bx_write_pc(pcv, regs);
    } else {
        (*regs).ARM_pc = pcv;
    }
}

#[inline]
pub const fn is_writeback(insn: u32) -> u32 {
    (insn ^ 0x0100_0000) & 0x0120_0000
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union DecodeItem {
    pub bits: u32,
    pub table: *const DecodeItem,
    pub action: i32,
}

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DecodeType {
    End,
    Table,
    Custom,
    Simulate,
    Emulate,
    Or,
    Reject,
    NumDecodeTypes,
}

pub const DECODE_TYPE_BITS: u32 = 4;
pub const DECODE_TYPE_MASK: u32 = (1 << DECODE_TYPE_BITS) - 1;

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum DecodeRegType {
    None = 0,
    Any,
    Sameas16,
    Sp,
    Pc,
    Nosp,
    Nosppc,
    Nopc,
    Nopcwb,
    Nopcx,
    Nosppcx,
}
pub const REG_TYPE_0: DecodeRegType = DecodeRegType::None;

#[macro_export]
macro_rules! REGS {
    ($r16:expr, $r12:expr, $r8:expr, $r4:expr, $r0:expr) => {
        (($r16 as u32) << 16) + (($r12 as u32) << 12) + (($r8 as u32) << 8)
            + (($r4 as u32) << 4) + ($r0 as u32)
    };
}

#[repr(C)]
pub struct DecodeHeader {
    pub type_regs: DecodeItem,
    pub mask: DecodeItem,
    pub value: DecodeItem,
}

#[repr(C)]
pub struct DecodeTable { pub header: DecodeHeader, pub table: DecodeItem }
#[repr(C)]
pub struct DecodeCustom { pub header: DecodeHeader, pub decoder: DecodeItem }
#[repr(C)]
pub struct DecodeSimulate { pub header: DecodeHeader, pub handler: DecodeItem }
#[repr(C)]
pub struct DecodeEmulate { pub header: DecodeHeader, pub handler: DecodeItem }
#[repr(C)]
pub struct DecodeOr { pub header: DecodeHeader }
#[repr(C)]
pub struct DecodeReject { pub header: DecodeHeader }

#[macro_export]
macro_rules! DECODE_HEADER {
    ($ty:expr, $mask:expr, $value:expr, $regs:expr) => {
        DecodeHeader {
            type_regs: DecodeItem { bits: ($ty as u32) | (($regs as u32) << DECODE_TYPE_BITS) },
            mask: DecodeItem { bits: $mask },
            value: DecodeItem { bits: $value },
        }
    };
}
#[macro_export]
macro_rules! DECODE_END { () => { DecodeItem { bits: DecodeType::End as u32 } }; }
#[macro_export]
macro_rules! DECODE_TABLE { ($mask:expr, $value:expr, $table:expr) => {
    (DECODE_HEADER!(DecodeType::Table, $mask, $value, 0), DecodeItem { table: $table })
}; }
#[macro_export]
macro_rules! DECODE_CUSTOM { ($mask:expr, $value:expr, $decoder:expr) => {
    (DECODE_HEADER!(DecodeType::Custom, $mask, $value, 0), DecodeItem { action: $decoder })
}; }
#[macro_export]
macro_rules! DECODE_SIMULATEX { ($mask:expr, $value:expr, $handler:expr, $regs:expr) => {
    (DECODE_HEADER!(DecodeType::Simulate, $mask, $value, $regs), DecodeItem { action: $handler })
}; }
#[macro_export]
macro_rules! DECODE_SIMULATE { ($mask:expr, $value:expr, $handler:expr) => {
    DECODE_SIMULATEX!($mask, $value, $handler, 0)
}; }
#[macro_export]
macro_rules! DECODE_EMULATEX { ($mask:expr, $value:expr, $handler:expr, $regs:expr) => {
    (DECODE_HEADER!(DecodeType::Emulate, $mask, $value, $regs), DecodeItem { action: $handler })
}; }
#[macro_export]
macro_rules! DECODE_EMULATE { ($mask:expr, $value:expr, $handler:expr) => {
    DECODE_EMULATEX!($mask, $value, $handler, 0)
}; }
#[macro_export]
macro_rules! DECODE_OR { ($mask:expr, $value:expr) => {
    DECODE_HEADER!(DecodeType::Or, $mask, $value, 0)
}; }
#[macro_export]
macro_rules! DECODE_REJECT { ($mask:expr, $value:expr) => {
    DECODE_HEADER!(DecodeType::Reject, $mask, $value, 0)
}; }

#[repr(i32)]
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum ProbesInsn { Rejected, Good, GoodNoSlot }

pub type ProbesOpcodeT = u32;
pub type ProbesInsnHandlerT = unsafe extern "C" fn();
pub type ProbesCustomDecodeT = unsafe extern "C" fn(ProbesOpcodeT, *mut ArchProbesInsn, *const DecodeHeader) -> ProbesInsn;
pub type ProbesCheckT = unsafe extern "C" fn(ProbesOpcodeT, *mut ArchProbesInsn, *const DecodeHeader) -> ProbesInsn;

#[repr(C)]
pub union DecodeAction {
    pub handler: *const ProbesInsnHandlerT,
    pub decoder: *const ProbesCustomDecodeT,
}
#[repr(C)]
pub struct DecodeChecker { pub checker: *const ProbesCheckT }

#[repr(C)]
pub struct ArchProbesInsn;

extern "C" {
    pub static mut probes_simulate_nop: ProbesInsnHandlerT;
    pub static mut probes_emulate_none: ProbesInsnHandlerT;
    pub fn probes_decode_insn(insn: ProbesOpcodeT, asi: *mut ArchProbesInsn,
        table: *const DecodeItem, thumb: bool, emulate: bool,
        actions: *const DecodeAction, checkers: *const *const DecodeChecker) -> ProbesInsn;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
