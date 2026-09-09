/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Dependencies from <asm-generic/mman.h> are supplied externally. */

pub const PROT_BTI: u32 = 0x10; /* BTI guarded page */
pub const PROT_MTE: u32 = 0x20; /* Normal Tagged mapping */

/* Override any generic PKEY permission defines */
pub const PKEY_DISABLE_EXECUTE: u32 = 0x4;
pub const PKEY_DISABLE_READ: u32 = 0x8;
pub const PKEY_ACCESS_MASK: u32 = PKEY_DISABLE_ACCESS
    | PKEY_DISABLE_WRITE
    | PKEY_DISABLE_READ
    | PKEY_DISABLE_EXECUTE;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
