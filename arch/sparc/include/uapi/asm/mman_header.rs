/* SPDX-License-Identifier: GPL-2.0 WITH Linux-syscall-note */
/*
 * Original header guard: _UAPI__SPARC_MMAN_H__
 * Dependency: <asm-generic/mman-common.h>
 */

/* SunOS'ified... */

pub const PROT_ADI: i32 = 0x10; /* ADI enabled */

pub const MAP_RENAME: i32 = MAP_ANONYMOUS; /* In SunOS terminology */
pub const MAP_NORESERVE: i32 = 0x40; /* don't reserve swap pages */
pub const MAP_INHERIT: i32 = 0x80; /* SunOS doesn't do this, but... */
pub const MAP_LOCKED: i32 = 0x100; /* lock the mapping */
pub const _MAP_NEW: u32 = 0x80000000; /* Binary compatibility is fun... */

pub const MAP_GROWSDOWN: i32 = 0x0200; /* stack-like segment */
pub const MAP_DENYWRITE: i32 = 0x0800; /* ETXTBSY */
pub const MAP_EXECUTABLE: i32 = 0x1000; /* mark it as an executable */

pub const MCL_CURRENT: i32 = 0x2000; /* lock all currently mapped pages */
pub const MCL_FUTURE: i32 = 0x4000; /* lock all additions to address space */
pub const MCL_ONFAULT: i32 = 0x8000; /* lock all pages that are faulted in */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
