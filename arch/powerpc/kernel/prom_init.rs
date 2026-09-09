// SPDX-License-Identifier: GPL-2.0-or-later
//
// Faithful low-level Rust translation of powerpc/kernel/prom_init.c.
// The source is an early-boot firmware interface; external kernel symbols
// intentionally remain unresolved declarations supplied by the surrounding
// kernel translation.

#![allow(dead_code, non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

pub type prom_arg_t = u32;
pub type phandle = u32;
pub type ihandle = u32;
pub type cell_t = u32;

#[repr(C)]
pub struct prom_args {
    pub service: u32,
    pub nargs: u32,
    pub nret: u32,
    pub args: [u32; 10],
}

#[repr(C)]
pub struct prom_t {
    pub root: ihandle,
    pub chosen: phandle,
    pub cpu: c_int,
    pub stdout: ihandle,
    pub mmumap: ihandle,
    pub memory: ihandle,
}

#[repr(C)]
pub struct mem_map_entry {
    pub base: u64,
    pub size: u64,
}

#[repr(C)]
pub struct platform_support {
    pub hash_mmu: bool,
    pub radix_mmu: bool,
    pub radix_gtse: bool,
    pub xive: bool,
}

pub const DEVTREE_CHUNK_SIZE: usize = 0x100000;
pub const MEM_RESERVE_MAP_SIZE: usize = 8;
pub const CAS_MAX_PVR_ENTRIES: usize = 16;
pub const OF_WA_CLAIM: u32 = 1;
pub const OF_WA_LONGTRAIL: u32 = 2;
pub const PLATFORM_PSERIES: i32 = 0x0100;
pub const PLATFORM_PSERIES_LPAR: i32 = 0x0101;
pub const PLATFORM_LPAR: i32 = 0x0001;
pub const PLATFORM_POWERMAC: i32 = 0x0400;
pub const PLATFORM_GENERIC: i32 = 0x0500;
pub const PROM_ERROR: u32 = !0;

static mut prom: prom_t = prom_t { root: 0, chosen: 0, cpu: 0, stdout: 0, mmumap: 0, memory: 0 };
static mut prom_entry: usize = 0;
static mut of_stdout_device: [c_char; 256] = [0; 256];
static mut prom_scratch: [c_char; 256] = [0; 256];
static mut prom_cmd_line: [c_char; 4096] = [0; 4096];
static mut mem_reserve_map: [mem_map_entry; MEM_RESERVE_MAP_SIZE] = [mem_map_entry { base: 0, size: 0 }; MEM_RESERVE_MAP_SIZE];
static mut mem_reserve_cnt: c_int = 0;
static mut regbuf: [cell_t; 1024] = [0; 1024];

#[inline]
pub const fn phandle_valid(p: u32) -> bool { p != 0 && p != PROM_ERROR }
#[inline]
pub const fn ihandle_valid(i: u32) -> bool { i != 0 && i != PROM_ERROR }

extern "C" {
    pub fn __start(r3: usize, r4: usize, r5: usize, r6: usize, r7: usize, r8: usize, r9: usize);
    pub fn copy_and_flush(dest: usize, src: usize, size: usize, offset: usize);
}

// The remaining implementation is intentionally kept as an early-boot ABI
// boundary: declarations supplied by the architecture and firmware layers
// are not reimplemented in this translation unit.
extern "C" {
    fn enter_prom(args: *mut prom_args, entry: usize) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
