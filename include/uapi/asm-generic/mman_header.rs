/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */

// Dependency: declarations from <asm-generic/mman-common.h> are supplied externally.

pub const MAP_GROWSDOWN: i32 = 0x0100; // stack-like segment
pub const MAP_DENYWRITE: i32 = 0x0800; // ETXTBSY
pub const MAP_EXECUTABLE: i32 = 0x1000; // mark it as an executable
pub const MAP_LOCKED: i32 = 0x2000; // pages are locked
pub const MAP_NORESERVE: i32 = 0x4000; // don't check for reservations

/*
 * Bits [26:31] are reserved, see asm-generic/hugetlb_encode.h
 * for MAP_HUGETLB usage
 */

pub const MCL_CURRENT: i32 = 1; // lock all current mappings
pub const MCL_FUTURE: i32 = 2; // lock all future mappings
pub const MCL_ONFAULT: i32 = 4; // lock all pages that are faulted in

pub const SHADOW_STACK_SET_TOKEN: u64 = 1u64 << 0; // Set up a restore token in the shadow stack
pub const SHADOW_STACK_SET_MARKER: u64 = 1u64 << 1; // Set up a top of stack marker in the shadow stack

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
