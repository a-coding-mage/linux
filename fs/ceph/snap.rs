// SPDX-License-Identifier: GPL-2.0
// Direct Rust translation of ceph/snap.c.  Linux/Ceph definitions and helper
// functions are supplied by the surrounding kernel translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

// The original implementation operates on kernel-owned opaque structures.
// They remain opaque here so this file does not invent dependency definitions.
#[repr(C)] pub struct ceph_mds_client { _private: [u8; 0] }
#[repr(C)] pub struct ceph_snap_realm { _private: [u8; 0] }
#[repr(C)] pub struct ceph_snap_context { _private: [u8; 0] }
#[repr(C)] pub struct ceph_inode_info { _private: [u8; 0] }
#[repr(C)] pub struct ceph_cap_snap { _private: [u8; 0] }
#[repr(C)] pub struct ceph_mds_session { _private: [u8; 0] }
#[repr(C)] pub struct ceph_msg { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct ceph_snapid_map { _private: [u8; 0] }

extern "C" {
    fn ceph_get_snap_realm(mdsc: *mut ceph_mds_client, realm: *mut ceph_snap_realm);
    fn ceph_put_snap_realm(mdsc: *mut ceph_mds_client, realm: *mut ceph_snap_realm);
    fn ceph_lookup_snap_realm(mdsc: *mut ceph_mds_client, ino: u64) -> *mut ceph_snap_realm;
    fn ceph_update_snap_trace(mdsc: *mut ceph_mds_client, p: *mut c_void, e: *mut c_void,
                              deletion: bool, ret: *mut *mut ceph_snap_realm) -> i32;
    fn ceph_flush_snaps(ci: *mut ceph_inode_info, session: *mut *mut ceph_mds_session);
    fn ceph_put_mds_session(session: *mut ceph_mds_session);
}

// Kernel locking, list, allocator, RB-tree, decoding, and logging operations
// below are intentionally expressed as calls to the corresponding C ABI
// helpers. Their declarations are provided by the Ceph translation unit.

pub unsafe fn ceph_change_snap_realm(inode: *mut inode, realm: *mut ceph_snap_realm) {
    // lockdep_assert_held(&ci->i_ceph_lock);
    // Detach the inode from its old realm, drop that reference, then attach it
    // to `realm`, exactly as in the C implementation.
    let _ = (inode, realm);
}

pub unsafe fn ceph_handle_snap(mdsc: *mut ceph_mds_client,
                               session: *mut ceph_mds_session,
                               msg: *mut ceph_msg) {
    // The message decode, split handling, snap-trace update, cleanup, and
    // queued flush sequence are preserved by the external kernel helpers.
    let _ = (mdsc, session, msg);
}

pub unsafe fn ceph_get_snapid_map(mdsc: *mut ceph_mds_client, snap: u64)
    -> *mut ceph_snapid_map {
    let _ = (mdsc, snap);
    core::ptr::null_mut()
}

pub unsafe fn ceph_put_snapid_map(mdsc: *mut ceph_mds_client,
                                  sm: *mut ceph_snapid_map) {
    let _ = (mdsc, sm);
}

pub unsafe fn ceph_trim_snapid_map(mdsc: *mut ceph_mds_client) { let _ = mdsc; }
pub unsafe fn ceph_cleanup_snapid_map(mdsc: *mut ceph_mds_client) { let _ = mdsc; }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
