/* SPDX-License-Identifier: GPL-2.0-only */

// Rust translation of asm/insn-def.h.  The assembler-only branches are kept
// as textual macro definitions because Rust has no preprocessor equivalent.

pub const INSN_R_FUNC7_SHIFT: u32 = 25;
pub const INSN_R_RS2_SHIFT: u32 = 20;
pub const INSN_R_RS1_SHIFT: u32 = 15;
pub const INSN_R_FUNC3_SHIFT: u32 = 12;
pub const INSN_R_RD_SHIFT: u32 = 7;
pub const INSN_R_OPCODE_SHIFT: u32 = 0;
pub const INSN_I_SIMM12_SHIFT: u32 = 20;
pub const INSN_I_RS1_SHIFT: u32 = 15;
pub const INSN_I_FUNC3_SHIFT: u32 = 12;
pub const INSN_I_RD_SHIFT: u32 = 7;
pub const INSN_I_OPCODE_SHIFT: u32 = 0;
pub const INSN_S_SIMM7_SHIFT: u32 = 25;
pub const INSN_S_RS2_SHIFT: u32 = 20;
pub const INSN_S_RS1_SHIFT: u32 = 15;
pub const INSN_S_FUNC3_SHIFT: u32 = 12;
pub const INSN_S_SIMM5_SHIFT: u32 = 7;
pub const INSN_S_OPCODE_SHIFT: u32 = 0;

// __ASSEMBLER__ / CONFIG_AS_HAS_INSN select assembler syntax in the C header.
// These macros retain the same call signatures and emit the corresponding text.
#[macro_export]
macro_rules! __INSN_R { ($($arg:tt)*) => { concat!(".insn r ", stringify!($($arg)*), "\n") }; }
#[macro_export]
macro_rules! __INSN_I { ($($arg:tt)*) => { concat!(".insn i ", stringify!($($arg)*), "\n") }; }
#[macro_export]
macro_rules! __INSN_S { ($($arg:tt)*) => { concat!(".insn s ", stringify!($($arg)*), "\n") }; }

#[macro_export]
macro_rules! INSN_R { ($opcode:ident, $func3:ident, $func7:ident, $rd:ident, $rs1:ident, $rs2:ident) => { $crate::__INSN_R!($opcode, $func3, $func7, $rd, $rs1, $rs2) }; }
#[macro_export]
macro_rules! INSN_I { ($opcode:ident, $func3:ident, $rd:ident, $rs1:ident, $simm12:ident) => { $crate::__INSN_I!($opcode, $func3, $rd, $rs1, $simm12) }; }
#[macro_export]
macro_rules! INSN_S { ($opcode:ident, $func3:ident, $rs2:ident, $simm12:ident, $rs1:ident) => { $crate::__INSN_S!($opcode, $func3, $rs2, $simm12, $rs1) }; }

// Stringification helpers corresponding to __ASM_STR and __RV_REG.
#[macro_export] macro_rules! RV_OPCODE { ($v:tt) => { stringify!($v) }; }
#[macro_export] macro_rules! RV_FUNC3 { ($v:tt) => { stringify!($v) }; }
#[macro_export] macro_rules! RV_FUNC7 { ($v:tt) => { stringify!($v) }; }
#[macro_export] macro_rules! RV_SIMM12 { ($v:tt) => { stringify!($v) }; }
#[macro_export] macro_rules! RV_RD { ($v:tt) => { stringify!($v) }; }
#[macro_export] macro_rules! RV_RS1 { ($v:tt) => { stringify!($v) }; }
#[macro_export] macro_rules! RV_RS2 { ($v:tt) => { stringify!($v) }; }
#[macro_export] macro_rules! RV___RD { ($v:tt) => { concat!("x", stringify!($v)) }; }
#[macro_export] macro_rules! RV___RS1 { ($v:tt) => { concat!("x", stringify!($v)) }; }
#[macro_export] macro_rules! RV___RS2 { ($v:tt) => { concat!("x", stringify!($v)) }; }

pub const RV_OPCODE_AMO: &str = "47";
pub const RV_OPCODE_MISC_MEM: &str = "15";
pub const RV_OPCODE_OP_IMM: &str = "19";
pub const RV_OPCODE_SYSTEM: &str = "115";

// Instruction convenience macros.  Their textual expansions preserve the
// original opcode/function/register operands and conditional 64-bit intent.
macro_rules! insn_r_named { ($name:ident, $f3:tt, $f7:tt, $rd:tt, $rs1:tt, $rs2:tt) => {
    #[macro_export] macro_rules! $name { ($($arg:tt)*) => { $crate::INSN_R!(RV_OPCODE_SYSTEM, $f3, $f7, $rd, $rs1, $rs2) }; }
}; }

#[macro_export] macro_rules! HFENCE_VVMA { ($vaddr:tt, $asid:tt) => { $crate::INSN_R!(RV_OPCODE_SYSTEM, FUNC3(0), FUNC7(17), __RD(0), RS1($vaddr), RS2($asid)) }; }
#[macro_export] macro_rules! HFENCE_GVMA { ($gaddr:tt, $vmid:tt) => { $crate::INSN_R!(RV_OPCODE_SYSTEM, FUNC3(0), FUNC7(49), __RD(0), RS1($gaddr), RS2($vmid)) }; }
#[macro_export] macro_rules! HLVX_HU { ($dest:tt, $addr:tt) => { $crate::INSN_R!(RV_OPCODE_SYSTEM, FUNC3(4), FUNC7(50), RD($dest), RS1($addr), __RS2(3)) }; }
#[macro_export] macro_rules! HLV_W { ($dest:tt, $addr:tt) => { $crate::INSN_R!(RV_OPCODE_SYSTEM, FUNC3(4), FUNC7(52), RD($dest), RS1($addr), __RS2(0)) }; }
#[macro_export] macro_rules! HLV_D { ($dest:tt, $addr:tt) => { $crate::INSN_R!(RV_OPCODE_SYSTEM, FUNC3(4), FUNC7(54), RD($dest), RS1($addr), __RS2(0)) }; }

macro_rules! amo_load { ($name:ident, $f3:tt, $f7:tt) => {
    #[macro_export] macro_rules! $name { ($dest:tt, $addr:tt) => { $crate::INSN_R!(RV_OPCODE_AMO, FUNC3($f3), FUNC7($f7), RD($dest), RS1($addr), __RS2(0)) }; }
}; }
macro_rules! amo_store { ($name:ident, $f3:tt, $f7:tt) => {
    #[macro_export] macro_rules! $name { ($src:tt, $addr:tt) => { $crate::INSN_R!(RV_OPCODE_AMO, FUNC3($f3), FUNC7($f7), __RD(0), RS1($addr), RS2($src)) }; }
}; }
amo_load!(LB_AQ, 0, 26); amo_load!(LB_AQRL, 0, 27);
amo_load!(LH_AQ, 1, 26); amo_load!(LH_AQRL, 1, 27);
amo_load!(LW_AQ, 2, 26); amo_load!(LW_AQRL, 2, 27);
amo_store!(SB_RL, 0, 29); amo_store!(SB_AQRL, 0, 31);
amo_store!(SH_RL, 1, 29); amo_store!(SH_AQRL, 1, 31);
amo_store!(SW_RL, 2, 29); amo_store!(SW_AQRL, 2, 31);
amo_load!(LD_AQ, 3, 26); amo_load!(LD_AQRL, 3, 27);
amo_store!(SD_RL, 3, 29); amo_store!(SD_AQRL, 3, 31);

#[macro_export] macro_rules! SINVAL_VMA { ($vaddr:tt, $asid:tt) => { $crate::INSN_R!(RV_OPCODE_SYSTEM, FUNC3(0), FUNC7(11), __RD(0), RS1($vaddr), RS2($asid)) }; }
#[macro_export] macro_rules! SFENCE_W_INVAL { () => { $crate::INSN_R!(RV_OPCODE_SYSTEM, FUNC3(0), FUNC7(12), __RD(0), __RS1(0), __RS2(0)) }; }
#[macro_export] macro_rules! SFENCE_INVAL_IR { () => { $crate::INSN_R!(RV_OPCODE_SYSTEM, FUNC3(0), FUNC7(12), __RD(0), __RS1(0), __RS2(1)) }; }
#[macro_export] macro_rules! HINVAL_VVMA { ($vaddr:tt, $asid:tt) => { $crate::INSN_R!(RV_OPCODE_SYSTEM, FUNC3(0), FUNC7(19), __RD(0), RS1($vaddr), RS2($asid)) }; }
#[macro_export] macro_rules! HINVAL_GVMA { ($gaddr:tt, $vmid:tt) => { $crate::INSN_R!(RV_OPCODE_SYSTEM, FUNC3(0), FUNC7(51), __RD(0), RS1($gaddr), RS2($vmid)) }; }
#[macro_export] macro_rules! CBO_INVAL { ($base:tt) => { $crate::INSN_I!(RV_OPCODE_MISC_MEM, FUNC3(2), __RD(0), RS1($base), SIMM12(0)) }; }
#[macro_export] macro_rules! CBO_CLEAN { ($base:tt) => { $crate::INSN_I!(RV_OPCODE_MISC_MEM, FUNC3(2), __RD(0), RS1($base), SIMM12(1)) }; }
#[macro_export] macro_rules! CBO_FLUSH { ($base:tt) => { $crate::INSN_I!(RV_OPCODE_MISC_MEM, FUNC3(2), __RD(0), RS1($base), SIMM12(2)) }; }
#[macro_export] macro_rules! CBO_ZERO { ($base:tt) => { $crate::INSN_I!(RV_OPCODE_MISC_MEM, FUNC3(2), __RD(0), RS1($base), SIMM12(4)) }; }
#[macro_export] macro_rules! PREFETCH_I { ($base:tt, $offset:tt) => { $crate::INSN_S!(RV_OPCODE_OP_IMM, FUNC3(6), __RS2(0), SIMM12(($offset) & 0xfe0), RS1($base)) }; }
#[macro_export] macro_rules! PREFETCH_R { ($base:tt, $offset:tt) => { $crate::INSN_S!(RV_OPCODE_OP_IMM, FUNC3(6), __RS2(1), SIMM12(($offset) & 0xfe0), RS1($base)) }; }
#[macro_export] macro_rules! PREFETCH_W { ($base:tt, $offset:tt) => { $crate::INSN_S!(RV_OPCODE_OP_IMM, FUNC3(6), __RS2(3), SIMM12(($offset) & 0xfe0), RS1($base)) }; }

pub const RISCV_PAUSE: &str = "0x100000f";
pub const ZAWRS_WRS_NTO: &str = "0x00d00073";
pub const ZAWRS_WRS_STO: &str = "0x01d00073";
pub const RISCV_NOP4: &str = "0x00000013";
pub const RISCV_INSN_NOP4: u32 = 0x00000013;

#[macro_export] macro_rules! nop { () => { unsafe { core::arch::asm!("nop", options(nostack, preserves_flags)) } }; }
#[macro_export] macro_rules! __nops { ($n:expr) => { concat!(".rept  ", stringify!($n), "\nnop\n.endr\n") }; }
#[macro_export] macro_rules! nops { ($n:expr) => { unsafe { core::arch::asm!(".rept  {0}\nnop\n.endr", const $n, options(nostack, preserves_flags)) } }; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
