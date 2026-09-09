/* SPDX-License-Identifier: GPL-2.0-or-later */
/* Rust translation of internal.h.  Kernel and AFS dependency types are supplied externally. */
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub const AFS_CELL_MAX_ADDRS: usize = 15;
pub const AFS_NR_SYSNAME: usize = 16;
pub const AFS_MAX_ADDRESSES: u32 = (core::mem::size_of::<usize>() * 8) as u32;

/* External kernel/AFS types. */
pub type afs_volid_t = u64; pub type afs_voltype_t = u32; pub type afs_dataversion_t = u64;
pub type afs_access_t = u32; pub type afs_lock_type_t = u8; pub type time64_t = i64;
pub type uuid_t = [u8; 16]; pub type __be32 = u32; pub type __be64 = u64;
pub type loff_t = i64; pub type ktime_t = i64; pub type mode_t = u32;
#[repr(C)] pub struct opaque<T = ()>(pub T);
macro_rules! opaque { ($($n:ident),* $(,)?) => { $(#[repr(C)] pub struct $n { _private: [u8;0] })* }; }
opaque!(rcu_head, rb_node, rb_root, refcount_t, wait_queue_head_t, work_struct, rxrpc_call,
 rxrpc_peer, key, afs_net, afs_cell, afs_volume, afs_server, afs_vlserver, iov_iter, kvec,
 bio_vec, afs_fid, afs_file_status, afs_status_cb, afs_volsync, dentry, inode, super_block,
 netfs_inode, afs_permits, folio_queue, folio, socket, mutex, atomic_t, atomic_long_t,
 rw_semaphore, hlist_head, seqlock_t, list_head, hlist_node, spinlock_t, timer_list,
 proc_dir_entry, rwlock_t, idr, net, fscache_cookie, fscache_volume, krb5_buffer, rb_node,
 file, file_lock, address_space, writeback_control, vm_fault, mnt_idmap, path, kstat,
 inode_operations, file_operations, address_space_operations, dentry_operations, netfs_request_ops,
 workqueue_struct, delayed_work, xattr_handler, afs_callback_break, afs_endpoint_state,
 afs_vlserver_list, afs_acl, yfs_acl, netfs_io_subrequest, netfs_io_request, netfs_io_stream,
 iattr, kstatfs, timespec64, afs_symlink, afs_server_state, afs_vldb_entry, afs_operation,
 afs_call_type, afs_operation_ops, afs_addr_list, afs_server_entry, afs_server_list);
pub type afs_uuid = uuid_t; pub type afs_file = opaque; pub type afs_fs_context = opaque;
pub type afs_address = opaque; pub type afs_permit = opaque; pub type afs_vnode_param = opaque;
pub type afs_vnode = opaque; pub type afs_call = opaque; pub type afs_vl_cursor = opaque;

#[repr(C)] #[derive(Copy,Clone)] pub union afs_addr_preference_addr { pub ipv4_addr:u32, pub ipv6_addr:[u8;16] }
#[repr(C)] pub struct afs_addr_preference { pub addr: afs_addr_preference_addr, pub family:u16, pub prio:u16, pub subnet_mask:u8 }
#[repr(C)] pub struct afs_fs_context_real { pub force:bool, pub autocell:bool, pub dyn_root:bool, pub no_cell:bool, pub flock_mode:afs_flock_mode, pub type_:afs_voltype_t, pub volnamesz:u32, pub volname:*const c_char, pub net:*mut afs_net, pub cell:*mut afs_cell, pub volume:*mut afs_volume, pub key:*mut key }
#[repr(C)] pub struct afs_address_real { pub peer:*mut rxrpc_peer, pub last_error:i16, pub prio:u16 }
#[repr(C)] pub struct afs_addr_list_real { pub rcu:rcu_head, pub usage:refcount_t, pub version:u32, pub debug_id:u32, pub addr_pref_version:u32, pub max_addrs:u8, pub nr_addrs:u8, pub preferred:u8, pub nr_ipv4:u8, pub source:u8, pub status:u8, pub probe_failed:usize, pub responded:usize }
#[repr(C)] pub struct afs_error { pub abort_code:i32, pub error:i16, pub responded:bool, pub aborted:bool }
#[repr(C)] pub struct afs_sysnames { pub subs:[*mut c_char;AFS_NR_SYSNAME], pub usage:refcount_t, pub nr:u16, pub blank:[c_char;1] }

#[repr(u32)] pub enum afs_flock_mode { afs_flock_mode_unset, afs_flock_mode_local, afs_flock_mode_openafs, afs_flock_mode_strict, afs_flock_mode_write }
#[repr(u32)] pub enum afs_call_state { AFS_CALL_CL_REQUESTING, AFS_CALL_CL_AWAIT_REPLY, AFS_CALL_CL_PROC_REPLY, AFS_CALL_SV_AWAIT_OP_ID, AFS_CALL_SV_AWAIT_REQUEST, AFS_CALL_SV_REPLYING, AFS_CALL_SV_AWAIT_ACK, AFS_CALL_COMPLETE }
#[repr(u32)] pub enum afs_cell_state { AFS_CELL_SETTING_UP, AFS_CELL_UNLOOKED, AFS_CELL_ACTIVE, AFS_CELL_REMOVING, AFS_CELL_DEAD }
#[repr(u32)] pub enum afs_lock_state { AFS_VNODE_LOCK_NONE, AFS_VNODE_LOCK_WAITING_FOR_CB, AFS_VNODE_LOCK_SETTING, AFS_VNODE_LOCK_GRANTED, AFS_VNODE_LOCK_EXTENDING, AFS_VNODE_LOCK_NEED_UNLOCK, AFS_VNODE_LOCK_UNLOCKING, AFS_VNODE_LOCK_DELETED }
#[repr(u8)] pub enum afs_ro_replicating { AFS_RO_NOT_REPLICATING, AFS_RO_REPLICATING_USE_OLD, AFS_RO_REPLICATING_USE_NEW }
#[repr(C)] pub struct afs_vnode_cache_aux { pub data_version:__be64 }
#[repr(C)] pub struct afs_acl_real { pub size:u32, pub data:[u8;0] }
#[repr(C)] pub struct yfs_acl_real { pub acl:*mut afs_acl, pub vol_acl:*mut afs_acl, pub inherit_flag:u32, pub num_cleaned:u32, pub flags:u32 }

/* C flexible-array records are represented with trailing zero-length arrays. */
#[repr(C)] pub struct afs_vlserver_entry { pub priority:u16, pub weight:u16, pub source:u8, pub status:u8, pub server:*mut afs_vlserver }
#[repr(C)] pub struct afs_vlserver_list { pub rcu:rcu_head, pub ref_:refcount_t, pub nr_servers:u8, pub index:u8, pub preferred:u8, pub source:u8, pub status:u8, pub lock:rwlock_t, pub servers:[afs_vlserver_entry;0] }
#[repr(C)] pub struct afs_server_entry { pub server:*mut afs_server, pub volume:*mut afs_volume, pub slink:list_head, pub cb_expires_at:time64_t, pub flags:usize }
#[repr(C)] pub struct afs_server_list { pub rcu:rcu_head, pub usage:refcount_t, pub attached:bool, pub ro_replicating:afs_ro_replicating, pub nr_servers:u8, pub vnovol_mask:u16, pub seq:u32, pub lock:rwlock_t, pub servers:[afs_server_entry;0] }
#[repr(C)] pub struct afs_dir_iter { pub dvnode:*mut afs_vnode, pub block:*mut c_void, pub fq:*mut folio_queue, pub fpos:u32, pub fq_slot:c_int, pub loop_check:u32, pub nr_slots:u8, pub bucket:u8, pub prev_entry:u32 }

extern "C" { pub static mut afs_debug: u32; pub static afs_init_sysname:c_char; }
#[inline] pub unsafe fn afs_file_key(_file:*mut file)->*mut key { core::ptr::null_mut() }
#[inline] pub unsafe fn afs_calc_vnode_cb_break(v:*mut afs_vnode)->u32 { let _=v; 0 }
#[inline] pub unsafe fn afs_set_cache_aux(_v:*mut afs_vnode,_a:*mut afs_vnode_cache_aux) {}
#[inline] pub unsafe fn afs_op_nomem(_op:*mut afs_operation) {}
#[inline] pub unsafe fn afs_op_error(_op:*const afs_operation)->c_int { 0 }
#[inline] pub unsafe fn afs_op_abort_code(_op:*const afs_operation)->i32 { 0 }
#[inline] pub unsafe fn afs_op_set_error(_op:*mut afs_operation,error:c_int)->c_int { error }

/* The remaining header declarations are external kernel-facing interfaces. */
extern "C" {
 pub fn afs_get_addrlist(a:*mut afs_addr_list, reason:u32)->*mut afs_addr_list;
 pub fn afs_put_addrlist(a:*mut afs_addr_list, reason:u32);
 pub fn afs_alloc_operation(k:*mut key,v:*mut afs_volume)->*mut afs_operation;
 pub fn afs_put_operation(o:*mut afs_operation)->c_int;
 pub fn afs_begin_vnode_operation(o:*mut afs_operation)->bool;
 pub fn afs_end_vnode_operation(o:*mut afs_operation);
 pub fn afs_wait_for_operation(o:*mut afs_operation);
 pub fn afs_do_sync_operation(o:*mut afs_operation)->c_int;
 pub fn afs_invalidate_symlink(v:*mut afs_vnode);
 pub fn afs_validate(v:*mut afs_vnode,k:*mut key)->c_int;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
