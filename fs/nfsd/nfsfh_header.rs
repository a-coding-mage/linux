/* SPDX-License-Identifier: GPL-2.0 */
/*
 * Rust translation of nfsfh.h. C includes and externally supplied kernel
 * symbols are intentionally left as external dependencies.
 */

#[repr(C)]
pub struct knfsd_fh {
    pub fh_size: core::ffi::c_uint,
    pub fh_raw: [u8; NFS4_FHSIZE],
}

pub const NFS4_FHSIZE: usize = 128; // supplied by linux/nfs4.h

#[inline]
pub unsafe fn fh_version(fh: *const knfsd_fh) -> *mut u8 { (*fh as *const _ as *mut knfsd_fh).as_mut().unwrap().fh_raw.as_mut_ptr() }
#[inline]
pub unsafe fn fh_auth_type(fh: *const knfsd_fh) -> *mut u8 { (*fh as *const _ as *mut knfsd_fh).as_mut().unwrap().fh_raw.as_mut_ptr().add(1) }
#[inline]
pub unsafe fn fh_fsid_type(fh: *const knfsd_fh) -> *mut u8 { (*fh as *const _ as *mut knfsd_fh).as_mut().unwrap().fh_raw.as_mut_ptr().add(2) }
#[inline]
pub unsafe fn fh_fileid_type(fh: *const knfsd_fh) -> *mut u8 { (*fh as *const _ as *mut knfsd_fh).as_mut().unwrap().fh_raw.as_mut_ptr().add(3) }

#[inline]
pub unsafe fn fh_fsid(fh: *const knfsd_fh) -> *mut u32 {
    (*fh).fh_raw.as_ptr().add(4) as *mut u32
}

#[inline]
pub fn ino_t_to_u32(ino: ino_t) -> u32 { ino as u32 }
#[inline]
pub fn u32_to_ino_t(uino: u32) -> ino_t { uino as ino_t }

#[repr(C)]
pub struct svc_fh {
    pub fh_handle: knfsd_fh,
    pub fh_maxsize: core::ffi::c_int,
    pub fh_dentry: *mut dentry,
    pub fh_export: *mut svc_export,
    pub fh_want_write: bool,
    pub fh_no_wcc: bool,
    pub fh_no_atomic_attr: bool,
    pub fh_use_wgather: bool,
    pub fh_64bit_cookies: bool,
    pub fh_flags: core::ffi::c_int,
    pub fh_post_saved: bool,
    pub fh_pre_saved: bool,
    pub fh_pre_size: u64,
    pub fh_pre_mtime: timespec64,
    pub fh_pre_ctime: timespec64,
    pub fh_pre_change: u64,
    pub fh_post_attr: kstat,
    pub fh_post_change: u64,
}

pub const NFSD4_FH_FOREIGN: core::ffi::c_int = 1 << 0;
#[inline] pub unsafe fn SET_FH_FLAG(c: *mut svc_fh, f: core::ffi::c_int) { (*c).fh_flags |= f; }
#[inline] pub unsafe fn HAS_FH_FLAG(c: *const svc_fh, f: core::ffi::c_int) -> core::ffi::c_int { (*c).fh_flags & f }

#[repr(C)]
#[derive(Copy, Clone)]
pub enum nfsd_fsid { FSID_DEV = 0, FSID_NUM, FSID_MAJOR_MINOR, FSID_ENCODE_DEV, FSID_UUID4_INUM, FSID_UUID8, FSID_UUID16, FSID_UUID16_INUM }
#[repr(C)]
#[derive(Copy, Clone)]
pub enum fsid_source { FSIDSOURCE_DEV, FSIDSOURCE_FSID, FSIDSOURCE_UUID }

extern "C" {
    fn htonl(value: u32) -> u32;
    fn new_encode_dev(dev: dev_t) -> u32;
    fn major(dev: dev_t) -> u32;
    fn minor(dev: dev_t) -> u32;
    fn memcpy(dst: *mut core::ffi::c_void, src: *const core::ffi::c_void, n: usize) -> *mut core::ffi::c_void;
    fn BUG() -> !;
}

#[inline]
pub unsafe fn mk_fsid(vers: core::ffi::c_int, fsidv: *mut u32, dev: dev_t, ino: ino_t, fsid: u32, uuid: *mut u8) {
    let up: *mut u32;
    match vers {
        x if x == nfsd_fsid::FSID_DEV as core::ffi::c_int => {
            *fsidv = htonl((major(dev) << 16) | minor(dev));
            *fsidv.add(1) = ino_t_to_u32(ino);
        }
        x if x == nfsd_fsid::FSID_NUM as core::ffi::c_int => *fsidv = fsid,
        x if x == nfsd_fsid::FSID_MAJOR_MINOR as core::ffi::c_int => {
            *fsidv = htonl(major(dev));
            *fsidv.add(1) = htonl(minor(dev));
            *fsidv.add(2) = ino_t_to_u32(ino);
        }
        x if x == nfsd_fsid::FSID_ENCODE_DEV as core::ffi::c_int => {
            *fsidv = new_encode_dev(dev);
            *fsidv.add(1) = ino_t_to_u32(ino);
        }
        x if x == nfsd_fsid::FSID_UUID4_INUM as core::ffi::c_int => {
            up = uuid as *mut u32;
            *fsidv = ino_t_to_u32(ino);
            *fsidv.add(1) = *up ^ *up.add(1) ^ *up.add(2) ^ *up.add(3);
        }
        x if x == nfsd_fsid::FSID_UUID8 as core::ffi::c_int => {
            up = uuid as *mut u32;
            *fsidv = *up ^ *up.add(2);
            *fsidv.add(1) = *up.add(1) ^ *up.add(3);
        }
        x if x == nfsd_fsid::FSID_UUID16 as core::ffi::c_int => { memcpy(fsidv as *mut _, uuid as *const _, 16); }
        x if x == nfsd_fsid::FSID_UUID16_INUM as core::ffi::c_int => {
            *(fsidv as *mut u64) = ino as u64;
            memcpy(fsidv.add(2) as *mut _, uuid as *const _, 16);
        }
        _ => BUG(),
    }
}

#[inline]
pub const fn key_len(typ: core::ffi::c_int) -> core::ffi::c_int {
    match typ {
        x if x == nfsd_fsid::FSID_DEV as core::ffi::c_int => 8,
        x if x == nfsd_fsid::FSID_NUM as core::ffi::c_int => 4,
        x if x == nfsd_fsid::FSID_MAJOR_MINOR as core::ffi::c_int => 12,
        x if x == nfsd_fsid::FSID_ENCODE_DEV as core::ffi::c_int => 8,
        x if x == nfsd_fsid::FSID_UUID4_INUM as core::ffi::c_int => 8,
        x if x == nfsd_fsid::FSID_UUID8 as core::ffi::c_int => 8,
        x if x == nfsd_fsid::FSID_UUID16 as core::ffi::c_int => 16,
        x if x == nfsd_fsid::FSID_UUID16_INUM as core::ffi::c_int => 24,
        _ => 0,
    }
}

extern "C" {
    pub fn fsid_source_fh(fh: *const knfsd_fh, exp: *mut svc_export) -> fsid_source;
    pub fn fsid_source(fhp: *const svc_fh) -> fsid_source;
    pub fn SVCFH_fmt(fhp: *mut svc_fh) -> *mut core::ffi::c_char;
    pub fn fh_verify(rqst: *mut svc_rqst, fh: *mut svc_fh, mode: umode_t, access: core::ffi::c_int) -> __be32;
    pub fn fh_verify_local(net: *mut net, cred: *mut svc_cred, domain: *mut auth_domain, fh: *mut svc_fh, mode: umode_t, access: core::ffi::c_int) -> __be32;
    pub fn fh_getattr(fhp: *const svc_fh, stat: *mut kstat) -> __be32;
    pub fn fh_compose(dst: *mut svc_fh, exp: *mut svc_export, dentry: *mut dentry, parent: *mut svc_fh) -> __be32;
    pub fn fh_update(fh: *mut svc_fh) -> __be32;
    pub fn fh_put(fh: *mut svc_fh);
    pub fn fh_append_mac(fh: *mut knfsd_fh, maxsize: core::ffi::c_int, net: *mut net) -> bool;
}

#[inline]
pub unsafe fn fh_copy(dst: *mut svc_fh, src: *const svc_fh) -> *mut svc_fh {
    // WARN_ON(src->fh_dentry);
    *dst = *src;
    dst
}

pub type ino_t = u64;
pub type dev_t = u64;
pub type umode_t = u32;
pub type __be32 = u32;
pub type timespec64 = core::ffi::c_longlong;
pub type dentry = core::ffi::c_void;
pub type svc_export = core::ffi::c_void;
pub type kstat = core::ffi::c_void;
pub type svc_rqst = core::ffi::c_void;
pub type net = core::ffi::c_void;
pub type svc_cred = core::ffi::c_void;
pub type auth_domain = core::ffi::c_void;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
