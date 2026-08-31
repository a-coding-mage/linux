/* SPDX-License-Identifier: GPL-2.0-only */
/*
 * jitdump.h: jitted code info encapsulation file format
 *
 * Adapted from OProfile GPLv2 support jidump.h:
 * Copyright 2007 OProfile authors
 * Jens Wilke
 * Daniel Hansel
 * Copyright IBM Corporation 2007
 */

use core::ffi::c_char;

/* From <string.h>. */
unsafe extern "C" {
    fn strlen(s: *const c_char) -> usize;
}

/* JiTD */
pub const JITHEADER_MAGIC: u32 = 0x4A695444;
pub const JITHEADER_MAGIC_SW: u32 = 0x4454694A;

pub const fn PADDING_8ALIGNED(x: usize) -> usize {
    (((x).wrapping_add(7)) & 7) ^ 7
}

pub const fn ALIGN_8(x: usize) -> usize {
    ((x).wrapping_add(7)) & !7
}

pub const JITHEADER_VERSION: u32 = 1;

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum jitdump_flags_bits {
    JITDUMP_FLAGS_ARCH_TIMESTAMP_BIT = 0,
    JITDUMP_FLAGS_MAX_BIT = 1,
}

pub const JITDUMP_FLAGS_ARCH_TIMESTAMP: u64 =
    1u64 << (jitdump_flags_bits::JITDUMP_FLAGS_ARCH_TIMESTAMP_BIT as u32);

pub const JITDUMP_FLAGS_RESERVED: u64 = if (jitdump_flags_bits::JITDUMP_FLAGS_MAX_BIT as u32) < 64 {
    !((1u64 << (jitdump_flags_bits::JITDUMP_FLAGS_MAX_BIT as u32)) - 1)
} else {
    0
};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jitheader {
    pub magic: u32,      /* characters "jItD" */
    pub version: u32,    /* header version */
    pub total_size: u32, /* total size of header */
    pub elf_mach: u32,   /* elf mach target */
    pub pad1: u32,       /* reserved */
    pub pid: u32,        /* JIT process id */
    pub timestamp: u64,  /* timestamp */
    pub flags: u64,      /* flags */
}

#[repr(u32)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum jit_record_type {
    JIT_CODE_LOAD = 0,
    JIT_CODE_MOVE = 1,
    JIT_CODE_DEBUG_INFO = 2,
    JIT_CODE_CLOSE = 3,
    JIT_CODE_UNWINDING_INFO = 4,

    JIT_CODE_MAX = 5,
}

/* record prefix (mandatory in each record) */
#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_prefix {
    pub id: u32,
    pub total_size: u32,
    pub timestamp: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_code_load {
    pub p: jr_prefix,

    pub pid: u32,
    pub tid: u32,
    pub vma: u64,
    pub code_addr: u64,
    pub code_size: u64,
    pub code_index: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_code_close {
    pub p: jr_prefix,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_code_move {
    pub p: jr_prefix,

    pub pid: u32,
    pub tid: u32,
    pub vma: u64,
    pub old_code_addr: u64,
    pub new_code_addr: u64,
    pub code_size: u64,
    pub code_index: u64,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct debug_entry {
    pub addr: u64,
    pub lineno: i32,  /* source line number starting at 1 */
    pub discrim: i32, /* column discriminator, 0 is default */
    /*
     * Null terminated filename, \xff\0 if same as previous entry.
     * C flexible array member: const char name[].
     */
    pub name: [c_char; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_code_debug_info {
    pub p: jr_prefix,

    pub code_addr: u64,
    pub nr_entry: u64,
    /* C flexible array member: struct debug_entry entries[]. */
    pub entries: [debug_entry; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct jr_code_unwinding_info {
    pub p: jr_prefix,

    pub unwinding_size: u64,
    pub eh_frame_hdr_size: u64,
    pub mapped_size: u64,
    /* C flexible array member: const char unwinding_data[]. */
    pub unwinding_data: [c_char; 0],
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union jr_entry {
    pub info: jr_code_debug_info,
    pub close: jr_code_close,
    pub load: jr_code_load,
    pub move_: jr_code_move,
    pub prefix: jr_prefix,
    pub unwinding: jr_code_unwinding_info,
}

pub unsafe fn debug_entry_next(ent: *mut debug_entry) -> *mut debug_entry {
    let a = ent.add(1) as *mut u8;
    let l = unsafe { strlen((*ent).name.as_ptr()) } + 1;
    unsafe { a.add(l) as *mut debug_entry }
}

pub unsafe fn debug_entry_file(ent: *mut debug_entry) -> *mut c_char {
    unsafe { ent.add(1) as *mut c_char }
}
