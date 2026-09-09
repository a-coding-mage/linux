/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2012 Regents of the University of California
 */

// C dependencies: linux/compiler.h, linux/const.h, linux/types.h, asm/asm.h,
// and asm-generic/bug.h provide the corresponding compiler helpers, integer
// types, assembly strings, and bug definitions.

pub const __INSN_LENGTH_MASK: usize = 0x3;
pub const __INSN_LENGTH_32: usize = 0x3;
pub const __COMPRESSED_INSN_MASK: usize = 0xffff;

pub const __BUG_INSN_32: usize = 0x00100073; /* ebreak */
pub const __BUG_INSN_16: usize = 0x9002; /* c.ebreak */

#[inline]
pub fn GET_INSN_LENGTH(insn: usize) -> usize {
    if (insn & __INSN_LENGTH_MASK) == __INSN_LENGTH_32 {
        4usize
    } else {
        2usize
    }
}

pub type bug_insn_t = u32;

// Build-time condition preserved from CONFIG_GENERIC_BUG_RELATIVE_POINTERS:
// relative pointers use "RISCV_INT 1b - ." and "RISCV_INT <file> - .";
// otherwise the corresponding RISCV_PTR forms are used.

// Build-time condition preserved from CONFIG_DEBUG_BUGVERBOSE: verbose bug
// entries include the file and line fields; otherwise they include flags only.

// Build-time condition preserved from CONFIG_GENERIC_BUG. The C macros below
// emit an ebreak instruction and, for generic bugs, populate __bug_table.
// The exact inline-assembly expansion depends on the external RISCV_* and
// WARN_CONDITION_STR definitions.

#[macro_export]
macro_rules! __BUG_FLAGS {
    ($cond_str:expr, $flags:expr) => {{
        // Corresponds to volatile inline assembly in the C header:
        // ARCH_WARN_ASM("%0", "%1", "%2", "%3") with the bug metadata.
        unsafe { core::arch::asm!("ebreak", options(nostack, preserves_flags)); }
        let _ = ($cond_str, $flags);
    }};
}

#[macro_export]
macro_rules! BUG {
    () => {{
        $crate::__BUG_FLAGS!("", 0);
        unsafe { core::hint::unreachable_unchecked() }
    }};
}

#[macro_export]
macro_rules! __WARN_FLAGS {
    ($cond_str:expr, $flags:expr) => {
        $crate::__BUG_FLAGS!($cond_str, BUGFLAG_WARNING | ($flags))
    };
}

// Corresponds to ARCH_WARN_REACHABLE.
// Corresponds to HAVE_ARCH_BUG.

#[repr(C)]
pub struct pt_regs;

#[repr(C)]
pub struct task_struct;

extern "C" {
    pub fn __show_regs(regs: *mut pt_regs);
    pub fn die(regs: *mut pt_regs, str_: *const core::ffi::c_char);
    pub fn do_trap(regs: *mut pt_regs, signo: i32, code: i32, addr: usize);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
