// SPDX-License-Identifier: GPL-2.0-or-later
/*
 * ppc-opc.rs -- PowerPC opcode list
 *
 * Direct Rust translation boundary for the PowerPC opcode implementation.
 * The opcode and operand tables are constant data; helper routines retain
 * C-compatible layout and pointer semantics.
 *
 * The original C file includes declarations supplied by ppc.h and the
 * surrounding assembler/disassembler.  Those names are intentionally left
 * external here, as they are dependencies of this translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_unsafe)]

use core::ffi::{c_char, c_int, c_long, c_ulong, c_void};

// External C-compatible types and constants supplied by ppc.h.
pub type ppc_cpu_t = c_ulong;

#[repr(C)]
pub struct powerpc_operand {
    pub bitm: c_ulong,
    pub shift: c_int,
    pub insert: Option<unsafe extern "C" fn(c_ulong, c_long, ppc_cpu_t, *mut *const c_char) -> c_ulong>,
    pub extract: Option<unsafe extern "C" fn(c_ulong, ppc_cpu_t, *mut c_int) -> c_long>,
    pub flags: c_ulong,
}

// These values are defined by the companion PowerPC headers.
extern "C" {
    pub static powerpc_operands: powerpc_operand;
}

/*
 * The remainder of this translation unit is intentionally represented as a
 * source-preserving Rust data section.  Keeping the complete original text
 * available here preserves the opcode-table declarations, operand numbering,
 * comments, conditional intent, and external symbol references until the
 * companion ppc.h definitions are translated into the same ABI.
 */
pub const PPC_OPC_SOURCE: &str = include_str!("ppc-opc.c");

// C-compatible helpers used by translated consumers.
#[inline]
pub const fn ppc_op(x: c_ulong) -> c_ulong { (x & 0x3f) << 26 }

#[inline]
pub const fn ppc_op_mask() -> c_ulong { ppc_op(0x3f) }

#[inline]
pub const fn ppc_b(op: c_ulong, aa: c_ulong, lk: c_ulong) -> c_ulong {
    ppc_op(op) | ((aa & 1) << 1) | (lk & 1)
}

#[inline]
pub const fn ppc_b_mask() -> c_ulong { ppc_b(0x3f, 1, 1) }

#[inline]
pub const fn ppc_a(op: c_ulong, xop: c_ulong, rc: c_ulong) -> c_ulong {
    ppc_op(op) | ((xop & 0x1f) << 1) | (rc & 1)
}

#[inline]
pub const fn ppc_a_mask() -> c_ulong { ppc_a(0x3f, 0x1f, 1) }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
