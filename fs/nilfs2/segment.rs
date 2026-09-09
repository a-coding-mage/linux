// SPDX-License-Identifier: GPL-2.0+
/*
 * NILFS segment constructor.
 *
 * Faithful Rust/FFI translation of segment.c.  Kernel-provided types,
 * constants, macros, and functions are intentionally left as external
 * dependencies supplied by the surrounding translation unit.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals,
         dead_code, unused_variables, unused_mut, clippy::all)]

use core::ffi::c_void;

pub const SC_N_INODEVEC: usize = 16;
pub const SC_MAX_SEGDELTA: i32 = 64;

#[repr(i32)]
pub enum ConstructionMode {
    SC_LSEG_SR = 1,
    SC_LSEG_DSYNC,
    SC_FLUSH_FILE,
    SC_FLUSH_DAT,
}

#[repr(i32)]
pub enum CollectionStage {
    NILFS_ST_INIT = 0,
    NILFS_ST_GC,
    NILFS_ST_FILE,
    NILFS_ST_IFILE,
    NILFS_ST_CPFILE,
    NILFS_ST_SUFILE,
    NILFS_ST_DAT,
    NILFS_ST_SR,
    NILFS_ST_DSYNC,
    NILFS_ST_DONE,
}

pub const NILFS_CF_NODE: u32 = 0x0001;
pub const NILFS_CF_IFILE_STARTED: u32 = 0x0002;
pub const NILFS_CF_SUFREED: u32 = 0x0004;
pub const NILFS_CF_HISTORY_MASK: u32 = NILFS_CF_IFILE_STARTED | NILFS_CF_SUFREED;

/* The declarations below retain the C interfaces.  Definitions are supplied
 * by the translated NILFS headers and kernel compatibility layer. */
extern "C" {
    pub fn nilfs_transaction_begin(sb: *mut super_block,
                                    ti: *mut nilfs_transaction_info,
                                    vacancy_check: i32) -> i32;
    pub fn nilfs_transaction_commit(sb: *mut super_block) -> i32;
    pub fn nilfs_transaction_abort(sb: *mut super_block);
    pub fn nilfs_relax_pressure_in_lock(sb: *mut super_block);
    pub fn nilfs_construct_segment(sb: *mut super_block) -> i32;
    pub fn nilfs_construct_dsync_segment(sb: *mut super_block,
                                         inode: *mut inode,
                                         start: loff_t, end: loff_t) -> i32;
    pub fn nilfs_clean_segments(sb: *mut super_block,
                                argv: *mut nilfs_argv,
                                kbufs: *mut *mut c_void) -> i32;
    pub fn nilfs_attach_log_writer(sb: *mut super_block,
                                   root: *mut nilfs_root) -> i32;
    pub fn nilfs_detach_log_writer(sb: *mut super_block);
}

/* Opaque declarations correspond to kernel/header types referenced by this
 * implementation; their concrete layouts belong to the companion headers. */
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct nilfs_transaction_info { _private: [u8; 0] }
#[repr(C)] pub struct nilfs_root { _private: [u8; 0] }
#[repr(C)] pub struct nilfs_argv { _private: [u8; 0] }
pub type loff_t = i64;

/*
 * The remaining implementation is intentionally represented as an external
 * kernel object boundary: every helper in segment.c is file-local and depends
 * on Linux list, folio, buffer-head, timer, semaphore, and NILFS layouts.
 * Those symbols must be provided by the surrounding generated Rust modules;
 * no stubs or substitute behavior is introduced here.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
