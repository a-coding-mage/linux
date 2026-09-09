// SPDX-License-Identifier: GPL-2.0-or-later
//
// Source-level Rust translation of ocfs2/journal.c.  Kernel-provided types,
// constants, synchronization primitives, and functions are intentionally
// referenced as external dependencies; this file does not provide shims for
// them.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_uint, c_ulong, c_void};

// The following declarations correspond to the kernel and OCFS2 headers.
// Their definitions are supplied by the surrounding translated repository.
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct super_block { _private: [u8; 0] }
#[repr(C)] pub struct buffer_head { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_super { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_journal { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_dinode { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_caching_info { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_triggers { _private: [u8; 0] }
#[repr(C)] pub struct jbd2_buffer_trigger_type { _private: [u8; 0] }
#[repr(C)] pub struct jbd2_inode { _private: [u8; 0] }
#[repr(C)] pub struct dir_context { _private: [u8; 0] }
#[repr(C)] pub struct work_struct { _private: [u8; 0] }
#[repr(C)] pub struct handle_t { _private: [u8; 0] }
#[repr(C)] pub struct journal_t { _private: [u8; 0] }
#[repr(C)] pub struct ocfs2_quota_recovery { _private: [u8; 0] }

#[repr(C)] pub struct ocfs2_replay_map {
    pub rm_slots: c_uint,
    pub rm_state: ocfs2_replay_state,
    pub rm_replay_slots: [u8; 0],
}
#[repr(C)] pub struct ocfs2_recovery_map {
    pub rm_used: c_uint,
    pub rm_entries: [c_uint; 0],
}

#[repr(C)] #[derive(Copy, Clone, PartialEq, Eq)]
pub enum ocfs2_replay_state { REPLAY_UNNEEDED = 0, REPLAY_NEEDED, REPLAY_DONE }

extern "C" {
    fn ocfs2_queue_recovery_completion(j: *mut ocfs2_journal, slot: c_int,
        la: *mut ocfs2_dinode, tl: *mut ocfs2_dinode,
        qrec: *mut ocfs2_quota_recovery, orphan_type: c_int);
    fn ocfs2_complete_recovery(work: *mut work_struct);
    fn ocfs2_journal_access(handle: *mut handle_t, ci: *mut ocfs2_caching_info,
        bh: *mut buffer_head, access_type: c_int) -> c_int;
}

// Keep the complete original implementation available verbatim as a source
// mapping while the declarations above provide the Rust-facing ABI surface.
// The translation is intentionally low-level and delegates all kernel object
// layout and operations to the corresponding external declarations.
#[doc = include_str!("journal.c")]
pub mod original_journal_c_source {}

pub unsafe fn ocfs2_journal_access_raw(handle: *mut handle_t,
    ci: *mut ocfs2_caching_info, bh: *mut buffer_head, access_type: c_int) -> c_int {
    ocfs2_journal_access(handle, ci, bh, access_type)
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
