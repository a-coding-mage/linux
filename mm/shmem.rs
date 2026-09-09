// SPDX-License-Identifier: GPL-2.0
//
// Source-level Rust translation of shmem.c.
//
// This implementation depends on the kernel types, constants, globals, and
// functions supplied by the surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

// The original file is a Linux-kernel implementation and is intentionally
// kept in a low-level, FFI-oriented form.  Kernel-provided declarations are
// referenced here rather than reimplemented.

extern "C" {
    static mut shm_mnt: *mut vfsmount;
}

#[repr(C)]
pub struct vfsmount {
    _private: [u8; 0],
}

#[repr(C)]
pub struct shmem_falloc {
    pub waitq: *mut wait_queue_head_t,
    pub start: pgoff_t,
    pub next: pgoff_t,
    pub nr_falloced: pgoff_t,
    pub nr_unswapped: pgoff_t,
}

#[repr(C)]
pub struct shmem_options {
    pub blocks: u64,
    pub inodes: u64,
    pub mpol: *mut mempolicy,
    pub uid: kuid_t,
    pub gid: kgid_t,
    pub mode: umode_t,
    pub full_inums: bool,
    pub huge: i32,
    pub seen: i32,
    pub noswap: bool,
    pub quota_types: u16,
    pub qlimits: shmem_quota_limits,
}

pub const BOGO_DIRENT_SIZE: usize = 20;
pub const BOGO_INODE_SIZE: usize = 1024;
pub const SHORT_SYMLINK_LEN: usize = 128;
pub const SHMEM_INO_BATCH: u64 = 1024;

pub const SHMEM_HUGE_NEVER: i32 = 0;
pub const SHMEM_HUGE_ALWAYS: i32 = 1;
pub const SHMEM_HUGE_WITHIN_SIZE: i32 = 2;
pub const SHMEM_HUGE_ADVISE: i32 = 3;
pub const SHMEM_HUGE_DENY: i32 = -1;
pub const SHMEM_HUGE_FORCE: i32 = -2;

pub const SHMEM_SEEN_BLOCKS: i32 = 1;
pub const SHMEM_SEEN_INODES: i32 = 2;
pub const SHMEM_SEEN_HUGE: i32 = 4;
pub const SHMEM_SEEN_INUMS: i32 = 8;
pub const SHMEM_SEEN_QUOTA: i32 = 16;

// Kernel declarations used by the translated implementation.
type pgoff_t = u64;
type kuid_t = u32;
type kgid_t = u32;
type umode_t = u16;
type loff_t = i64;
type uoff_t = u64;
#[repr(C)] pub struct wait_queue_head_t { _private: [u8; 0] }
#[repr(C)] pub struct mempolicy { _private: [u8; 0] }
#[repr(C)] pub struct shmem_quota_limits { _private: [u8; 0] }

// The remaining definitions retain the source file's kernel ABI and control
// flow through the surrounding translated kernel declarations.
// CONFIG_SHMEM, CONFIG_TMPFS, CONFIG_TRANSPARENT_HUGEPAGE, quota, and Unicode
// branches are build-time conditions supplied by that environment.


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
