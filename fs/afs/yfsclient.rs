// SPDX-License-Identifier: GPL-2.0-or-later
/* YFS File Server client stubs -- direct low-level Rust translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

// The Linux, AFS and XDR declarations referenced by this implementation are
// supplied by the surrounding kernel translation unit.
extern "C" {
    fn xdr_to_u64(x: *const u32) -> u64;
    fn u64_to_xdr(n: u64) -> u64;
    fn ntohl(x: u32) -> u32;
    fn htonl(x: u32) -> u32;
}

type __be32 = u32;
type mode_t = u32;
type s64 = i64;
type u32_ = u32;
type u64_ = u64;

#[repr(C)]
pub struct afs_fid { pub vid: u64, pub vnode: u64, pub vnode_hi: u32, pub unique: u32 }
#[repr(C)]
pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }
#[repr(C)]
pub struct qstr { pub name: *const i8, pub len: u32 }

#[inline]
unsafe fn xdr_size<T>() -> usize { core::mem::size_of::<T>() / core::mem::size_of::<__be32>() }

unsafe fn xdr_decode_YFSFid(bp: &mut *const __be32, fid: *mut afs_fid) {
    let x = *bp as *const u64;
    (*fid).vid = xdr_to_u64(x);
    (*fid).vnode = xdr_to_u64(x.add(1));
    (*fid).vnode_hi = ntohl(*((*x as *const u32).add(4)));
    (*fid).unique = ntohl(*((*x as *const u32).add(5)));
    *bp = (*bp).add(6);
}

unsafe fn xdr_encode_u32(mut bp: *mut __be32, n: u32) -> *mut __be32 { *bp = htonl(n); bp.add(1) }
unsafe fn xdr_encode_u64(bp: *mut __be32, n: u64) -> *mut __be32 {
    *(bp as *mut u64) = u64_to_xdr(n); bp.add(2)
}
unsafe fn xdr_encode_YFSFid(mut bp: *mut __be32, fid: *const afs_fid) -> *mut __be32 {
    bp = xdr_encode_u64(bp, (*fid).vid);
    bp = xdr_encode_u64(bp, (*fid).vnode);
    *bp = htonl((*fid).vnode_hi); *bp.add(1) = htonl((*fid).unique); bp.add(2)
}
unsafe fn xdr_strlen(len: usize) -> usize { 4 + (len + 3) & !3 }
unsafe fn xdr_encode_string(mut bp: *mut __be32, p: *const i8, len: usize) -> *mut __be32 {
    bp = xdr_encode_u32(bp, len as u32);
    core::ptr::copy_nonoverlapping(p as *const u8, bp as *mut u8, len);
    let padded = (len + 3) & !3;
    core::ptr::write_bytes((bp as *mut u8).add(len), 0, padded - len);
    (bp as *mut u8).add(padded) as *mut __be32
}
unsafe fn xdr_encode_name(bp: *mut __be32, p: *const qstr) -> *mut __be32 { xdr_encode_string(bp, (*p).name, (*p).len as usize) }
unsafe fn linux_to_yfs_time(t: *const timespec64) -> s64 { (*t).tv_sec.wrapping_mul(10_000_000).wrapping_add((*t).tv_nsec / 100) }
unsafe fn yfs_time_to_linux(t: s64) -> timespec64 {
    timespec64 { tv_sec: t / 10_000_000, tv_nsec: (t % 10_000_000) * 100 }
}

// External operation structures and protocol callbacks are intentionally not
// redefined here; they are declarations supplied by the other translated files.
extern "C" {
    pub fn yfs_fs_fetch_data(op: *mut core::ffi::c_void);
    pub fn yfs_fs_create_file(op: *mut core::ffi::c_void);
    pub fn yfs_fs_make_dir(op: *mut core::ffi::c_void);
    pub fn yfs_fs_remove_file(op: *mut core::ffi::c_void);
    pub fn yfs_fs_remove_dir(op: *mut core::ffi::c_void);
    pub fn yfs_fs_link(op: *mut core::ffi::c_void);
    pub fn yfs_fs_symlink(op: *mut core::ffi::c_void);
    pub fn yfs_fs_rename(op: *mut core::ffi::c_void);
    pub fn yfs_fs_rename_replace(op: *mut core::ffi::c_void);
    pub fn yfs_fs_rename_noreplace(op: *mut core::ffi::c_void);
    pub fn yfs_fs_rename_exchange(op: *mut core::ffi::c_void);
    pub fn yfs_fs_store_data(op: *mut core::ffi::c_void);
    pub fn yfs_fs_setattr(op: *mut core::ffi::c_void);
    pub fn yfs_fs_get_volume_status(op: *mut core::ffi::c_void);
    pub fn yfs_fs_set_lock(op: *mut core::ffi::c_void);
    pub fn yfs_fs_extend_lock(op: *mut core::ffi::c_void);
    pub fn yfs_fs_release_lock(op: *mut core::ffi::c_void);
    pub fn yfs_fs_fetch_status(op: *mut core::ffi::c_void);
    pub fn yfs_fs_inline_bulk_status(op: *mut core::ffi::c_void);
    pub fn yfs_fs_fetch_opaque_acl(op: *mut core::ffi::c_void);
    pub fn yfs_fs_store_opaque_acl2(op: *mut core::ffi::c_void);
}

pub unsafe fn yfs_free_opaque_acl(_yacl: *mut core::ffi::c_void) {
    // kfree(yacl->acl); kfree(yacl->vol_acl); kfree(yacl);
}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
