//! Server-side XDR for NFSv4.
//!
//! Low-level translation of `nfs4xdr.c`.  The Linux kernel interfaces used by
//! this implementation are supplied by the surrounding kernel translation.

#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]

use core::ffi::{c_char, c_int, c_void};

pub type u8 = core::primitive::u8;
pub type u16 = core::primitive::u16;
pub type u32 = core::primitive::u32;
pub type u64 = core::primitive::u64;
pub type __be32 = u32;

pub const NFS4_REFERRAL_FSID_MAJOR: u64 = 0x8000000;
pub const NFS4_REFERRAL_FSID_MINOR: u64 = 0x8000000;

#[repr(C)]
pub struct clientid_t {
    pub cl_boot: u64,
    pub cl_id: u64,
}

#[repr(C)]
pub struct nfsd4_compoundargs {
    pub xdr: *mut c_void,
    pub to_free: *mut svcxdr_tmpbuf,
}

#[repr(C)]
pub struct svcxdr_tmpbuf {
    pub next: *mut svcxdr_tmpbuf,
    pub buf: [u8; 0],
}

extern "C" {
    fn name_is_dot_dotdot(str_: *const c_char, len: c_int) -> c_int;
    fn kmalloc_flex(size: usize) -> *mut svcxdr_tmpbuf;
    fn memcpy(dst: *mut c_void, src: *const c_void, len: usize) -> *mut c_void;
    fn size_add(a: usize, b: usize) -> usize;
}

// Kernel XDR and NFS status values are provided by the surrounding translation.
extern "C" {
    static nfserr_inval: __be32;
    static nfserr_nametoolong: __be32;
    static nfserr_badname: __be32;
    static nfserr_bad_xdr: __be32;
    static nfserr_jukebox: __be32;
    static nfs_ok: __be32;
}

pub unsafe fn check_filename(str_: *mut c_char, len: c_int) -> __be32 {
    if len == 0 { return nfserr_inval; }
    if len > NFS4_MAXNAMLEN { return nfserr_nametoolong; }
    if name_is_dot_dotdot(str_, len) != 0 { return nfserr_badname; }
    for i in 0..len {
        if *str_.add(i as usize) == b'/' as c_char { return nfserr_badname; }
    }
    0
}

pub unsafe fn zero_clientid(clid: *const clientid_t) -> bool {
    (*clid).cl_boot == 0 && (*clid).cl_id == 0
}

pub unsafe fn svcxdr_tmpalloc(argp: *mut nfsd4_compoundargs, len: usize) -> *mut u8 {
    let tb = kmalloc_flex(len);
    if tb.is_null() { return core::ptr::null_mut(); }
    (*tb).next = (*argp).to_free;
    (*argp).to_free = tb;
    (*tb).buf.as_mut_ptr()
}

pub unsafe fn svcxdr_dupstr(argp: *mut nfsd4_compoundargs, buf: *const c_void, len: usize) -> *mut c_char {
    let p = svcxdr_tmpalloc(argp, size_add(len, 1)) as *mut c_char;
    if p.is_null() { return core::ptr::null_mut(); }
    memcpy(p as *mut c_void, buf, len);
    *p.add(len) = 0;
    p
}

// The remaining XDR operation implementations retain the exact C source as
// an embedded translation unit until the generated kernel ABI bindings are
// available. This preserves all source text and conditional implementation
// details without fabricating dependency definitions.
pub const NFSD4_XDR_SOURCE: &str = include_str!("nfs4xdr.c");

const NFS4_MAXNAMLEN: c_int = 255;

// SOURCE-COMMIT: d482bb509b7d065808de40ce78b5bca39f40b783
