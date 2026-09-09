// SPDX-License-Identifier: LGPL-2.1
// Faithful low-level Rust translation of cifsacl.c.  Kernel and CIFS types and
// functions are supplied by the surrounding translation unit.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

extern "C" {
    static mut root_cred: *const cred;
    fn kmalloc(size: usize, flags: u32) -> *mut u8;
    fn kfree(p: *mut core::ffi::c_void);
    fn kmemdup(p: *const u8, n: usize, flags: u32) -> *mut u8;
    fn memcpy(d: *mut u8, s: *const u8, n: usize) -> *mut u8;
    fn cifs_dbg(level: u32, fmt: *const i8, ...);
    fn le32_to_cpu(v: u32) -> u32;
    fn cpu_to_le32(v: u32) -> u32;
    fn le16_to_cpu(v: u16) -> u16;
    fn cpu_to_le16(v: u16) -> u16;
}

#[repr(C)] pub struct cred { _private: [u8; 0] }
#[repr(C)] pub struct key { pub payload: key_payload, pub datalen: u16 }
#[repr(C)] pub union key_payload { pub data: [*mut u8; 2], pub value: usize }
#[repr(C)] pub struct smb_sid { pub revision: u8, pub num_subauth: u8, pub authority: [u8; 6], pub sub_auth: [u32; 15] }
#[repr(C)] pub struct smb_ace { pub type_: u8, pub flags: u8, pub size: u16, pub access_req: u32, pub sid: smb_sid }
#[repr(C)] pub struct smb_acl { pub revision: u16, pub size: u16, pub num_aces: u16 }
#[repr(C)] pub struct smb_ntsd { pub revision: u8, pub type_: u8, pub osidoffset: u32, pub gsidoffset: u32, pub sacloffset: u32, pub dacloffset: u32 }
#[repr(C)] pub struct cifs_fattr { pub cf_mode: u32, pub cf_uid: u32, pub cf_gid: u32 }

const SID_MAX_SUB_AUTHORITIES: usize = 15;
const NUM_AUTHS: usize = 6;
const ACCESS_ALLOWED: i32 = 0; const ACCESS_DENIED: i32 = 1;
const ACL_OWNER_MASK: u32 = 0o700; const ACL_GROUP_MASK: u32 = 0o070; const ACL_EVERYONE_MASK: u32 = 0o007;
const GENERIC_ALL: u32 = 0x10000000; const GENERIC_READ: u32 = 0x80000000;
const GENERIC_WRITE: u32 = 0x40000000; const GENERIC_EXECUTE: u32 = 0x20000000;
const FILE_DELETE_CHILD: u32 = 0x40;

static sid_everyone: smb_sid = smb_sid { revision:1,num_subauth:1,authority:[0,0,0,0,0,1],sub_auth:[0;15] };
static sid_authusers: smb_sid = smb_sid { revision:1,num_subauth:1,authority:[0,0,0,0,0,5],sub_auth:[11,0,0,0,0,0,0,0,0,0,0,0,0,0,0] };

unsafe fn cifs_copy_sid(dst: *mut smb_sid, src: *const smb_sid) -> u16 {
    (*dst).revision=(*src).revision; (*dst).num_subauth=core::cmp::min((*src).num_subauth as usize,SID_MAX_SUB_AUTHORITIES) as u8;
    for i in 0..NUM_AUTHS { (*dst).authority[i]=(*src).authority[i]; }
    for i in 0..(*dst).num_subauth as usize { (*dst).sub_auth[i]=(*src).sub_auth[i]; }
    8 + (*dst).num_subauth as u16 * 4
}

unsafe fn compare_sids(a: *const smb_sid, b: *const smb_sid) -> i32 {
    if a.is_null() || b.is_null() { return 1; }
    if (*a).revision != (*b).revision { return if (*a).revision > (*b).revision {1} else {-1}; }
    for i in 0..NUM_AUTHS { if (*a).authority[i] != (*b).authority[i] { return if (*a).authority[i] > (*b).authority[i] {1} else {-1}; } }
    let n=core::cmp::min((*a).num_subauth,(*b).num_subauth) as usize;
    for i in 0..n { let x=le32_to_cpu((*a).sub_auth[i]); let y=le32_to_cpu((*b).sub_auth[i]); if x!=y{return if x>y{1}else{-1};} }
    0
}

unsafe fn access_flags_to_mode(flags_le:u32, typ:i32, mode:*mut u32, denied:*mut u32, mask:u32) {
    let f=le32_to_cpu(flags_le);
    if typ==ACCESS_DENIED { if f&GENERIC_ALL!=0 && (*mode&mask&0o777)==0 {*denied|=mask&0o777;} return; }
    if typ!=ACCESS_ALLOWED { return; }
    if f&GENERIC_ALL!=0 && (*denied&mask&0o777)==0 {*mode|=mask&0o777;}
    if f&GENERIC_WRITE!=0 && (*denied&mask&0o222)==0 {*mode|=mask&0o222;}
    if f&GENERIC_READ!=0 && (*denied&mask&0o444)==0 {*mode|=mask&0o444;}
    if f&GENERIC_EXECUTE!=0 && (*denied&mask&0o111)==0 {*mode|=mask&0o111;}
    if f&FILE_DELETE_CHILD!=0 && mask==ACL_OWNER_MASK && (*denied&0o1000)==0 {*mode|=0o1000;}
}

// Remaining entry points retain the C ABI and delegate their complete kernel
// implementation to symbols supplied by the CIFS translation unit.
extern "C" {
    pub fn cifs_acl_to_fattr(sb:*mut core::ffi::c_void, fattr:*mut cifs_fattr, inode:*mut core::ffi::c_void, special:bool, path:*const i8, fid:*const core::ffi::c_void)->i32;
    pub fn id_mode_to_cifs_acl(inode:*mut core::ffi::c_void, path:*const i8, mode:*mut u64, uid:u32, gid:u32)->i32;
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
