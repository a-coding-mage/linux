/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

/* Depends on asm-generic/mman-common-tools.h. */

pub const MAP_GROWSDOWN: u32 = 0x0100; /* stack-like segment */
pub const MAP_DENYWRITE: u32 = 0x0800; /* ETXTBSY */
pub const MAP_EXECUTABLE: u32 = 0x1000; /* mark it as an executable */
pub const MAP_LOCKED: u32 = 0x2000; /* pages are locked */
pub const MAP_NORESERVE: u32 = 0x4000; /* don't check for reservations */

/*
 * Bits [26:31] are reserved, see asm-generic/hugetlb_encode.h
 * for MAP_HUGETLB usage
 */

pub const MCL_CURRENT: u32 = 1; /* lock all current mappings */
pub const MCL_FUTURE: u32 = 2; /* lock all future mappings */
pub const MCL_ONFAULT: u32 = 4; /* lock all pages that are faulted in */

pub const SHADOW_STACK_SET_TOKEN: u64 = 1u64 << 0; /* Set up a restore token in the shadow stack */
pub const SHADOW_STACK_SET_MARKER: u64 = 1u64 << 1; /* Set up a top of stack marker in the shadow stack */

// SOURCE-COMMIT: 08dbfad3f5040f5bdb6c529da20d6d4e81fefd72
