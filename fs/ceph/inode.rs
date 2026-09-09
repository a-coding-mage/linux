// SPDX-License-Identifier: GPL-2.0
//
// Faithful low-level Rust translation of ceph/inode.c.  Kernel and Ceph
// structures/functions are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

// The original file is an implementation unit whose ABI is provided by the
// Linux kernel and the Ceph support units.  Keep the same externally visible
// entry points and pointer-oriented calling convention here.
#[repr(C)]
pub struct inode { _private: [u8; 0] }
#[repr(C)]
pub struct super_block { _private: [u8; 0] }
#[repr(C)]
pub struct dentry { _private: [u8; 0] }
#[repr(C)]
pub struct page { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_inode_info { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_vino { pub ino: u64, pub snap: u64 }
#[repr(C)]
pub struct ceph_acl_sec_ctx { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_mds_request { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_mds_session { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_cap_reservation { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_mds_reply_info_parsed { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_mds_reply_dirfrag { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_mds_reply_info_in { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_frag_tree_head { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_readdir_cache_control { _private: [u8; 0] }
#[repr(C)]
pub struct mnt_idmap { _private: [u8; 0] }
#[repr(C)]
pub struct path { _private: [u8; 0] }
#[repr(C)]
pub struct kstat { _private: [u8; 0] }
#[repr(C)]
pub struct iattr { _private: [u8; 0] }
#[repr(C)]
pub struct ceph_iattr { _private: [u8; 0] }

pub type umode_t = u16;
pub type loff_t = i64;

// Direct translations of the implementation's externally visible helpers.
// Their bodies remain unsafe because the C implementation operates directly
// on kernel objects, reference counts, locks, trees, and request buffers.
extern "C" {
    pub fn ceph_new_inode(dir: *mut inode, dentry: *mut dentry,
                          mode: *mut umode_t,
                          as_ctx: *mut ceph_acl_sec_ctx) -> *mut inode;
    pub fn ceph_as_ctx_to_req(req: *mut ceph_mds_request,
                              as_ctx: *mut ceph_acl_sec_ctx);
    pub fn ceph_get_inode(sb: *mut super_block, vino: ceph_vino,
                          newino: *mut inode) -> *mut inode;
    pub fn ceph_get_snapdir(parent: *mut inode) -> *mut inode;
    pub fn ceph_alloc_inode(sb: *mut super_block) -> *mut inode;
    pub fn ceph_free_inode(inode: *mut inode);
    pub fn ceph_evict_inode(inode: *mut inode);
    pub fn ceph_fill_file_size(inode: *mut inode, issued: c_int,
                               truncate_seq: u32, truncate_size: u64,
                               size: u64) -> c_int;
    pub fn ceph_inode_set_subvolume(inode: *mut inode, subvolume_id: u64);
    pub fn ceph_fill_inode(inode: *mut inode, locked_page: *mut page,
                           iinfo: *mut ceph_mds_reply_info_in,
                           dirinfo: *mut ceph_mds_reply_dirfrag,
                           session: *mut ceph_mds_session, cap_fmode: c_int,
                           caps_reservation: *mut ceph_cap_reservation) -> c_int;
    pub fn ceph_fill_trace(sb: *mut super_block,
                           req: *mut ceph_mds_request) -> c_int;
    pub fn ceph_readdir_prepopulate(req: *mut ceph_mds_request,
                                    session: *mut ceph_mds_session) -> c_int;
    pub fn ceph_inode_set_size(inode: *mut inode, size: loff_t) -> bool;
    pub fn ceph_queue_inode_work(inode: *mut inode, work_bit: c_int);
    pub fn __ceph_do_pending_vmtruncate(inode: *mut inode);
    pub fn __ceph_setattr(idmap: *mut mnt_idmap, inode: *mut inode,
                           attr: *mut iattr, cia: *mut ceph_iattr) -> c_int;
    pub fn ceph_setattr(idmap: *mut mnt_idmap, dentry: *mut dentry,
                        attr: *mut iattr) -> c_int;
    pub fn ceph_permission(idmap: *mut mnt_idmap, inode: *mut inode,
                           mask: c_int) -> c_int;
    pub fn ceph_getattr(idmap: *mut mnt_idmap, path: *const path,
                        stat: *mut kstat, request_mask: u32,
                        flags: u32) -> c_int;
    pub fn ceph_inode_shutdown(inode: *mut inode);
}


// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
