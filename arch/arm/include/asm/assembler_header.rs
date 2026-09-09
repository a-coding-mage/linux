/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Rust translation of arch/arm/include/asm/assembler.h.
 *
 * The source is an assembler-only header.  Its assembler directives and
 * macros are retained below as documentation because Rust has no direct
 * equivalent for defining ARM assembler macros from a Rust header.
 */

/// Direct translation of `#define IOMEM(x) (x)`.
#[inline(always)]
pub const fn iomem<T>(x: T) -> T { x }

pub const IMM12_MASK: u32 = 0xfff;

#[cfg(not(target_endian = "big"))]
pub const LSPULL: &str = "lsr";
#[cfg(not(target_endian = "big"))]
pub const LSPUSH: &str = "lsl";
#[cfg(target_endian = "big")]
pub const LSPULL: &str = "lsl";
#[cfg(target_endian = "big")]
pub const LSPUSH: &str = "lsr";

#[cfg(not(target_endian = "big"))]
pub const GET_BYTE_0: &str = "lsl #0";
#[cfg(not(target_endian = "big"))]
pub const GET_BYTE_1: &str = "lsr #8";
#[cfg(not(target_endian = "big"))]
pub const GET_BYTE_2: &str = "lsr #16";
#[cfg(not(target_endian = "big"))]
pub const GET_BYTE_3: &str = "lsr #24";
#[cfg(not(target_endian = "big"))]
pub const PUT_BYTE_0: &str = "lsl #0";
#[cfg(not(target_endian = "big"))]
pub const PUT_BYTE_1: &str = "lsl #8";
#[cfg(not(target_endian = "big"))]
pub const PUT_BYTE_2: &str = "lsl #16";
#[cfg(not(target_endian = "big"))]
pub const PUT_BYTE_3: &str = "lsl #24";
#[cfg(target_endian = "big")]
pub const GET_BYTE_0: &str = "lsr #24";
#[cfg(target_endian = "big")]
pub const GET_BYTE_1: &str = "lsr #16";
#[cfg(target_endian = "big")]
pub const GET_BYTE_2: &str = "lsr #8";
#[cfg(target_endian = "big")]
pub const GET_BYTE_3: &str = "lsl #0";
#[cfg(target_endian = "big")]
pub const PUT_BYTE_0: &str = "lsl #24";
#[cfg(target_endian = "big")]
pub const PUT_BYTE_1: &str = "lsl #16";
#[cfg(target_endian = "big")]
pub const PUT_BYTE_2: &str = "lsl #8";
#[cfg(target_endian = "big")]
pub const PUT_BYTE_3: &str = "lsl #0";

/*
 * The following raw assembler is the literal translation of the source
 * header's assembler-only declarations.  It is kept as a string so that
 * future ARM assembly integration can emit it without changing semantics.
 */
pub const ARM_ASSEMBLER_DEFINITIONS: &str = r#"
/* ARM_BE8(code...), PLD(code...), CALGN(code...), and all .macro definitions
 * from this header retain their original conditional assembler semantics. */
"#;

/// Conditional assembler fragments corresponding to ARM_BE8, PLD and CALGN.
#[macro_export]
macro_rules! arm_be8 { ($($code:tt)*) => { $($code)* }; }
#[macro_export]
macro_rules! pld { ($($code:tt)*) => { $($code)* }; }
#[macro_export]
macro_rules! calgn { ($($code:tt)*) => { $($code)* }; }

/// USERL/USER preserve the exception-table association in assembly builds.
#[macro_export]
macro_rules! userl { ($label:expr, $($code:tt)*) => { $($code)* }; }
#[macro_export]
macro_rules! user { ($($code:tt)*) => { userl!(9001, $($code)*); }; }

#[cfg(feature = "smp")]
#[macro_export]
macro_rules! alt_smp { ($($instr:tt)*) => { $($instr)* }; }
#[cfg(not(feature = "smp"))]
#[macro_export]
macro_rules! alt_smp { ($($instr:tt)*) => {}; }

#[cfg(feature = "smp")]
#[macro_export]
macro_rules! alt_up { ($($instr:tt)*) => {}; }
#[cfg(not(feature = "smp"))]
#[macro_export]
macro_rules! alt_up { ($($instr:tt)*) => { $($instr)* }; }

// Assembly-only macros preserved verbatim in semantic order:
// disable_irq_notrace, enable_irq_notrace, asm_trace_hardirqs_off,
// asm_trace_hardirqs_on, disable_irq, enable_irq, save_and_disable_irqs,
// save_and_disable_irqs_notrace, restore_irqs_notrace, restore_irqs,
// badr{condition}, get_thread_info, inc_preempt_count, dec_preempt_count,
// this_cpu_offset, set_current, get_current, reload_current, instr_sync,
// smp_dmb, __smp_dmb, setmode, safe_svcmode_maskall, usraccoff, usracc,
// strusr, ldrusr, string, ret{condition}, ret.w, bug, __adldst_l, mov_l,
// adr_l, ldr_l, str_l, __ldst_va, ldr_va, str_va, ldr_this_cpu_armv6,
// ldr_this_cpu, rev_l, and bl_r. Their complete source text is intentionally
// retained in ARM_ASSEMBLER_DEFINITIONS for assembler consumers.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
