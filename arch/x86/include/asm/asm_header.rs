/* SPDX-License-Identifier: GPL-2.0 */
//! Rust translation of x86/include/asm/asm.h.
//!
//! C preprocessor branches depending on `__ASSEMBLER__`, `__x86_64__`,
//! `__KERNEL__`, and configuration symbols are represented with Rust cfgs.

#[cfg(target_arch = "x86_64")]
macro_rules! __asm_sel { ($a:expr, $b:expr) => { $b }; }
#[cfg(not(target_arch = "x86_64"))]
macro_rules! __asm_sel { ($a:expr, $b:expr) => { $a }; }

#[cfg(target_arch = "x86_64")]
macro_rules! __asm_reg { ($r:ident) => { concat!("r", stringify!($r)) }; }
#[cfg(not(target_arch = "x86_64"))]
macro_rules! __asm_reg { ($r:ident) => { concat!("e", stringify!($r)) }; }

macro_rules! _ASM_BYTES { ($($x:tt)*) => { concat!(" .byte ", stringify!($($x)*), " ;") }; }
macro_rules! __ASM_SIZE { ($i:ident) => { __asm_sel!(concat!(stringify!($i), "l"), concat!(stringify!($i), "q")) }; }
macro_rules! __ASM_REG { ($r:ident) => { __asm_reg!($r) }; }

#[cfg(target_arch = "x86_64")]
pub const _ASM_PTR: &str = ".quad";
#[cfg(not(target_arch = "x86_64"))]
pub const _ASM_PTR: &str = ".long";
#[cfg(target_arch = "x86_64")]
pub const _ASM_ALIGN: &str = ".balign 8";
#[cfg(not(target_arch = "x86_64"))]
pub const _ASM_ALIGN: &str = ".balign 4";

macro_rules! _ASM_MOV { () => { __ASM_SIZE!(mov) }; }
macro_rules! _ASM_INC { () => { __ASM_SIZE!(inc) }; }
macro_rules! _ASM_DEC { () => { __ASM_SIZE!(dec) }; }
macro_rules! _ASM_ADD { () => { __ASM_SIZE!(add) }; }
macro_rules! _ASM_SUB { () => { __ASM_SIZE!(sub) }; }
macro_rules! _ASM_XADD { () => { __ASM_SIZE!(xadd) }; }
macro_rules! _ASM_MUL { () => { __ASM_SIZE!(mul) }; }

macro_rules! _ASM_AX { () => { __ASM_REG!(ax) }; }
macro_rules! _ASM_BX { () => { __ASM_REG!(bx) }; }
macro_rules! _ASM_CX { () => { __ASM_REG!(cx) }; }
macro_rules! _ASM_DX { () => { __ASM_REG!(dx) }; }
macro_rules! _ASM_SP { () => { __ASM_REG!(sp) }; }
macro_rules! _ASM_BP { () => { __ASM_REG!(bp) }; }
macro_rules! _ASM_SI { () => { __ASM_REG!(si) }; }
macro_rules! _ASM_DI { () => { __ASM_REG!(di) }; }

/* Adds a (%rip) suffix on 64 bits only; for immediate memory references. */
#[cfg(target_arch = "x86_64")]
macro_rules! _ASM_RIP { ($x:expr) => { concat!($x, " (%%rip)") }; }
#[cfg(not(target_arch = "x86_64"))]
macro_rules! _ASM_RIP { ($x:expr) => { $x }; }

#[cfg(not(target_arch = "x86_64"))]
mod asm_args {
    pub const ARG1: &str = "eax"; pub const ARG2: &str = "edx"; pub const ARG3: &str = "ecx";
    pub const ARG1L: &str = "eax"; pub const ARG2L: &str = "edx"; pub const ARG3L: &str = "ecx";
    pub const ARG1W: &str = "ax"; pub const ARG2W: &str = "dx"; pub const ARG3W: &str = "cx";
    pub const ARG1B: &str = "al"; pub const ARG2B: &str = "dl"; pub const ARG3B: &str = "cl";
}

#[cfg(target_arch = "x86_64")]
mod asm_args {
    pub const ARG1: &str = "rdi"; pub const ARG2: &str = "rsi"; pub const ARG3: &str = "rdx";
    pub const ARG4: &str = "rcx"; pub const ARG5: &str = "r8"; pub const ARG6: &str = "r9";
    pub const ARG1Q: &str = "rdi"; pub const ARG2Q: &str = "rsi"; pub const ARG3Q: &str = "rdx";
    pub const ARG4Q: &str = "rcx"; pub const ARG5Q: &str = "r8"; pub const ARG6Q: &str = "r9";
    pub const ARG1L: &str = "edi"; pub const ARG2L: &str = "esi"; pub const ARG3L: &str = "edx";
    pub const ARG4L: &str = "ecx"; pub const ARG5L: &str = "r8d"; pub const ARG6L: &str = "r9d";
    pub const ARG1W: &str = "di"; pub const ARG2W: &str = "si"; pub const ARG3W: &str = "dx";
    pub const ARG4W: &str = "cx"; pub const ARG5W: &str = "r8w"; pub const ARG6W: &str = "r9w";
    pub const ARG1B: &str = "dil"; pub const ARG2B: &str = "sil"; pub const ARG3B: &str = "dl";
    pub const ARG4B: &str = "cl"; pub const ARG5B: &str = "r8b"; pub const ARG6B: &str = "r9b";
}

#[cfg(not(target_arch = "x86_64"))]
pub type EaxEdxValue = u64;
#[cfg(target_arch = "x86_64")]
pub type EaxEdxValue = (u64, u64);

#[cfg(target_arch = "x86_64")]
pub fn eax_edx_val(low: u64, high: u64) -> u64 { low | (high << 32) }
#[cfg(not(target_arch = "x86_64"))]
pub fn eax_edx_val(val: u64, _high: u64) -> u64 { val }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
