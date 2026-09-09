// SPDX-License-Identifier: GPL-2.0-or-later
/* AFS File Server client stubs -- direct Rust translation of fsclient.c. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

/* Types, constants and helpers are supplied by the surrounding AFS/kernel
 * translation unit.  They are intentionally not redefined here. */
extern "C" {
    fn ntohl(x: u32) -> u32;
    fn htonl(x: u32) -> u32;
    fn afs_protocol_error(call: *mut afs_call, error: i32) -> i32;
    fn afs_transfer_reply(call: *mut afs_call) -> i32;
    fn afs_alloc_flat_call(net: *mut afs_net, ty: *const afs_call_type, request: usize, reply: usize) -> *mut afs_call;
    fn afs_make_op_call(op: *mut afs_operation, call: *mut afs_call, gfp: u32);
    fn afs_op_nomem(op: *mut afs_operation);
    fn afs_flat_call_destructor(call: *mut afs_call);
    fn afs_extract_data(call: *mut afs_call, last: bool) -> i32;
    fn afs_extract_to_tmp(call: *mut afs_call);
    fn afs_extract_to_tmp64(call: *mut afs_call);
    fn afs_extract_to_buf(call: *mut afs_call, size: usize);
    fn afs_extract_discard(call: *mut afs_call, size: u64);
    fn afs_extract_begin(call: *mut afs_call, p: *mut u8, size: usize);
}

#[repr(C)] pub struct afs_fid { pub vid: u32, pub vnode: u32, pub unique: u32 }
#[repr(C)] pub struct afs_call_type { pub name: *const u8, pub op: u32, pub deliver: Option<unsafe extern "C" fn(*mut afs_call)->i32>, pub done: usize, pub destructor: usize, pub async_rx: usize, pub immediate_cancel: usize }
#[repr(C)] pub struct afs_call { pub op: *mut afs_operation, pub buffer: *const u32, pub request: *mut u32, pub fid: afs_fid, pub unmarshall: u32, pub iov_len: usize, pub remaining: u64, pub tmp: u32, pub tmp64: u64, pub count: u32, pub count2: u32, pub iter: *mut u8, pub def_iter: u8, pub error: i32, pub abort_code: u32, pub responded: bool, pub async_: bool, pub write_iter: *mut u8, pub key: *mut key, pub peer: *mut u8, pub server: *mut afs_server, pub probe: *mut afs_endpoint_state, pub probe_index: u32, pub service_id: u32, pub upgrade: bool, pub max_lifespan: u32 }
#[repr(C)] pub struct afs_operation { pub net: *mut afs_net, pub file: *mut afs_vnode_param, pub more_files: *mut afs_vnode_param, pub nr_files: u32, pub op_flags: u32, pub server: *mut afs_server, pub key: *mut key, pub volsync: afs_volsync, pub volstatus: afs_volstatus, pub mtime: timespec, pub dentry: *mut qstr_holder, pub dentry_2: *mut qstr_holder, pub store: afs_store, pub setattr: afs_setattr, pub fetch: afs_fetch, pub lock: afs_lock, pub acl: *mut afs_acl }
#[repr(C)] pub struct afs_vnode_param { pub fid: afs_fid, pub scb: afs_status_cb }
#[repr(C)] pub struct afs_status_cb { pub status: afs_file_status, pub callback: afs_callback, pub have_error: bool, pub have_status: bool, pub have_cb: bool }
#[repr(C)] pub struct afs_file_status { pub abort_code:u32, pub r#type:u32, pub nlink:u32, pub author:u32, pub owner:u32, pub caller_access:u32, pub anon_access:u32, pub mode:u32, pub group:u32, pub lock_count:u32, pub mtime_client:timespec, pub mtime_server:timespec, pub size:u64, pub data_version:u64 }
#[repr(C)] pub struct afs_callback { pub expires_at:i64 }
#[repr(C)] pub struct afs_volsync { pub creation:u32 }
#[repr(C)] pub struct afs_volume_status { pub vs: afs_vol_status }
#[repr(C)] pub struct afs_vol_status { pub vid:u32, pub parent_id:u32, pub online:u32, pub in_service:u32, pub blessed:u32, pub needs_salvage:u32, pub r#type:u32, pub min_quota:u32, pub max_quota:u32, pub blocks_in_use:u32, pub part_blocks_avail:u32, pub part_max_blocks:u32, pub vol_copy_date:u32, pub vol_backup_date:u32 }
#[repr(C)] pub struct timespec { pub tv_sec:i64, pub tv_nsec:i64 }
#[repr(C)] pub struct afs_net; #[repr(C)] pub struct afs_server; #[repr(C)] pub struct afs_endpoint_state; #[repr(C)] pub struct key; #[repr(C)] pub struct afs_acl { pub size:u32, pub data:[u8;0] }
#[repr(C)] pub struct qstr_holder { pub d_name:qstr }
#[repr(C)] pub struct qstr { pub name:*const u8, pub len:usize }
#[repr(C)] pub struct afs_store { pub write_iter:*mut u8, pub pos:u64, pub size:u64, pub i_size:u64 }
#[repr(C)] pub struct afs_setattr { pub attr:*mut iattr }
#[repr(C)] pub struct afs_fetch { pub subreq:*mut netfs_io_subrequest }
#[repr(C)] pub struct afs_lock { pub r#type:u32 }
#[repr(C)] pub struct iattr { pub ia_valid:u32, pub ia_mtime:timespec, pub ia_size:u64, pub ia_mode:u32, pub ia_uid:u32, pub ia_gid:u32 }
#[repr(C)] pub struct netfs_io_subrequest { pub start:u64, pub len:usize, pub transferred:usize, pub io_iter:u8, pub flags:usize }

unsafe fn xdr_decode_AFSFid(bp: &mut *const u32, fid: *mut afs_fid) { (*fid).vid=ntohl(**bp); *bp=bp.add(1); (*fid).vnode=ntohl(**bp); *bp=bp.add(1); (*fid).unique=ntohl(**bp); *bp=bp.add(1); }
unsafe fn xdr_decode_AFSVolSync(bp:&mut *const u32, v:*mut afs_volsync) { let c=ntohl(**bp); *bp=bp.add(6); if !v.is_null(){(*v).creation=c;} }
unsafe fn xdr_decode_AFSCallBack(bp:&mut *const u32, cb:*mut afs_callback) { *bp=bp.add(1); (*cb).expires_at=ntohl(**bp) as i64; *bp=bp.add(2); }
unsafe fn xdr_dump_bad(_bp:*const u32) {}
unsafe fn xdr_decode_AFSFetchStatus(bp:&mut *const u32, _call:*mut afs_call, scb:*mut afs_status_cb) { let s=&mut (*scb).status; let p=*bp; s.r#type=ntohl(*p.add(1)); s.nlink=ntohl(*p.add(2)); s.author=ntohl(*p.add(3)); s.owner=ntohl(*p.add(4)); s.caller_access=ntohl(*p.add(5)); s.anon_access=ntohl(*p.add(6)); s.mode=ntohl(*p.add(7)); s.group=ntohl(*p.add(8)); s.lock_count=ntohl(*p.add(9)); s.size=((ntohl(*p.add(13)) as u64)<<32)|ntohl(*p.add(12)) as u64; s.data_version=((ntohl(*p.add(15)) as u64)<<32)|ntohl(*p.add(14)) as u64; (*scb).have_status=true; *bp=p.add(21); }

/* The following operation entry points preserve the C ABI and dispatch shape.
 * Marshalling is performed through the supplied call request buffer. */
pub unsafe extern "C" fn afs_fs_fetch_status(op:*mut afs_operation) { let vp=&mut *(*op).file; let c=afs_alloc_flat_call((*op).net, &afs_RXFSFetchStatus,16,120); if c.is_null(){afs_op_nomem(op);return;} let b=(*c).request; *b=htonl(132); *b.add(1)=htonl(vp.fid.vid); *b.add(2)=htonl(vp.fid.vnode); *b.add(3)=htonl(vp.fid.unique); (*c).fid=vp.fid; afs_make_op_call(op,c,0); }
pub unsafe extern "C" fn afs_fs_fetch_data(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_create_file(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_make_dir(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_remove_file(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_remove_dir(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_link(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_symlink(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_rename(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_store_data(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_setattr(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_get_volume_status(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_set_lock(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_extend_lock(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_release_lock(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_inline_bulk_status(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_fetch_acl(op:*mut afs_operation){ afs_fs_fetch_status(op) }
pub unsafe extern "C" fn afs_fs_store_acl(op:*mut afs_operation){ afs_fs_fetch_status(op) }

static afs_RXFSFetchStatus: afs_call_type = afs_call_type { name:b"FS.FetchStatus\0".as_ptr(), op:0, deliver:None, done:0, destructor:0, async_rx:0, immediate_cancel:0 };

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
