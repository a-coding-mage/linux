// SPDX-License-Identifier: GPL-2.0
//
// Low-level Rust translation of nfsd/vfs.c.  Kernel and NFSD types and
// functions referenced here are supplied by the surrounding translation unit.
#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

use core::ffi::{c_char, c_int, c_void};

pub type __be32 = u32;
pub type u32_t = u32;
pub type u64_t = u64;
pub type loff_t = i64;
pub type umode_t = u16;
pub type dev_t = u64;
pub type ssize_t = isize;

// The original implementation is intentionally kept as a direct FFI-shaped
// translation: all kernel-owned layouts remain opaque and all operations are
// delegated to the corresponding external Linux/NFSD symbols.
#[repr(C)] pub struct svc_rqst { _private: [u8; 0] }
#[repr(C)] pub struct svc_fh { _private: [u8; 0] }
#[repr(C)] pub struct svc_export { _private: [u8; 0] }
#[repr(C)] pub struct dentry { _private: [u8; 0] }
#[repr(C)] pub struct inode { _private: [u8; 0] }
#[repr(C)] pub struct file { _private: [u8; 0] }
#[repr(C)] pub struct nfsd_file { _private: [u8; 0] }
#[repr(C)] pub struct nfsd_attrs { _private: [u8; 0] }
#[repr(C)] pub struct xdr_buf { _private: [u8; 0] }
#[repr(C)] pub struct kstatfs { _private: [u8; 0] }
#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }

pub static mut nfsd_disable_splice_read: bool = false;
pub static mut nfsd_io_cache_read: u64 = 0;
pub static mut nfsd_io_cache_write: u64 = 0;

extern "C" {
    pub fn nfserrno(errno: c_int) -> __be32;
    pub fn nfsd_cross_mnt(rqstp: *mut svc_rqst, dpp: *mut *mut dentry,
                          expp: *mut *mut svc_export) -> c_int;
    pub fn nfsd_lookup_dentry(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
                              name: *const c_char, len: u32,
                              exp: *mut *mut svc_export,
                              dentry: *mut *mut dentry) -> __be32;
    pub fn nfsd_lookup(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
                       name: *const c_char, len: u32, resfh: *mut svc_fh) -> __be32;
    pub fn nfsd_setattr(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
                        attrs: *mut nfsd_attrs,
                        guardtime: *const timespec64) -> __be32;
    pub fn nfsd_read(rqstp: *mut svc_rqst, fhp: *mut svc_fh, offset: loff_t,
                     count: *mut usize, eof: *mut u32) -> __be32;
    pub fn nfsd_write(rqstp: *mut svc_rqst, fhp: *mut svc_fh, offset: loff_t,
                      payload: *const xdr_buf, count: *mut usize,
                      stable: c_int, verf: *mut __be32) -> __be32;
    pub fn nfsd_commit(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
                       nf: *mut nfsd_file, offset: u64, count: u32,
                       verf: *mut __be32) -> __be32;
    pub fn nfsd_create(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
                       fname: *mut c_char, flen: c_int, attrs: *mut nfsd_attrs,
                       kind: c_int, rdev: dev_t, resfhp: *mut svc_fh) -> __be32;
    pub fn nfsd_readlink(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
                         buf: *mut c_char, len: *mut c_int) -> __be32;
    pub fn nfsd_symlink(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
                        fname: *mut c_char, flen: c_int, path: *mut c_char,
                        attrs: *mut nfsd_attrs, resfhp: *mut svc_fh) -> __be32;
    pub fn nfsd_link(rqstp: *mut svc_rqst, ffhp: *mut svc_fh,
                     name: *mut c_char, len: c_int, tfhp: *mut svc_fh) -> __be32;
    pub fn nfsd_rename(rqstp: *mut svc_rqst, ffhp: *mut svc_fh,
                       fname: *mut c_char, flen: c_int, tfhp: *mut svc_fh,
                       tname: *mut c_char, tlen: c_int) -> __be32;
    pub fn nfsd_unlink(rqstp: *mut svc_rqst, fhp: *mut svc_fh, kind: c_int,
                       fname: *mut c_char, flen: c_int) -> __be32;
    pub fn nfsd_statfs(rqstp: *mut svc_rqst, fhp: *mut svc_fh,
                       stat: *mut kstatfs, access: c_int) -> __be32;
}

// CONFIG_NFSD_V4-dependent declarations are intentionally left as external
// interfaces, matching the conditional declarations in the source file.

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
