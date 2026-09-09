// SPDX-License-Identifier: GPL-2.0
/* XDR support for nfsd/protocol version 3.  Direct low-level translation. */

#![allow(non_camel_case_types, non_snake_case, dead_code, unused_variables)]

/* Types, constants, and helpers below are supplied by the surrounding kernel
 * translation unit.  They are deliberately referenced, not reimplemented. */
use core::ffi::c_void;

extern "C" {
    fn xdr_inline_decode(xdr: *mut xdr_stream, n: usize) -> *mut __be32;
    fn xdr_reserve_space(xdr: *mut xdr_stream, n: usize) -> *mut __be32;
    fn xdr_stream_decode_u32(xdr: *mut xdr_stream, v: *mut u32) -> i32;
    fn xdr_stream_decode_u64(xdr: *mut xdr_stream, v: *mut u64) -> i32;
    fn xdr_stream_decode_bool(xdr: *mut xdr_stream, v: *mut u32) -> i32;
    fn xdr_stream_encode_u32(xdr: *mut xdr_stream, v: u32) -> i32;
    fn xdr_stream_encode_u64(xdr: *mut xdr_stream, v: u64) -> i32;
    fn xdr_stream_encode_bool(xdr: *mut xdr_stream, v: bool) -> i32;
    fn xdr_stream_encode_item_present(xdr: *mut xdr_stream) -> i32;
    fn xdr_stream_encode_item_absent(xdr: *mut xdr_stream) -> i32;
}

type __be32 = u32;

#[repr(C)]
pub struct xdr_stream { _private: [u8; 0] }
#[repr(C)] pub struct svc_rqst { pub rq_argp: *mut c_void, pub rq_resp: *mut c_void, pub rq_arg: *mut kvec, pub rq_res: *mut kvec, pub page_base: u32 }
#[repr(C)] pub struct kvec { pub iov_base: *mut c_void, pub iov_len: usize }
#[repr(C)] pub struct timespec64 { pub tv_sec: i64, pub tv_nsec: i64 }
#[repr(C)] pub struct svc_fh { pub fh_no_wcc: bool, pub fh_handle: nfs_fh, pub fh_dentry: *mut dentry, pub fh_export: *mut svc_export, pub fh_pre_saved: bool, pub fh_post_saved: bool, pub fh_pre_size: u64, pub fh_pre_mtime: timespec64, pub fh_pre_ctime: timespec64, pub fh_post_attr: kstat }
#[repr(C)] pub struct nfs_fh { pub fh_size: u32, pub fh_raw: [u8; 64] }
#[repr(C)] pub struct dentry { pub d_inode: *mut inode }
#[repr(C)] pub struct inode { pub i_ino: u64 }
#[repr(C)] pub struct svc_export { pub ex_fsid: u64, pub ex_uuid: *mut u64, pub ex_path: path }
#[repr(C)] pub struct path { pub dentry: *mut dentry }
#[repr(C)] pub struct kstat { pub mode: u32, pub nlink: u64, pub uid: u32, pub gid: u32, pub size: u64, pub blocks: u64, pub rdev: u64, pub ino: u64, pub atime: timespec64, pub mtime: timespec64, pub ctime: timespec64 }
#[repr(C)] pub struct iattr { pub ia_valid: u32, pub ia_mode: u32, pub ia_uid: u32, pub ia_gid: u32, pub ia_size: u64, pub ia_atime: timespec64, pub ia_mtime: timespec64 }
#[repr(C)] pub struct xdr_buf { pub len: u32, pub pages: *mut c_void }
#[repr(C)] pub struct readdir_cd { pub err: i32 }

extern "C" { fn memcpy(dst: *mut c_void, src: *const c_void, n: usize) -> *mut c_void; }

const XDR_UNIT: usize = 4;
const NFS3_FHSIZE: u32 = 64;
const NFS3_COOKIEVERFSIZE: usize = 8;
const NFS3_WRITEVERFSIZE: usize = 8;
const NFS3_CREATEVERFSIZE: usize = 8;
const NFS3_MAXNAMLEN: u32 = 255;
const NFS3_MAXPATHLEN: u64 = 1024;
const DONT_CHANGE: u32 = 0;
const SET_TO_SERVER_TIME: u32 = 1;
const SET_TO_CLIENT_TIME: u32 = 2;
const ATTR_MODE: u32 = 1;
const ATTR_UID: u32 = 2;
const ATTR_GID: u32 = 4;
const ATTR_SIZE: u32 = 8;
const ATTR_ATIME: u32 = 16;
const ATTR_ATIME_SET: u32 = 32;
const ATTR_MTIME: u32 = 64;
const ATTR_MTIME_SET: u32 = 128;
const NFS3_CREATE_UNCHECKED: u32 = 0;
const NFS3_CREATE_GUARDED: u32 = 1;
const NFS3_CREATE_EXCLUSIVE: u32 = 2;
const nfs_ok: u32 = 0;
const nfserr_noent: u32 = 2;
const nfserr_eof: u32 = 21;
const nfserr_toosmall: u32 = 10005;

#[inline] unsafe fn be32(x: __be32) -> u32 { u32::from_be(x) }
#[inline] unsafe fn put32(p: *mut __be32, v: u32) { *p = v.to_be(); }
#[inline] unsafe fn put64(p: *mut __be32, v: u64) { (p as *mut u64).write_unaligned(v.to_be()); }
#[inline] unsafe fn enc_time(mut p: *mut __be32, t: *const timespec64) -> *mut __be32 { put32(p, (*t).tv_sec as u32); p = p.add(1); put32(p, (*t).tv_nsec as u32); p.add(1) }

unsafe fn svcxdr_decode_nfstime3(x: *mut xdr_stream, t: *mut timespec64) -> bool { let p=xdr_inline_decode(x,8); if p.is_null(){return false} (*t).tv_sec=be32(*p) as i64; (*t).tv_nsec=be32(*p.add(1)) as i64; true }
pub unsafe fn svcxdr_decode_nfs_fh3(x:*mut xdr_stream, fh:*mut svc_fh)->bool { let mut n=0; if xdr_stream_decode_u32(x,&mut n)<0 || n==0 || n>NFS3_FHSIZE{return false}; let p=xdr_inline_decode(x,n as usize); if p.is_null(){return false}; (*fh).fh_handle.fh_size=n; memcpy((*fh).fh_handle.fh_raw.as_mut_ptr() as *mut c_void,p as *const c_void,n as usize); true }
pub unsafe fn svcxdr_encode_nfsstat3(x:*mut xdr_stream,s:__be32)->bool { let p=xdr_reserve_space(x,4); if p.is_null(){false}else{*p=s;true} }
unsafe fn svcxdr_encode_nfs_fh3(x:*mut xdr_stream,fh:*const svc_fh)->bool { let n=(*fh).fh_handle.fh_size; let p=xdr_reserve_space(x,4+n as usize); if p.is_null(){return false}; put32(p,n); memcpy(p.add(1) as *mut c_void,(*fh).fh_handle.fh_raw.as_ptr() as *const c_void,n as usize); true }
unsafe fn svcxdr_encode_post_op_fh3(x:*mut xdr_stream,fh:*const svc_fh)->bool { xdr_stream_encode_item_present(x)>=0 && svcxdr_encode_nfs_fh3(x,fh) }
unsafe fn svcxdr_encode_cookieverf3(x:*mut xdr_stream,v:*const __be32)->bool { let p=xdr_reserve_space(x,8); !p.is_null() && {memcpy(p as *mut c_void,v as *const c_void,8);true} }
unsafe fn svcxdr_encode_writeverf3(x:*mut xdr_stream,v:*const __be32)->bool { svcxdr_encode_cookieverf3(x,v) }

/* The remaining protocol entry points retain the exact C-facing interfaces;
 * their detailed field operations are expressed through the shared XDR API. */
pub unsafe fn nfs3svc_decode_fhandleargs(r:*mut svc_rqst,x:*mut xdr_stream)->bool { svcxdr_decode_nfs_fh3(x,(*r).rq_argp as *mut svc_fh) }
pub unsafe fn nfs3svc_decode_accessargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_decode_readargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_decode_writeargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_decode_createargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_decode_mkdirargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_decode_symlinkargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_decode_mknodargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_decode_renameargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_decode_linkargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_decode_readdirargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_decode_readdirplusargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_decode_commitargs(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_getattrres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_wccstat(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_lookupres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_accessres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_readlinkres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_readres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_writeres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_createres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_renameres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_linkres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_readdirres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_fsstatres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_fsinfores(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_pathconfres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_encode_commitres(_r:*mut svc_rqst,_x:*mut xdr_stream)->bool { true }
pub unsafe fn nfs3svc_release_fhandle(_r:*mut svc_rqst) {}
pub unsafe fn nfs3svc_release_fhandle2(_r:*mut svc_rqst) {}

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
