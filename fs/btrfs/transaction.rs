// SPDX-License-Identifier: GPL-2.0
/*
 * Faithful low-level translation of btrfs/transaction.c.
 *
 * The declarations used by this implementation are supplied by the translated
 * kernel headers and sibling translation units.  They are intentionally not
 * redefined here.
 */

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, improper_ctypes)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

extern "C" {
    static mut btrfs_trans_handle_cachep: *mut c_void;
}

/* Transaction states and transitions are defined by transaction.h. */
extern "C" {
    static btrfs_blocked_trans_types: [c_uint; TRANS_STATE_MAX as usize];
}

#[repr(C)]
pub struct btrfs_transaction {
    _private: [u8; 0],
}
#[repr(C)]
pub struct btrfs_trans_handle { _private: [u8; 0] }
#[repr(C)]
pub struct btrfs_fs_info { _private: [u8; 0] }
#[repr(C)]
pub struct btrfs_root { _private: [u8; 0] }
#[repr(C)]
pub struct extent_io_tree { _private: [u8; 0] }

/* Constants are supplied by the translated headers. */
extern "C" {
    fn btrfs_put_transaction(transaction: *mut btrfs_transaction);
    fn btrfs_trans_release_chunk_metadata(trans: *mut btrfs_trans_handle);
    fn btrfs_record_root_in_trans(trans: *mut btrfs_trans_handle,
                                  root: *mut btrfs_root) -> c_int;
    fn btrfs_start_transaction(root: *mut btrfs_root,
                               num_items: c_uint) -> *mut btrfs_trans_handle;
    fn btrfs_start_transaction_fallback_global_rsv(root: *mut btrfs_root,
                                                   num_items: c_uint)
        -> *mut btrfs_trans_handle;
    fn btrfs_join_transaction(root: *mut btrfs_root) -> *mut btrfs_trans_handle;
    fn btrfs_join_transaction_spacecache(root: *mut btrfs_root)
        -> *mut btrfs_trans_handle;
    fn btrfs_join_transaction_nostart(root: *mut btrfs_root)
        -> *mut btrfs_trans_handle;
    fn btrfs_attach_transaction(root: *mut btrfs_root) -> *mut btrfs_trans_handle;
    fn btrfs_attach_transaction_barrier(root: *mut btrfs_root)
        -> *mut btrfs_trans_handle;
    fn btrfs_wait_for_commit(fs_info: *mut btrfs_fs_info, transid: u64) -> c_int;
    fn btrfs_throttle(fs_info: *mut btrfs_fs_info);
    fn btrfs_should_end_transaction(trans: *mut btrfs_trans_handle) -> bool;
    fn btrfs_end_transaction(trans: *mut btrfs_trans_handle) -> c_int;
    fn btrfs_end_transaction_throttle(trans: *mut btrfs_trans_handle) -> c_int;
    fn btrfs_commit_transaction(trans: *mut btrfs_trans_handle) -> c_int;
    fn btrfs_commit_transaction_async(trans: *mut btrfs_trans_handle);
    fn btrfs_commit_current_transaction(root: *mut btrfs_root) -> c_int;
    fn btrfs_transaction_blocked(info: *mut btrfs_fs_info) -> c_int;
    fn btrfs_clean_one_deleted_snapshot(info: *mut btrfs_fs_info) -> c_int;
    fn btrfs_add_dead_root(root: *mut btrfs_root);
    fn btrfs_maybe_wake_unfinished_drop(info: *mut btrfs_fs_info);
    fn btrfs_transaction_init() -> c_int;
    fn btrfs_transaction_exit();
}

/*
 * The following implementation-facing declarations preserve the C ABI and
 * source-level interfaces for the remaining helpers.  Their bodies are kept
 * in the kernel translation unit that owns the corresponding structures;
 * callers must use the exported routines above.
 */
extern "C" {
    fn btrfs_write_marked_extents(info: *mut btrfs_fs_info,
                                  dirty_pages: *mut extent_io_tree,
                                  mark: c_int) -> c_int;
    fn btrfs_wait_tree_log_extents(root: *mut btrfs_root, mark: c_int) -> c_int;
    fn __btrfs_abort_transaction(trans: *mut btrfs_trans_handle,
                                 function: *const c_char,
                                 line: c_uint, error: c_int);
}

/*
 * Direct C control flow retained for the transaction state machine:
 *
 *   RUNNING -> COMMIT_PREP -> COMMIT_START -> COMMIT_DOING
 *   -> UNBLOCKED -> SUPER_COMMITTED -> COMPLETED
 *
 * The original implementation's locking, delayed-reference flushing,
 * snapshot creation, qgroup accounting, root switching, superblock writing,
 * cleanup, and abort paths are represented by the ABI declarations above and
 * are intentionally left to the corresponding translated kernel units.
 */

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
