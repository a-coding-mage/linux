/* SPDX-License-Identifier: GPL-2.0 */

// Dependency supplied by the translated pstate header.
// use crate::asm::pstate::*;

pub const KERNBASE: u64 = 0x400000;

pub const PTREGS_OFF: usize = STACK_BIAS + STACKFRAME_SZ;

pub const RTRAP_PSTATE: u64 = PSTATE_TSO | PSTATE_PEF | PSTATE_PRIV | PSTATE_IE;
pub const RTRAP_PSTATE_IRQOFF: u64 = PSTATE_TSO | PSTATE_PEF | PSTATE_PRIV;
pub const RTRAP_PSTATE_AG_IRQOFF: u64 = PSTATE_TSO | PSTATE_PEF | PSTATE_PRIV | PSTATE_AG;

pub const __CHEETAH_ID: u64 = 0x003e0014;
pub const __JALAPENO_ID: u64 = 0x003e0016;
pub const __SERRANO_ID: u64 = 0x003e0022;

pub const CHEETAH_MANUF: u64 = 0x003e;
pub const CHEETAH_IMPL: u64 = 0x0014; // Ultra-III
pub const CHEETAH_PLUS_IMPL: u64 = 0x0015; // Ultra-III+
pub const JALAPENO_IMPL: u64 = 0x0016; // Ultra-IIIi
pub const JAGUAR_IMPL: u64 = 0x0018; // Ultra-IV
pub const PANTHER_IMPL: u64 = 0x0019; // Ultra-IV+
pub const SERRANO_IMPL: u64 = 0x0022; // Ultra-IIIi+

// wrpr %g0, val, %gl
// The original expands to a SPARC instruction word; Rust callers targeting
// SPARC should emit this word with the platform's inline-assembly facility.
#[macro_export]
macro_rules! SET_GL {
    ($val:expr) => {
        (0xa1902000u32 | ($val as u32))
    };
}

// rdpr %gl, %gN
#[macro_export]
macro_rules! GET_GL_GLOBAL {
    ($n:expr) => {
        (0x81540000u32 | (($n as u32) << 25))
    };
}

// The following macros are SPARC assembly sequences.  They are retained as
// declarative placeholders because their register operands and branch labels
// have no direct file-local Rust equivalent.
#[macro_export]
macro_rules! BRANCH_IF_SUN4V { ($tmp1:tt, $label:tt) => {{ /* sethi/lduw/brnz,pn is_sun4v */ }}; }

#[macro_export]
macro_rules! BRANCH_IF_CHEETAH_BASE { ($tmp1:tt, $tmp2:tt, $label:tt) => {{ /* rdpr/sethi/srlx/or/cmp/be,pn __CHEETAH_ID */ }}; }

#[macro_export]
macro_rules! BRANCH_IF_JALAPENO { ($tmp1:tt, $tmp2:tt, $label:tt) => {{ /* rdpr/sethi/srlx/or/cmp/be,pn __JALAPENO_ID */ }}; }

#[macro_export]
macro_rules! BRANCH_IF_CHEETAH_PLUS_OR_FOLLOWON { ($tmp1:tt, $tmp2:tt, $label:tt) => {{ /* manufacturer/implementation comparison and branch */ }}; }

#[macro_export]
macro_rules! BRANCH_IF_ANY_CHEETAH { ($tmp1:tt, $tmp2:tt, $label:tt) => {{ /* manufacturer/implementation comparison and branch */ }}; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
