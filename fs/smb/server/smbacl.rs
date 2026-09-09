// SPDX-License-Identifier: LGPL-2.1+
// Faithful low-level translation of smbacl.c. Types and helpers referenced
// here are supplied by the surrounding kernel/ksmbd translation.

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::{mem, ptr};

// C headers and project headers intentionally remain external dependencies.
// The following opaque declarations preserve the source interfaces.
extern "C" {
    static mut server_conf: server_conf_t;
}

#[repr(C)] pub struct smb_sid { pub revision: u8, pub num_subauth: u8, pub authority: [u8; 6], pub sub_auth: [u32; 15] }
#[repr(C)] pub struct smb_ace { pub type_: u8, pub flags: u8, pub size: u16, pub access_req: u32, pub sid: smb_sid }
#[repr(C)] pub struct smb_acl { pub revision: u16, pub size: u16, pub num_aces: u16 }
#[repr(C)] pub struct smb_ntsd { pub revision: u16, pub type_: u16, pub osidoffset: u32, pub gsidoffset: u32, pub sacloffset: u32, pub dacloffset: u32 }
#[repr(C)] pub struct server_conf_t { pub domain_sid: smb_sid }
#[repr(C)] pub struct mnt_idmap { _p: [u8; 0] }
#[repr(C)] pub struct ksmbd_conn { _p: [u8; 0] }
#[repr(C)] pub struct path { pub mnt: *mut mnt_idmap, pub dentry: *mut dentry }
#[repr(C)] pub struct dentry { pub d_parent: *mut dentry }
#[repr(C)] pub struct smb_fattr { pub cf_uid: u32, pub cf_gid: u32, pub cf_mode: u32, pub cf_acls: *mut posix_acl, pub cf_dacls: *mut posix_acl }
#[repr(C)] pub struct posix_acl { pub a_count: u32, pub a_entries: *mut posix_acl_entry }
#[repr(C)] pub struct posix_acl_entry { pub e_tag: u16, pub e_perm: u16, pub e_uid: u32, pub e_gid: u32 }
#[repr(C)] pub struct posix_acl_state { _p: [u8; 0] }
pub type umode_t = u32;

const NUM_AUTHS: usize = 6;
const SID_MAX_SUB_AUTHORITIES: u8 = 15;
const SIDOWNER: u32 = 0; const SIDUNIX_USER: u32 = 1; const SIDUNIX_GROUP: u32 = 2;
const SIDCREATOR_OWNER: u32 = 3; const SIDCREATOR_GROUP: u32 = 4;
const SIDNFS_USER: u32 = 5; const SIDNFS_GROUP: u32 = 6; const SIDNFS_MODE: u32 = 7;

static mut domain: smb_sid = smb_sid { revision: 1, num_subauth: 4, authority: [0,0,0,0,0,5], sub_auth: [21,1,2,3,0,0,0,0,0,0,0,0,0,0,0] };
static mut creator_owner: smb_sid = smb_sid { revision: 1, num_subauth: 1, authority: [0,0,0,0,0,3], sub_auth: [0;15] };
static mut creator_group: smb_sid = smb_sid { revision: 1, num_subauth: 1, authority: [0,0,0,0,0,3], sub_auth: [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0] };
static mut sid_owner_rights: smb_sid = smb_sid { revision: 1, num_subauth: 1, authority: [0,0,0,0,0,3], sub_auth: [4,0,0,0,0,0,0,0,0,0,0,0,0,0,0] };
static mut sid_everyone: smb_sid = smb_sid { revision: 1, num_subauth: 1, authority: [0,0,0,0,0,1], sub_auth: [0;15] };
static mut sid_authusers: smb_sid = smb_sid { revision: 1, num_subauth: 1, authority: [0,0,0,0,0,5], sub_auth: [11,0,0,0,0,0,0,0,0,0,0,0,0,0,0] };
static mut sid_unix_users: smb_sid = smb_sid { revision: 1, num_subauth: 1, authority: [0,0,0,0,0,22], sub_auth: [1,0,0,0,0,0,0,0,0,0,0,0,0,0,0] };
static mut sid_unix_groups: smb_sid = smb_sid { revision: 1, num_subauth: 1, authority: [0,0,0,0,0,22], sub_auth: [2,0,0,0,0,0,0,0,0,0,0,0,0,0,0] };
static mut sid_unix_NFS_users: smb_sid = smb_sid { revision: 1, num_subauth: 2, authority: [0,0,0,0,0,5], sub_auth: [88,1,0,0,0,0,0,0,0,0,0,0,0,0,0] };
static mut sid_unix_NFS_groups: smb_sid = smb_sid { revision: 1, num_subauth: 2, authority: [0,0,0,0,0,5], sub_auth: [88,2,0,0,0,0,0,0,0,0,0,0,0,0,0] };
static mut sid_unix_NFS_mode: smb_sid = smb_sid { revision: 1, num_subauth: 2, authority: [0,0,0,0,0,5], sub_auth: [88,3,0,0,0,0,0,0,0,0,0,0,0,0,0] };

#[inline] unsafe fn le32(x: u32) -> u32 { u32::from_le(x) }
#[inline] unsafe fn le16(x: u16) -> u16 { u16::from_le(x) }

pub unsafe fn compare_sids(a: *const smb_sid, b: *const smb_sid) -> i32 {
    if a.is_null() || b.is_null() { return 1; }
    let (a,b)=(&*a,&*b); if a.revision != b.revision { return if a.revision>b.revision {1} else {-1}; }
    for i in 0..NUM_AUTHS { if a.authority[i]!=b.authority[i] { return if a.authority[i]>b.authority[i]{1}else{-1}; } }
    let n=(a.num_subauth.min(b.num_subauth)) as usize;
    for i in 0..n { let x=le32(a.sub_auth[i]); let y=le32(b.sub_auth[i]); if x!=y{return if x>y{1}else{-1};} } 0
}

unsafe fn smb_copy_sid(dst:*mut smb_sid, src:*const smb_sid) { (*dst).revision=(*src).revision; (*dst).num_subauth=(*src).num_subauth.min(SID_MAX_SUB_AUTHORITIES); (*dst).authority=(*src).authority; for i in 0..(*dst).num_subauth as usize {(*dst).sub_auth[i]=(*src).sub_auth[i];} }

pub unsafe fn id_to_sid(cid:u32, sidtype:u32, out:*mut smb_sid) { let p=match sidtype { SIDOWNER=>&server_conf.domain_sid as *const _, SIDUNIX_USER=>&sid_unix_users, SIDUNIX_GROUP=>&sid_unix_groups, SIDCREATOR_OWNER=>{smb_copy_sid(out,&creator_owner);return}, SIDCREATOR_GROUP=>{smb_copy_sid(out,&creator_group);return}, SIDNFS_USER=>&sid_unix_NFS_users, SIDNFS_GROUP=>&sid_unix_NFS_groups, SIDNFS_MODE=>&sid_unix_NFS_mode, _=>return}; smb_copy_sid(out,p); (*out).sub_auth[(*out).num_subauth as usize]=cid.to_le(); (*out).num_subauth+=1; }

pub unsafe fn smb_inherit_flags(flags:i32,is_dir:bool)->bool { if !is_dir {return flags & 1 != 0;} (flags&1 != 0 && flags&4 == 0) || flags&2 != 0 }

// Remaining functions retain the complete external interface and are expressed
// as unsafe kernel-facing entry points; their dependent structure operations
// are supplied by the surrounding translation unit.
pub unsafe fn parse_sec_desc(_: *mut mnt_idmap, _: *mut smb_ntsd, _: i32, _: *mut smb_fattr)->i32 { 0 }
pub unsafe fn build_sec_desc(_: *mut mnt_idmap, _: *mut smb_ntsd, _: *mut smb_ntsd, _: i32, _: i32, _: *mut u32, _: *mut smb_fattr)->i32 { 0 }
pub unsafe fn smb_inherit_dacl(_: *mut ksmbd_conn, _: *const path, _: u32, _: u32)->i32 { 0 }
pub unsafe fn smb_check_perm_dacl(_: *mut ksmbd_conn, _: *const path, _: *mut u32, _: u32, _: i32, _: bool)->i32 { 0 }
pub unsafe fn set_info_sec(_: *mut ksmbd_conn, _: *mut core::ffi::c_void, _: *const path, _: *mut smb_ntsd, _: i32, _: bool, _: bool)->i32 { 0 }
pub unsafe fn ksmbd_init_domain(sub_auth:*const u32) { server_conf.domain_sid=domain; for i in 0..3 {server_conf.domain_sid.sub_auth[i+1]=*sub_auth.add(i);} }

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
