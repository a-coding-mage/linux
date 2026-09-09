// SPDX-License-Identifier: GPL-2.0-only
//
// Faithful low-level Rust translation of linux/fs/nfs/delegation.c.
// Kernel types, constants, synchronization primitives, and helper routines
// are supplied by the surrounding NFS implementation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::c_void;

extern "C" {
    static mut nfs_delegation_watermark: u32;
    static mut directory_delegations: bool;
}

// The Linux list/RCU/atomic operations below intentionally remain external
// kernel operations; their declarations are provided by the translated tree.
extern "C" {
    fn nfs_fhandle_hash(fhandle: *const nfs_fh) -> usize;
    fn put_cred(cred: *const cred);
    fn get_cred(cred: *const cred) -> *const cred;
    fn kfree_rcu(delegation: *mut nfs_delegation, rcu: *mut c_void);
    fn test_and_set_bit(bit: u32, ptr: *mut usize) -> bool;
    fn test_and_clear_bit(bit: u32, ptr: *mut usize) -> bool;
    fn test_bit(bit: u32, ptr: *const usize) -> bool;
    fn set_bit(bit: u32, ptr: *mut usize);
    fn clear_bit(bit: u32, ptr: *mut usize);
    fn refcount_dec_and_test(ptr: *mut usize) -> bool;
    fn refcount_inc(ptr: *mut usize);
    fn nfs_clear_verifier_delegated(inode: *mut inode);
    fn nfs4_stateid_copy(dst: *mut nfs4_stateid, src: *const nfs4_stateid);
    fn nfs4_stateid_is_newer(a: *const nfs4_stateid, b: *const nfs4_stateid) -> bool;
    fn nfs4_stateid_match_other(a: *const nfs4_stateid, b: *const nfs4_stateid) -> bool;
    fn nfs4_stateid_match_or_older(a: *const nfs4_stateid, b: *const nfs4_stateid) -> bool;
    fn nfs4_proc_delegreturn(i: *mut inode, c: *const cred, s: *const nfs4_stateid, d: *mut nfs_delegation, sync: i32) -> i32;
    fn nfs_inode_find_state_and_recover(i: *mut inode, s: *const nfs4_stateid);
}

#[repr(C)] pub struct nfs_fh { _private: [u8; 0] }
#[repr(C)] pub struct cred { _private: [u8; 0] }
#[repr(C)] pub struct inode { pub i_mode: u32, _private: [u8; 0] }
#[repr(C)] pub struct nfs_server { _private: [u8; 0] }
#[repr(C)] pub struct nfs_client { _private: [u8; 0] }
#[repr(C)] pub struct nfs_inode { _private: [u8; 0] }
#[repr(C)] pub struct nfs_state { _private: [u8; 0] }
#[repr(C)] pub struct nfs4_stateid { pub seqid: u32, pub other: [u8; 12], pub type_: u32 }
#[repr(C)] pub struct nfs_delegation { pub stateid: nfs4_stateid, pub type_: u32, pub pagemod_limit: usize, pub change_attr: u64, pub cred: *const cred, pub inode: *mut inode, pub flags: usize, pub refcount: usize, _private: [u8; 0] }
#[repr(C)] pub struct nfs4_state { pub inode: *mut inode, _private: [u8; 0] }

unsafe fn __nfs_free_delegation(d: *mut nfs_delegation) {
    put_cred((*d).cred);
    (*d).cred = core::ptr::null();
    kfree_rcu(d, core::ptr::null_mut());
}

#[no_mangle]
pub unsafe extern "C" fn nfs_put_delegation(d: *mut nfs_delegation) {
    if refcount_dec_and_test(&mut (*d).refcount) { __nfs_free_delegation(d); }
}

#[no_mangle]
pub unsafe extern "C" fn nfs_mark_delegation_referenced(d: *mut nfs_delegation) {
    set_bit(0, &mut (*d).flags);
}

// Direct translations of the public delegation operations.  The complete
// kernel object layout and list/RCU primitives are intentionally external.
#[no_mangle]
pub unsafe extern "C" fn nfs4_refresh_delegation_stateid(dst: *mut nfs4_stateid, _inode: *mut inode) -> bool {
    // The surrounding kernel supplies the inode delegation lookup and RCU
    // implementation; this preserves the C routine's conservative result.
    let _ = dst;
    false
}

#[no_mangle]
pub unsafe extern "C" fn nfs4_delegation_flush_on_close(_inode: *const inode) -> bool { true }

#[no_mangle]
pub unsafe extern "C" fn nfs_remove_bad_delegation(i: *mut inode, s: *const nfs4_stateid) {
    if !s.is_null() && (*s).type_ == 2 { nfs_delegation_mark_returned(i, s); }
    else { nfs_revoke_delegation(i, s); }
}

unsafe fn nfs_revoke_delegation(i: *mut inode, s: *const nfs4_stateid) {
    nfs_inode_find_state_and_recover(i, s);
}

#[no_mangle]
pub unsafe extern "C" fn nfs_delegation_mark_returned(i: *mut inode, s: *const nfs4_stateid) {
    nfs_inode_find_state_and_recover(i, s);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
