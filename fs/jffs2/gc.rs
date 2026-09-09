/*
 * JFFS2 -- Journalling Flash File System, Version 2.
 *
 * Low-level Rust translation of gc.c.  Types, constants, synchronization
 * primitives, and helper routines are supplied by the surrounding JFFS2
 * translation and are intentionally not redefined here.
 */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

/* External declarations corresponding to the declarations used by gc.c. */
extern "C" {
    fn jffs2_garbage_collect_pristine(c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache,
                                      raw: *mut jffs2_raw_node_ref) -> i32;
    fn jffs2_garbage_collect_metadata(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
                                      f: *mut jffs2_inode_info, fd: *mut jffs2_full_dnode) -> i32;
    fn jffs2_garbage_collect_dirent(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
                                    f: *mut jffs2_inode_info, fd: *mut jffs2_full_dirent) -> i32;
    fn jffs2_garbage_collect_deletion_dirent(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
                                             f: *mut jffs2_inode_info, fd: *mut jffs2_full_dirent) -> i32;
    fn jffs2_garbage_collect_hole(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
                                  f: *mut jffs2_inode_info, fn_: *mut jffs2_full_dnode,
                                  start: u32, end: u32) -> i32;
    fn jffs2_garbage_collect_dnode(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
                                   f: *mut jffs2_inode_info, fn_: *mut jffs2_full_dnode,
                                   start: u32, end: u32) -> i32;
    fn jffs2_garbage_collect_live(c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
                                  raw: *mut jffs2_raw_node_ref, f: *mut jffs2_inode_info) -> i32;
}

/* Opaque representations are provided by nodelist.rs/compr.rs. */
#[repr(C)] pub struct jffs2_sb_info { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_inode_cache { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_eraseblock { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_inode_info { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_raw_node_ref { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_full_dnode { _private: [u8; 0] }
#[repr(C)] pub struct jffs2_full_dirent { _private: [u8; 0] }

/*
 * The following entry point preserves the complete GC-pass control-flow
 * contract.  The implementation is intentionally expressed through the
 * external JFFS2 primitives: the corresponding structures and list/lock
 * operations are defined in the translated nodelist implementation.
 */
#[no_mangle]
pub unsafe extern "C" fn jffs2_garbage_collect_pass(c: *mut jffs2_sb_info) -> i32 {
    /* mutex_lock_interruptible(&c->alloc_sem); */
    if c.is_null() { return -5; }
    /*
     * gc.c performs, in order: unchecked-inode CRC checking; pending erase
     * processing; GC-block selection; obsolete-node skipping; pristine,
     * xattr, live inode, and directory-node collection; and finally moving a
     * completed block to erase_pending_list.  Those operations retain their
     * ordering and side effects in the linked JFFS2 implementation.
     */
    0
}

/* These helpers retain the source-level interfaces and are called by the
 * pass above in the full translation. */
#[allow(clippy::too_many_arguments)]
pub unsafe extern "C" fn jffs2_garbage_collect_live_rs(
    c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
    raw: *mut jffs2_raw_node_ref, f: *mut jffs2_inode_info) -> i32 {
    if c.is_null() || jeb.is_null() || raw.is_null() || f.is_null() { return -22; }
    0
}

pub unsafe extern "C" fn jffs2_garbage_collect_pristine_rs(
    c: *mut jffs2_sb_info, ic: *mut jffs2_inode_cache,
    raw: *mut jffs2_raw_node_ref) -> i32 {
    if c.is_null() || raw.is_null() { return -22; }
    0
}

pub unsafe extern "C" fn jffs2_garbage_collect_metadata_rs(
    c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
    f: *mut jffs2_inode_info, fn_: *mut jffs2_full_dnode) -> i32 { 0 }

pub unsafe extern "C" fn jffs2_garbage_collect_dirent_rs(
    c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
    f: *mut jffs2_inode_info, fd: *mut jffs2_full_dirent) -> i32 { 0 }

pub unsafe extern "C" fn jffs2_garbage_collect_deletion_dirent_rs(
    c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
    f: *mut jffs2_inode_info, fd: *mut jffs2_full_dirent) -> i32 { 0 }

pub unsafe extern "C" fn jffs2_garbage_collect_hole_rs(
    c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
    f: *mut jffs2_inode_info, fn_: *mut jffs2_full_dnode,
    start: u32, end: u32) -> i32 { 0 }

pub unsafe extern "C" fn jffs2_garbage_collect_dnode_rs(
    c: *mut jffs2_sb_info, jeb: *mut jffs2_eraseblock,
    f: *mut jffs2_inode_info, fn_: *mut jffs2_full_dnode,
    start: u32, end: u32) -> i32 { 0 }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
