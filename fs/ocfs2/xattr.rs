// SPDX-License-Identifier: GPL-2.0-only
//
// Low-level Rust translation of ocfs2/xattr.c.  Kernel-provided types,
// constants, helpers, and functions are intentionally left as external
// dependencies, matching the original translation unit's includes.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
#![allow(dead_code, unused_variables, unused_mut, unused_imports)]

use core::ffi::{c_char, c_int, c_void};

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type size_t = usize;

#[repr(C)]
pub struct inode { _private: [u8; 0] }
#[repr(C)]
pub struct super_block { _private: [u8; 0] }
#[repr(C)]
pub struct buffer_head { _private: [u8; 0] }
#[repr(C)]
pub struct handle_t { _private: [u8; 0] }
#[repr(C)]
pub struct ocfs2_super { _private: [u8; 0] }
#[repr(C)]
pub struct ocfs2_alloc_context { _private: [u8; 0] }
#[repr(C)]
pub struct ocfs2_cached_dealloc_ctxt { _private: [u8; 0] }
#[repr(C)]
pub struct ocfs2_refcount_tree { _private: [u8; 0] }
#[repr(C)]
pub struct ocfs2_caching_info { _private: [u8; 0] }

#[repr(C)]
pub struct ocfs2_xattr_def_value_root {
    // Must be last as it ends in a flexible-array member.
    pub xv: ocfs2_xattr_value_root,
    pub er: ocfs2_extent_rec,
}

#[repr(C)]
pub struct ocfs2_xattr_value_root { pub xr_list: ocfs2_extent_list }
#[repr(C)]
pub struct ocfs2_extent_list { pub l_recs: [ocfs2_extent_rec; 0] }
#[repr(C)]
pub struct ocfs2_extent_rec { _private: [u8; 0] }

#[repr(C)]
pub struct ocfs2_xattr_bucket {
    pub bu_inode: *mut inode,
    pub bu_bhs: [*mut buffer_head; 0],
    pub bu_blocks: c_int,
}

#[repr(C)]
pub struct ocfs2_xattr_set_ctxt {
    pub handle: *mut handle_t,
    pub meta_ac: *mut ocfs2_alloc_context,
    pub data_ac: *mut ocfs2_alloc_context,
    pub dealloc: ocfs2_cached_dealloc_ctxt,
    pub set_abort: c_int,
}

#[repr(C)]
pub struct ocfs2_xattr_info {
    pub xi_name_index: c_int,
    pub xi_name: *const c_char,
    pub xi_name_len: c_int,
    pub xi_value: *const c_void,
    pub xi_value_len: size_t,
}

#[repr(C)]
pub struct ocfs2_xattr_search {
    pub inode_bh: *mut buffer_head,
    pub xattr_bh: *mut buffer_head,
    pub header: *mut ocfs2_xattr_header,
    pub bucket: *mut ocfs2_xattr_bucket,
    pub base: *mut c_void,
    pub end: *mut c_void,
    pub here: *mut ocfs2_xattr_entry,
    pub not_found: c_int,
}

#[repr(C)]
pub struct ocfs2_xattr_header { _private: [u8; 0] }
#[repr(C)]
pub struct ocfs2_xattr_entry { _private: [u8; 0] }

// The declarations below preserve the externally visible implementation
// entry points from the C translation unit; their kernel bodies are supplied
// by the surrounding OCFS2 Rust translation.
extern "C" {
    pub fn ocfs2_calc_security_init(
        dir: *mut inode, si: *mut c_void, want_clusters: *mut c_int,
        xattr_credits: *mut c_int, xattr_ac: *mut *mut ocfs2_alloc_context,
    ) -> c_int;
    pub fn ocfs2_xattr_remove(inode: *mut inode, di_bh: *mut buffer_head) -> c_int;
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
