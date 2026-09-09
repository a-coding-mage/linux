/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * Copyright (C) 2025 Chen Miao
 */

// C header guard: _ASM_OPENRISC_PATCHING_H
// Dependency: <linux/types.h> supplies the u32 type.

/// Write an instruction to the specified address.
///
/// This is an externally defined C function. The raw pointer and unsafe ABI
/// preserve the original C interface and pointer behavior.
unsafe extern "C" {
    pub fn patch_insn_write(addr: *mut core::ffi::c_void, insn: u32) -> i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
